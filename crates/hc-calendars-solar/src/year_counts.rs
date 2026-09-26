//! Year counts laid over the Julian or the Gregorian year: the Spanish era,
//! the four Masonic years and the year After the Development of Agriculture.
//!
//! Each is a calendar whose days, months and leap rule are another
//! calendar's and whose year number is that calendar's plus a constant, so
//! the six are one table, [`ALL`], read by one [`YearCountCalendar`]; they
//! differ in data only, as policy §2 asks. Their system document is
//! `docs/systems/era-counts.md` in the repository, which also covers the
//! Era of Philip ([`crate::philip_era`]), the Bostran era
//! ([`crate::bostran`]) and the Era Fascista ([`crate::era_fascista`]),
//! whose year boundaries or days are their own.
//!
//! The Holocene, Minguo and Juche years are the same shape over the
//! Gregorian year and share the arithmetic, `common::offset_to_fixed`; they
//! keep modules of their own because each has a date type and a history of
//! its own. A year count added later with nothing of its own belongs here.
//!
//! The year number of each is counted from 1: a year count's year 0 and
//! the years before it are refused rather than written as a negative
//! number nobody wrote.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, Usage,
    YearKind,
};

use crate::{gregorian, julian};

/// The calendar a year count's days, months and leap rule are taken from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Base {
    /// The proleptic Gregorian calendar, [`crate::gregorian`].
    Gregorian,
    /// The proleptic Julian calendar, [`crate::julian`], in astronomical
    /// year numbering.
    Julian,
}

impl Base {
    /// Whether the base calendar's `year`, in astronomical numbering, is
    /// leap.
    #[must_use]
    pub const fn is_leap_year(self, year: i64) -> bool {
        match self {
            Self::Gregorian => gregorian::is_leap_year(year),
            Self::Julian => julian::is_leap_year(year),
        }
    }

    /// The fixed day of a date of the base calendar.
    ///
    /// # Errors
    ///
    /// As the base calendar's `to_fixed`.
    pub const fn to_fixed(self, year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
        match self {
            Self::Gregorian => gregorian::to_fixed(year, month, day),
            Self::Julian => julian::to_fixed(year, month, day),
        }
    }

    /// The base calendar's date of a fixed day.
    ///
    /// # Errors
    ///
    /// As the base calendar's `from_fixed`.
    pub const fn from_fixed(self, rd: Rd) -> CalendarResult<(i64, u8, u8)> {
        match self {
            Self::Gregorian => gregorian::from_fixed(rd),
            Self::Julian => julian::from_fixed(rd),
        }
    }

    /// The last fixed day the base calendar converts.
    #[must_use]
    pub const fn latest(self) -> Rd {
        match self {
            Self::Gregorian => gregorian::LATEST,
            Self::Julian => julian::LATEST,
        }
    }
}

/// A year count: an identifier, an era, a base calendar and an offset.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct YearCount {
    /// The calendar identifier.
    pub id: &'static str,
    /// The English name of the calendar.
    pub english_name: &'static str,
    /// The era code a date carries.
    pub era: &'static str,
    /// The calendar whose days are counted.
    pub base: Base,
    /// The year of the count less the base calendar's astronomical year.
    pub offset: i64,
    /// When the count was used, and on whose authority.
    pub usage: Usage,
    /// The languages the count's own sources write it in.
    pub native_locales: &'static [&'static str],
}

impl YearCount {
    /// The base calendar's astronomical year of year `year` of this count.
    #[must_use]
    pub const fn base_year(self, year: i64) -> i64 {
        year - self.offset
    }

    /// The first fixed day of year 1 of this count.
    #[must_use]
    pub const fn earliest(self) -> Rd {
        match self.base.to_fixed(self.base_year(1), 1, 1) {
            Ok(rd) => rd,
            Err(_) => self.base.latest(),
        }
    }

