//! The 旧暦 date the Moon-keyed annotations read.
//!
//! 六曜, 不成就日, the 旧暦 reading of 凶会日, 二十七宿 and the two
//! moon-viewing nights are keyed to the lunisolar month and day. They read
//! it here, from one of two calendars of `hc-calendars-lunar`, a
//! [`Reckoning`] the caller's meridian names:
//!
//! * At [`Meridian::CHINA`] and [`Meridian::CHINA_BEFORE_1929`],
//!   [`Reckoning::Chinese`]: the `chinese` registry calendar,
//!   [`hc_calendars_lunar::chinese::PARAMETERS`], the Shíxiàn rules at
//!   Beijing's meridian to 1928 and 120°E from 1929, with the published
//!   almanacs' corrections.
//! * At every other meridian, [`Reckoning::JapaneseTenpo`]: the Japanese
//!   Tenpō calendar with its 1872 bound removed,
//!   [`hc_calendars_lunar::japanese_tenpo::UNBOUNDED_PARAMETERS`], at
//!   Kyoto's meridian to 1872, Tokyo time for 1873–1887 and Japan Standard
//!   Time from 1888. That is what Japanese almanacs have printed as 旧暦
//!   since the calendar was abolished, and it agrees with the National
//!   Astronomical Observatory's tables of 2014 and 1984–85
//!   (`nao-topics-2014-2033`). Japan has made no official lunisolar
//!   calculation since then (`nao-faq-kyureki`), so the date is the rules'
//!   and no authority's. In 2033–34, where no numbering satisfies the
//!   天保暦 rule, it gives the first of the Observatory's three resolutions,
//!   閏11月 (`nao-rekiwiki-2033`). [`Meridian::KOREA`] is the same offset
//!   as [`Meridian::JAPAN`] and reads it too; no lunisolar calendar is kept
//!   at any other meridian, and the Japanese one is the one the
//!   annotations come from.
//!
//! Both calendars carry their own meridians, so a date is theirs, not a
//! reading of the Moon at the offset passed; the 節月 and the other solar
//! annotations are read at that offset.
//!
//! The calendars are read, and not a month-by-month rule: numbering each
//! month by the later of its 中気 at the meridian passed gives other months
//! on 89 of the 3,653 days of 2024–2033 at the Japanese meridian, all from
//! 25 August to 21 November 2033, and costs little less: 3,653 consecutive
//! days take 21 to 26 ms here inside one [`hc_core::memo::scope`], against
//! about 10 ms for that rule, in a release build on an M-series Mac.
//! `docs/systems/zassetsu-and-rokuyo.md` lists the days on which the two
//! differ.
//!
//! # Cost
//!
//! A reading searches for the winter solstices either side of the day and
//! the new moons between them. Each function here opens a
//! [`hc_core::memo::scope`], so a caller that opens one around a run of
//! days has each search done once; outside a scope, or in a build without
//! `hc-core`'s `memo` feature, a day costs about 45 µs.

use hc_calendar::{Month, Rd};
use hc_calendars_lunar::{LunisolarDate, LunisolarParameters, chinese, japanese_tenpo};
use hc_seasons::Meridian;

/// A lunisolar calendar the Moon-keyed annotations read; see the module
/// documentation for which meridian names which.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Reckoning {
    /// The Japanese Tenpō rules continued past 1872.
    JapaneseTenpo,
    /// The `chinese` registry calendar, its rules continued past 1645 and
    /// 2150, the years it converts.
    Chinese,
}

impl Reckoning {
    /// The reckoning a meridian names: [`Reckoning::Chinese`] at
    /// [`Meridian::CHINA`] and [`Meridian::CHINA_BEFORE_1929`],
    /// [`Reckoning::JapaneseTenpo`] at every other.
    #[must_use]
    pub const fn at(meridian: Meridian) -> Self {
        let offset = meridian.offset_seconds();
        if offset == Meridian::CHINA.offset_seconds()
            || offset == Meridian::CHINA_BEFORE_1929.offset_seconds()
        {
            Self::Chinese
        } else {
            Self::JapaneseTenpo
        }
    }