    /// Whether `year` of this count is leap: the base calendar's rule on
    /// the base calendar's year.
    #[must_use]
    pub const fn is_leap_year(self, year: i64) -> bool {
        self.base.is_leap_year(self.base_year(year))
    }

    /// The fixed day of a date of this count.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::YearOutOfRange`] for a year before 1, and
    /// the base calendar's errors otherwise.
    pub const fn to_fixed(self, year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
        if year < 1 {
            return Err(CalendarError::YearOutOfRange);
        }
        self.base.to_fixed(self.base_year(year), month, day)
    }

    /// The date of this count of a fixed day.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::BeforeEpoch`] before year 1, and the base
    /// calendar's errors otherwise.
    pub const fn from_fixed(self, rd: Rd) -> CalendarResult<(i64, u8, u8)> {
        if rd.0 < self.earliest().0 {
            return Err(CalendarError::BeforeEpoch);
        }
        match self.base.from_fixed(rd) {
            Ok((year, month, day)) => Ok((year + self.offset, month, day)),
            Err(error) => Err(error),
        }
    }
}

/// Where the Spanish era comes from.
pub const SPANISH_ERA_SOURCE: &str = "Wikipedia, \"Spanish era\", retrieved 2026-09-26 \
    [wikipedia-spanish-era]: the epoch of 1 January 38 BC, the Anno Domini year plus 38, \
    Era 941 as AD 903, the year officially from 1 January, and the years each kingdom \
    dropped it; Grumel, \"Eras, Historical\", New Catholic Encyclopedia \
    [grumel-eras-historical], for the era's use in Spain to the 14th century and in Portugal \
    until 1422, \"when it was officially abandoned\"";

/// Where the Masonic years come from.
pub const MASONIC_SOURCE: &str = "Lodge No. 43, F. & A. M., \"The Masonic Calendar\", 2 August \
    2014, retrieved 2026-09-26 [lodge43-masonic-calendar]: Anno Lucis of the Ancient Craft \
    Masons, the common year plus 4000; Anno Inventionis of the Royal Arch Masons, plus 530; \
    Anno Depositionis of the Royal and Select Masters, plus 1000; Anno Ordinis of the Knights \
    Templar, less 1118; with 2010 as 6010 A.L., 2540 A.I., 3010 A.Dep. and 892 A.O. \
    Wikipedia, \"Anno Lucis\", retrieved 2026-09-26 [wikipedia-anno-lucis], for Anno Lucis \
    adopted in the 18th century and AD 2026 as AL 6026. Neither dates a first use or names a \
    new year other than the common year's";

/// The Spanish era, *Era Hispanica*: the Julian year plus 38.
pub const SPANISH_ERA: YearCount = YearCount {
    id: "spanish-era",
    english_name: "Spanish era",
    era: "spanish-era",
    base: Base::Julian,
    offset: 38,
    // The sources date the era's use by the year only — see
    // [`SPANISH_ERA_ABANDONMENT`] — so no period of days is claimed.
    usage: Usage::UNRECORDED,
    native_locales: &["la", "es", "pt", "ca"],
};

/// *Anno Lucis*, the year of light of Craft Masonry: the Gregorian year
/// plus 4000.
pub const ANNO_LUCIS: YearCount = YearCount {
    id: "masonic-anno-lucis",
    english_name: "Masonic Anno Lucis",
    era: "al",
    base: Base::Gregorian,
    offset: 4_000,
    usage: Usage::undated(MASONIC_SOURCE),
    native_locales: &["la"],
};

/// *Anno Inventionis*, the year of the Royal Arch: the Gregorian year plus
/// 530.
pub const ANNO_INVENTIONIS: YearCount = YearCount {
    id: "masonic-anno-inventionis",
    english_name: "Masonic Anno Inventionis",
    era: "ai",
    base: Base::Gregorian,
    offset: 530,
    usage: Usage::undated(MASONIC_SOURCE),
    native_locales: &["la"],
};

/// *Anno Depositionis*, the year of the Royal and Select Masters: the
/// Gregorian year plus 1000.
pub const ANNO_DEPOSITIONIS: YearCount = YearCount {
    id: "masonic-anno-depositionis",
    english_name: "Masonic Anno Depositionis",
    era: "a-dep",
    base: Base::Gregorian,
    offset: 1_000,
    usage: Usage::undated(MASONIC_SOURCE),
    native_locales: &["la"],
};

/// *Anno Ordinis*, the year of the Knights Templar: the Gregorian year less
/// 1118.
pub const ANNO_ORDINIS: YearCount = YearCount {
    id: "masonic-anno-ordinis",
    english_name: "Masonic Anno Ordinis",
    era: "ao",
    base: Base::Gregorian,
    offset: -1_118,
    usage: Usage::undated(MASONIC_SOURCE),
    native_locales: &["la"],
};

/// After the Development of Agriculture: the Gregorian year plus 8000,
/// Merlin Stone's proposal of 1978 (Wikipedia, "After the Development of
/// Agriculture", retrieved 2026-09-26, `wikipedia-ada`: 1978 as 9978 ADA,
/// 2026 as 10026 ADA). Unrecorded, as a proposal.
pub const ADA: YearCount = YearCount {
    id: "ada",
    english_name: "After the Development of Agriculture",
    era: "ada",
    base: Base::Gregorian,
    offset: 8_000,
    usage: Usage::UNRECORDED,
    native_locales: &[],
};

/// Every year count in this module, in registry order.
pub const ALL: &[YearCount] = &[
    SPANISH_ERA,
    ANNO_LUCIS,
    ANNO_INVENTIONIS,
    ANNO_DEPOSITIONIS,
    ANNO_ORDINIS,
    ADA,
];

/// The years a kingdom stopped writing the Spanish era, as the source
/// gives them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Abandonment {
    /// The kingdom or region.
    pub kingdom: &'static str,
    /// The earliest Julian year, AD, the source gives for the change, or
    /// `None` where it gives none.
    pub earliest: Option<i64>,
    /// The latest Julian year, AD, the source gives for the change: the
    /// second of a pair such as 1349/1350, or the same year.
    pub latest: Option<i64>,
    /// The change as the source writes it.
    pub as_written: &'static str,
}

impl Abandonment {
    /// Whether the kingdom had dropped the era by Julian year `year`, AD:
    /// `Some(false)` before the earliest year the source gives,
    /// `Some(true)` after the latest, and `None` in between or where the
    /// source gives no year.
    #[must_use]
    pub const fn abandoned_by(self, year: i64) -> Option<bool> {
        match (self.earliest, self.latest) {
            (Some(earliest), _) if year < earliest => Some(false),
            (_, Some(latest)) if year > latest => Some(true),
            _ => None,
        }
    }
}

/// The years each kingdom dropped the Spanish era, as Wikipedia, "Spanish
/// era" (retrieved 2026-09-26), lists them; the years are the source's and
/// no day is claimed for any of them. Grumel has Portugal "until 1422,
/// when it was officially abandoned".
pub const SPANISH_ERA_ABANDONMENT: &[Abandonment] = &[
    Abandonment {
        kingdom: "Catalonia",
        earliest: Some(1_180),
        latest: Some(1_180),
        as_written: "1180",
    },
    Abandonment {
        kingdom: "Aragon",
        earliest: Some(1_349),
        latest: Some(1_350),
        as_written: "1349/1350",
    },
    Abandonment {
        kingdom: "Valencia",
        earliest: Some(1_358),
        latest: Some(1_358),
        as_written: "1358",
    },
    Abandonment {
        kingdom: "Castile",
        earliest: Some(1_382),
        latest: Some(1_383),
        as_written: "1382/1383",
    },
    Abandonment {
        kingdom: "Portugal",
        earliest: Some(1_420),
        latest: Some(1_422),
        as_written: "1420/1422",
    },
    Abandonment {
        kingdom: "Navarre",
        earliest: None,
        latest: None,
        as_written: "early 15th century",
    },
];