    /// The calendar's parameters.
    #[must_use]
    pub const fn parameters(self) -> &'static LunisolarParameters {
        match self {
            Self::JapaneseTenpo => &japanese_tenpo::UNBOUNDED_PARAMETERS,
            Self::Chinese => &chinese::PARAMETERS,
        }
    }

    /// The lunisolar date of a day.
    ///
    /// ```
    /// use hc_almanac::{Rd, lunisolar::Reckoning};
    ///
    /// // 10 February 2024 was the lunar new year: the first of the first
    /// // month in both calendars.
    /// for reckoning in [Reckoning::JapaneseTenpo, Reckoning::Chinese] {
    ///     let date = reckoning.date(Rd(738_926));
    ///     assert_eq!((date.month.ordinal, date.day), (1, 1));
    ///     assert!(!date.month.leap);
    /// }
    /// ```
    #[must_use]
    pub fn date(self, day: Rd) -> LunisolarDate {
        let (year, month, day) =
            hc_core::memo::scope(|| self.parameters().from_fixed_unbounded(day));
        LunisolarDate { year, month, day }
    }

    /// The first day in Gregorian `year` that is day `day` of an ordinary
    /// month `month`, if the year holds one.
    ///
    /// The months are walked from the one the Gregorian year opens in, so a
    /// date late in a lunisolar year that falls in January is found under
    /// the Gregorian year it lands in.
    #[must_use]
    pub fn day_in_gregorian_year(self, year: i64, month: u8, day: u8) -> Option<Rd> {
        if !hc_calendar::gregorian::year_in_range(year + 1) {
            return None;
        }
        let first = hc_calendar::gregorian::new_year(year);
        let end = hc_calendar::gregorian::new_year(year + 1);
        hc_core::memo::scope(|| {
            let mut cursor = first;
            // Thirteen months can begin in a Gregorian year, and the year
            // opens inside a fourteenth.
            for _ in 0..14 {
                let date = self.date(cursor);
                let start = Rd(cursor.0 - i64::from(date.day) + 1);
                if date.month == Month::regular(month) {
                    let candidate = Rd(start.0 + i64::from(day) - 1);
                    let found = self.date(candidate);
                    if found.month == date.month
                        && found.day == day
                        && (first..end).contains(&candidate)
                    {
                        return Some(candidate);
                    }
                }
                // Thirty days on is always inside the next month.
                cursor = Rd(start.0 + 30);
                if cursor >= end {
                    break;
                }
            }
            None
        })
    }
}

/// The lunisolar date of a day, in the reckoning `meridian` names.
#[must_use]
pub fn lunisolar_date(day: Rd, meridian: Meridian) -> LunisolarDate {
    Reckoning::at(meridian).date(day)
}

#[cfg(test)]
mod tests {
    use super::*;
    use hc_calendar::gregorian;
    use hc_seasons::{Meridian, SolarTerm, solar_terms::term_day};

    /// Published lunar new years, the first of the first month, which every
    /// almanac and news report agrees about.
    #[test]
    fn the_lunar_new_year_is_the_first_of_the_first_month() {
        for (year, month, day) in [
            (2015, 2, 19),
            (2016, 2, 8),
            (2017, 1, 28),
            (2018, 2, 16),
            (2019, 2, 5),
            (2020, 1, 25),
            (2021, 2, 12),
            (2022, 2, 1),
            (2023, 1, 22),
            (2024, 2, 10),
            (2025, 1, 29),
            (2026, 2, 17),
        ] {
            let date =
                Reckoning::JapaneseTenpo.date(gregorian::to_fixed_saturating(year, month, day));
            assert_eq!(
                (date.year, date.month.ordinal, date.month.leap, date.day),
                (year, 1, false, 1),
                "{year}-{month:02}-{day:02}"
            );
            let eve = Reckoning::JapaneseTenpo
                .date(Rd(gregorian::to_fixed_saturating(year, month, day).0 - 1));
            assert_eq!(eve.month.ordinal, 12);
            assert!((29..=30).contains(&eve.day));
            assert_eq!(
                Reckoning::JapaneseTenpo.day_in_gregorian_year(year, 1, 1),
                Some(gregorian::to_fixed_saturating(year, month, day))
            );
        }
    }

    /// 2023 had a leap second month, from 22 March, with no 中気.
    #[test]
    fn the_leap_second_month_of_2023_is_found() {
        let leap = Reckoning::JapaneseTenpo.date(gregorian::to_fixed_saturating(2023, 3, 22));
        assert_eq!(
            (leap.month.ordinal, leap.month.leap, leap.day),
            (2, true, 1)
        );
        let ordinary = Reckoning::JapaneseTenpo.date(gregorian::to_fixed_saturating(2023, 3, 21));
        assert_eq!((ordinary.month.ordinal, ordinary.month.leap), (2, false));
    }

    /// The months of 2014 as the Observatory tabulates them, with the leap
    /// ninth month (`nao-rekiwiki-chijun`, `nao-topics-2014-2033`).
    #[test]
    fn the_months_of_2014_are_the_ones_the_observatory_tabulates() {
        for ((year, month, day), ordinal, leap) in [
            ((2014, 1, 1), 12, false),
            ((2014, 1, 31), 1, false),
            ((2014, 3, 1), 2, false),
            ((2014, 3, 31), 3, false),
            ((2014, 4, 29), 4, false),
            ((2014, 5, 29), 5, false),
            ((2014, 6, 27), 6, false),
            ((2014, 7, 27), 7, false),
            ((2014, 8, 25), 8, false),
            ((2014, 9, 24), 9, false),
            ((2014, 10, 24), 9, true),
            ((2014, 11, 22), 10, false),
            ((2014, 12, 22), 11, false),
            ((2015, 1, 20), 12, false),
        ] {
            let date =
                Reckoning::JapaneseTenpo.date(gregorian::to_fixed_saturating(year, month, day));
            assert_eq!(
                (date.month.ordinal, date.month.leap, date.day),
                (ordinal, leap, 1),
                "{year}-{month:02}-{day:02}"
            );
        }
    }