/// The abandonment record of `kingdom`, by its English name.
#[must_use]
pub fn spanish_era_abandonment(kingdom: &str) -> Option<Abandonment> {
    SPANISH_ERA_ABANDONMENT
        .iter()
        .copied()
        .find(|entry| entry.kingdom.eq_ignore_ascii_case(kingdom))
}

/// A date of a year count.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct YearCountDate {
    /// The year of the count, from 1.
    pub year: i64,
    /// The base calendar's month, 1 for January through 12 for December.
    pub month: u8,
    /// The day of the month, from 1.
    pub day: u8,
}

/// A year count as a calendar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct YearCountCalendar(pub YearCount);

impl Calendar for YearCountCalendar {
    type Date = YearCountDate;

    fn usage(&self) -> Usage {
        self.0.usage
    }

    /// The base calendar's twelve months and the seven-day week.
    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        hc_calendar::shape::SOLAR_TWELVE
    }

    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        if year < 1 {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(self.0.is_leap_year(year))
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId(self.0.id),
            english_name: self.0.english_name,
            year_kind: YearKind::EpochForward,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(self.0.earliest()),
            latest: Some(self.0.base.latest()),
            native_locales: self.0.native_locales,
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        self.0.to_fixed(date.year, date.month, date.day)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (year, month, day) = self.0.from_fixed(rd)?;
        Ok(YearCountDate { year, month, day })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        DateFields::ymd(date.year, date.month, date.day)
            .with_era(self.0.era)
            .with_extra("base-year", self.0.base_year(date.year))
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        if fields.era.is_some_and(|era| era != self.0.era) {
            return Err(CalendarError::UnknownEra);
        }
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        let day = fields.require_day()?;
        self.0.to_fixed(fields.year, month.ordinal, day)?;
        Ok(YearCountDate {
            year: fields.year,
            month: month.ordinal,
            day,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_spanish_era_is_the_julian_year_plus_thirty_eight() {
        // Wikipedia, "Spanish era": Era 941 is AD 903. The page's other
        // example, a document of 1137 dated "Era millesima centesima
        // LXXVI", is 39 years apart, which a year of the Incarnation begun
        // in March would explain and the page does not say; it is not an
        // anchor (`docs/systems/era-counts.md`).
        assert_eq!(SPANISH_ERA.base_year(941), 903);
        assert_eq!(SPANISH_ERA.to_fixed(941, 3, 1), julian::to_fixed(903, 3, 1));
        assert_eq!(SPANISH_ERA.base_year(1_176), 1_138);
        // The epoch: 1 January 38 BC, astronomical year -37.
        assert_eq!(SPANISH_ERA.earliest(), julian::to_fixed(-37, 1, 1).unwrap());
        assert_eq!(
            SPANISH_ERA.from_fixed(SPANISH_ERA.earliest()),
            Ok((1, 1, 1))
        );
        // The days are Julian: Era 1138, AD 1100, has a 29 February that
        // the Gregorian calendar would not.
        assert!(SPANISH_ERA.to_fixed(1_138, 2, 29).is_ok());
    }

    #[test]
    fn the_kingdoms_dropped_the_spanish_era_in_the_years_the_source_gives() {
        let aragon = spanish_era_abandonment("aragon").unwrap();
        assert_eq!(aragon.abandoned_by(1_348), Some(false));
        assert_eq!(aragon.abandoned_by(1_349), None);
        assert_eq!(aragon.abandoned_by(1_350), None);
        assert_eq!(aragon.abandoned_by(1_351), Some(true));
        let portugal = spanish_era_abandonment("Portugal").unwrap();
        assert_eq!(portugal.abandoned_by(1_423), Some(true));
        let navarre = spanish_era_abandonment("Navarre").unwrap();
        assert_eq!(navarre.abandoned_by(1_500), None);
        assert!(spanish_era_abandonment("Leon").is_none());
        // In the order the source lists them, which is the order of the
        // years.
        let mut previous = 0;
        for entry in SPANISH_ERA_ABANDONMENT {
            if let Some(latest) = entry.latest {
                assert!(latest > previous, "{}", entry.kingdom);
                previous = latest;
            }
        }
    }

    #[test]
    fn the_masonic_years_are_the_lodges_worked_examples() {
        // Lodge No. 43: 2010 is 6010 A.L., 2540 A.I., 3010 A.Dep., 892 A.O.;
        // Wikipedia, "Anno Lucis": AD 2026 is AL 6026.
        let day = gregorian::to_fixed(2010, 6, 1).unwrap();
        for (count, year) in [
            (ANNO_LUCIS, 6_010),
            (ANNO_INVENTIONIS, 2_540),
            (ANNO_DEPOSITIONIS, 3_010),
            (ANNO_ORDINIS, 892),
        ] {
            assert_eq!(count.from_fixed(day), Ok((year, 6, 1)), "{}", count.id);
        }
        assert_eq!(
            ANNO_LUCIS.from_fixed(gregorian::to_fixed(2026, 9, 26).unwrap()),
            Ok((6_026, 9, 26))
        );
    }

    #[test]
    fn ada_is_the_common_year_plus_eight_thousand() {
        // Wikipedia, "After the Development of Agriculture": 1978 was
        // 9978 ADA and 2026 is 10026 ADA.
        for (common, ada) in [(1_978, 9_978), (2_026, 10_026)] {
            let rd = gregorian::to_fixed(common, 1, 1).unwrap();
            assert_eq!(ADA.from_fixed(rd), Ok((ada, 1, 1)));
        }
    }

    #[test]
    fn every_count_starts_at_its_year_one_and_refuses_the_day_before() {
        for count in ALL {
            let first = count.earliest();
            assert_eq!(count.from_fixed(first), Ok((1, 1, 1)), "{}", count.id);
            assert_eq!(
                count.from_fixed(Rd(first.0 - 1)),
                Err(CalendarError::BeforeEpoch),
                "{}",
                count.id
            );
            assert_eq!(
                count.to_fixed(0, 12, 31),
                Err(CalendarError::YearOutOfRange)
            );
        }
        // Anno Ordinis 1 is 1119: the Templars' vows of 1118 are year 0.
        assert_eq!(
            ANNO_ORDINIS.earliest(),
            gregorian::to_fixed(1_119, 1, 1).unwrap()
        );
    }

    #[test]
    fn every_count_round_trips_through_its_fields() {
        for count in ALL {
            let calendar = YearCountCalendar(*count);
            let meta = calendar.meta();
            let first = meta.earliest.unwrap().0;
            for rd in (first..first + 3_000_000).step_by(997) {
                let date = calendar.from_fixed(Rd(rd)).unwrap();
                assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)), "{} {rd}", count.id);
                let fields = calendar.to_fields(date).unwrap();
                assert_eq!(fields.era, Some(count.era));
                assert_eq!(calendar.from_fields(&fields), Ok(date), "{} {rd}", count.id);
                assert_eq!(
                    calendar.is_leap_year(date.year),
                    Ok(count.base.is_leap_year(date.year - count.offset))
                );
            }
            assert_eq!(
                calendar.from_fields(&DateFields::ymd(100, 1, 1).with_era("ad")),
                Err(CalendarError::UnknownEra)
            );
            assert_eq!(calendar.is_leap_year(0), Err(CalendarError::YearOutOfRange));
        }
    }

    #[test]
    fn the_identifiers_and_era_codes_are_distinct() {
        for (index, count) in ALL.iter().enumerate() {
            for other in &ALL[index + 1..] {
                assert_ne!(count.id, other.id);
                assert_ne!(count.era, other.era);
            }
        }
    }
}