    /// The month holding the winter solstice is the eleventh and the one
    /// holding the spring equinox the second; the numbering hangs off them.
    #[test]
    fn the_solstice_and_the_equinox_fix_their_months() {
        hc_core::memo::scope(|| {
            for year in 1990..2040 {
                let solstice = Reckoning::JapaneseTenpo.date(term_day(
                    year,
                    SolarTerm::WINTER_SOLSTICE,
                    Meridian::JAPAN,
                ));
                assert_eq!((solstice.month.ordinal, solstice.month.leap), (11, false));
                let equinox = Reckoning::JapaneseTenpo.date(term_day(
                    year,
                    SolarTerm::SPRING_EQUINOX,
                    Meridian::JAPAN,
                ));
                assert_eq!(equinox.month.ordinal, 2, "{year}");
            }
        });
    }

    /// At the Chinese meridian the date is the Chinese calendar's. The
    /// month starts here are the first days of the months in which it
    /// differs from the simplified derivation `hc-seasons` kept, as the
    /// Hong Kong Observatory's conversion tables give them
    /// (`hko-conversion-tables`, `hko-conversion-tables-moon-keyed`): the
    /// 10th month of 1914 from 17 November, the 1st of 1916 from
    /// 3 February and the 10th of 1920 from 10 November, whose new moons
    /// fell before midnight at Beijing's meridian, which the calendar keeps
    /// to 1928, and after it at 120°E, where the derivation read them; the 12th of
    /// 1984–85 from 21 January; the 8th to the 11th of 2033 and the leap
    /// 11th; and the 12th and the 1st of 2034 and of 2053.
    #[test]
    fn the_chinese_meridian_reads_the_months_the_observatory_tabulates() {
        let reckoning = Reckoning::at(Meridian::CHINA);
        assert_eq!(reckoning, Reckoning::Chinese);
        assert_eq!(
            Reckoning::at(Meridian::CHINA_BEFORE_1929),
            Reckoning::Chinese
        );
        for meridian in [Meridian::JAPAN, Meridian::KOREA, Meridian::UNIVERSAL] {
            assert_eq!(Reckoning::at(meridian), Reckoning::JapaneseTenpo);
        }
        for ((year, month, day), ordinal, leap) in [
            ((1914, 11, 17), 10, false),
            ((1916, 2, 3), 1, false),
            ((1920, 11, 10), 10, false),
            ((1985, 1, 21), 12, false),
            ((2033, 8, 25), 8, false),
            ((2033, 9, 23), 9, false),
            ((2033, 10, 23), 10, false),
            ((2033, 11, 22), 11, false),
            ((2033, 12, 22), 11, true),
            ((2034, 1, 20), 12, false),
            ((2034, 2, 19), 1, false),
            ((2053, 1, 20), 12, false),
            ((2053, 2, 19), 1, false),
        ] {
            let date = reckoning.date(gregorian::to_fixed_saturating(year, month, day));
            assert_eq!(
                (date.month.ordinal, date.month.leap, date.day),
                (ordinal, leap, 1),
                "{year}-{month:02}-{day:02}"
            );
        }
        // The Japanese calendar, a meridian further east, begins the first
        // three a day later.
        let japanese = Reckoning::JapaneseTenpo.date(gregorian::to_fixed_saturating(1914, 11, 17));
        assert_eq!((japanese.month.ordinal, japanese.day), (9, 30));
    }

    #[test]
    fn a_day_the_month_lacks_is_none() {
        // The eighth month of 2024 ran from 3 September to 2 October, 30
        // days, and the ninth from 3 October to 31 October, 29: the new
        // moons fell on 3 September, 3 October and 1 November JST
        // (`nao-rekiyoko-2024-sakugenbo`).
        assert_eq!(
            Reckoning::JapaneseTenpo.day_in_gregorian_year(2024, 8, 30),
            Some(gregorian::to_fixed_saturating(2024, 10, 2))
        );
        assert_eq!(
            Reckoning::JapaneseTenpo.day_in_gregorian_year(2024, 9, 30),
            None
        );
        assert_eq!(
            Reckoning::JapaneseTenpo.day_in_gregorian_year(2024, 13, 1),
            None
        );
    }
}
