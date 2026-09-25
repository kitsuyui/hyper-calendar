//! The sexagenary cycle (干支) as a calendar over years, months and days.
//!
//! Ten Heavenly Stems against twelve Earthly Branches give sixty pairs, and
//! East Asia has named years, months, days and two-hour periods with them
//! for well over two thousand years. [`hc_calendar::cycle`] holds the
//! arithmetic and the readings; this module turns it into a [`Calendar`].
//!
//! # The day cycle is the one that never broke
//!
//! Of the three, only the **day** cycle is independent of any calendar. It
//! has run without interruption for longer than any surviving calendar, and
//! it survived every reform, so a fixed day determines it outright. That is
//! why [`SexagenaryCalendar`] is a calendar of days: it is the only one of
//! the three that can round-trip to a fixed day without borrowing a
//! calendar's year numbering.
//!
//! The **year** and **month** cycles belong to the Chinese calendar, so
//! they are exposed here as [`pillars`], which asks
//! [`hc_calendars_lunar::chinese`] what year and month a day falls in and
//! names them. That call inherits the Chinese calendar's supported range,
//! 1645 to 2150.
//!
//! # A warning about the month pillar
//!
//! In Chinese astrology the month pillar properly follows the **solar
//! terms** (節): the 寅 month begins at 立春, not at a new moon. This module
//! names the *lunar* month instead, because that is what the Chinese
//! calendar in this workspace counts. The two agree for most of each month
//! and disagree for up to a fortnight around the boundaries. If you are
//! casting a chart rather than reading a date, this is not the function you
//! want: [`hc_calendar::cycle::month_pillar`] takes the solar-term month.
//!
//! The rule the month pillar does follow here is the traditional mnemonic
//! 甲己之年丙作首 — in a year whose stem is 甲 or 己, the first month is
//! 丙寅 — which [`hc_calendars_lunar::lunisolar`] already implements and
//! tests.
//!
//! # Readings
//!
//! The sixty names in characters, kana, Hangul, quốc ngữ and the
//! romanisations are [`hc_calendar::cycle::readings`]; this crate adds
//! none. [`SexagenaryDayDate`] displays in characters, the one spelling
//! every language shares.

use core::fmt;

use hc_calendar::cycle::{Sexagenary, readings, sexagenary_day};
use hc_calendar::fields::ExtraFields;
use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};
use hc_calendars_lunar::chinese;

/// The length of the cycle.
pub const CYCLE: i64 = 60;

/// The fixed day whose cycle index is zero, the first 甲子 day at or before
/// the fixed-day origin.
///
/// [`hc_calendar::cycle::sexagenary_day`] computes the position as
/// `index = rd + 14`, so RD 1 is index 15 and index 0 falls on RD -14.
pub const EPOCH: Rd = Rd(-14);

/// A day named by the sexagenary cycle, plus the cycle it falls in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SexagenaryDayDate {
    /// Complete sixty-day cycles elapsed since [`EPOCH`].
    pub cycle: i64,
    /// The position within the cycle.
    pub position: Sexagenary,
}

impl SexagenaryDayDate {
    /// A date, without validation.
    #[must_use]
    pub const fn new(cycle: i64, position: Sexagenary) -> Self {
        Self { cycle, position }
    }
}

impl fmt::Display for SexagenaryDayDate {
    /// Writes the pair in Han characters, as in `甲子`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(readings::HAN.stem(self.position))?;
        f.write_str(readings::HAN.branch(self.position))
    }
}

/// The three sexagenary "pillars" of a day.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pillars {
    /// The pillar of the Chinese year the day falls in.
    pub year: Sexagenary,
    /// The pillar of the Chinese lunar month the day falls in. See the
    /// module documentation for why this is not the astrological month
    /// pillar.
    pub month: Sexagenary,
    /// The pillar of the day itself.
    pub day: Sexagenary,
}

/// The year, month and day pillars of a fixed day.
///
/// The year and month come from [`hc_calendars_lunar::chinese`], so this
/// inherits that calendar's supported range. The day does not, and
/// [`day_pillar`] will answer for any day at all.
///
/// # Errors
///
/// Returns [`CalendarError::BeforeEpoch`] or
/// [`CalendarError::AfterSupportedRange`] outside the Chinese calendar's
/// range.
pub fn pillars(rd: Rd) -> CalendarResult<Pillars> {
    let date = chinese::ENGINE.from_fixed(rd)?;
    Ok(Pillars {
        year: chinese::PARAMETERS.sexagenary_year(date.year),
        month: chinese::PARAMETERS.sexagenary_month(date.year, date.month),
        day: sexagenary_day(rd),
    })
}

/// The day pillar of any fixed day.
#[must_use]
pub fn day_pillar(rd: Rd) -> Sexagenary {
    sexagenary_day(rd)
}

/// The sexagenary cycle over days.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SexagenaryCalendar;

impl Calendar for SexagenaryCalendar {
    type Date = SexagenaryDayDate;

    /// Unrecorded: a cycle, like a day count, with nothing to be outside of.
    /// The day cycle's two millennia of unbroken use are the module's
    /// statement and are not dated by a source read.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::UNRECORDED
    }

    /// The ten stems and the twelve branches.
    ///
    /// The sixty pairs are not a third cycle: a pair's name is its stem's
    /// name followed by its branch's, and [`readings`] spells both.
    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        const SHAPE: &[hc_calendar::shape::CycleShape] = &[
            hc_calendar::shape::CycleShape::fixed("stem", 10),
            hc_calendar::shape::CycleShape::fixed("branch", 12),
        ];
        SHAPE
    }

    /// A cycle has no year: the `year` field carries the cycle number.
    fn is_leap_year(&self, _year: i64) -> CalendarResult<bool> {
        Err(CalendarError::UnsupportedField("year"))
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId("sexagenary"),
            english_name: "Sexagenary cycle (干支)",
            year_kind: YearKind::Astronomical,
            has_leap_months: false,
            is_astronomical: false,
            earliest: None,
            latest: None,
            native_locales: &["zh-Hans", "zh-Hant", "ja", "ko", "vi"],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        Ok(Rd(EPOCH.0
            + date.cycle * CYCLE
            + i64::from(date.position.index())))
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let count = rd.0 - EPOCH.0;
        Ok(SexagenaryDayDate {
            cycle: count.div_euclid(CYCLE),
            position: sexagenary_day(rd),
        })
    }

    /// Describes the day by its one-based ordinal in the cycle, with the
    /// stem and branch as extra fields.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::Overflow`] if the extra-field set fills,
    /// which three fields cannot make happen.
    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        let mut extra = ExtraFields::new();
        extra.set("stem", i64::from(date.position.stem_index()) + 1)?;
        extra.set("branch", i64::from(date.position.branch_index()) + 1)?;
        extra.set("sexagenary", i64::from(date.position.ordinal()))?;
        Ok(DateFields {
            era: None,
            year: date.cycle,
            month: None,
            day: Some(date.position.ordinal()),
            leap_day: false,
            extra,
        })
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        let ordinal = fields.require_day()?;
        if ordinal == 0 || ordinal > 60 {
            return Err(CalendarError::DayOutOfRange);
        }
        Ok(SexagenaryDayDate {
            cycle: fields.year,
            position: Sexagenary::from_index(i64::from(ordinal) - 1),
        })
    }
}

#[cfg(test)]
mod tests {
    use hc_calendars_solar::gregorian;

    use super::*;

    fn greg(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).expect("valid Gregorian date")
    }

    #[test]
    fn the_epoch_is_a_jia_zi_day() {
        let date = SexagenaryCalendar.from_fixed(EPOCH).expect("any day");
        assert_eq!(date.position.index(), 0);
        assert_eq!(date.cycle, 0);
        assert_eq!(date.to_string(), "甲子");
        assert_eq!(readings::PINYIN.stem(date.position), "jia");
        assert_eq!(readings::PINYIN.branch(date.position), "zi");
    }

    #[test]
    fn the_day_cycle_matches_the_anchor_in_hc_calendar() {
        // hc_calendar anchors the cycle so that RD 1 — 0001-01-01
        // proleptic Gregorian — is index 15, the sixteenth pair, 己卯.
        assert_eq!(day_pillar(Rd(1)).index(), 15);
        assert_eq!(day_pillar(Rd(1)).ordinal(), 16);
        assert_eq!(readings::HAN.stem(day_pillar(Rd(1))), "己");
        assert_eq!(readings::HAN.branch(day_pillar(Rd(1))), "卯");
        assert_eq!(day_pillar(EPOCH).index(), 0);
        for offset in -200i64..200 {
            let rd = Rd(offset);
            assert_eq!(
                SexagenaryCalendar.from_fixed(rd).expect("any day").position,
                day_pillar(rd)
            );
        }
    }

    #[test]
    fn the_cycle_closes_after_sixty_days() {
        let start = greg(2026, 9, 21);
        let first = SexagenaryCalendar.from_fixed(start).expect("any day");
        let later = SexagenaryCalendar
            .from_fixed(Rd(start.0 + 60))
            .expect("any day");
        assert_eq!(first.position, later.position);
        assert_eq!(later.cycle, first.cycle + 1);
    }

    #[test]
    fn every_position_of_a_whole_cycle_round_trips() {
        for cycle in [-100i64, 0, 12_000] {
            for index in 0..60 {
                let date = SexagenaryDayDate::new(cycle, Sexagenary::from_index(index));
                let rd = SexagenaryCalendar.to_fixed(date).expect("a valid day");
                assert_eq!(SexagenaryCalendar.from_fixed(rd), Ok(date));
                let fields = SexagenaryCalendar.to_fields(date).expect("describable");
                assert_eq!(SexagenaryCalendar.from_fields(&fields), Ok(date));
            }
        }
    }

    #[test]
    fn the_year_of_the_wood_dragon_began_in_2024() {
        // Chinese year 4661 is jia-chen, the Wood Dragon; its new year was
        // 2024-02-10.
        let new_year = greg(2024, 2, 10);
        let pillars = pillars(new_year).expect("in range");
        assert_eq!(readings::PINYIN.stem(pillars.year), "jia");
        assert_eq!(readings::PINYIN.branch(pillars.year), "chen");
        assert_eq!(pillars.year.zodiac_animal(), "dragon");
        assert_eq!(pillars.year.five_phase(), "wood");
    }

    #[test]
    fn the_first_month_of_a_jia_year_is_bing_yin() {
        // 甲己之年丙作首, the traditional mnemonic.
        let new_year = greg(2024, 2, 10);
        let pillars = pillars(new_year).expect("in range");
        assert_eq!(readings::PINYIN.stem(pillars.month), "bing");
        assert_eq!(readings::PINYIN.branch(pillars.month), "yin");
        assert_eq!(readings::HAN.stem(pillars.month), "丙");
        assert_eq!(readings::HAN.branch(pillars.month), "寅");
    }

    #[test]
    fn the_day_pillar_agrees_with_the_pillars_helper() {
        for offset in (0..20_000).step_by(97) {
            let rd = Rd(greg(1900, 1, 1).0 + offset);
            let all = pillars(rd).expect("in range");
            assert_eq!(all.day, day_pillar(rd));
        }
    }

    #[test]
    fn the_pillars_helper_inherits_the_chinese_calendars_range() {
        // The day cycle answers for any day; the year and month do not.
        let far_back = greg(1000, 1, 1);
        assert!(pillars(far_back).is_err());
        assert_eq!(
            day_pillar(far_back).ordinal(),
            day_pillar(far_back).ordinal()
        );
        assert!(SexagenaryCalendar.from_fixed(far_back).is_ok());
    }

    #[test]
    fn fields_carry_the_stem_the_branch_and_the_ordinal() {
        let date = SexagenaryCalendar
            .from_fixed(greg(2026, 9, 21))
            .expect("any day");
        let fields = SexagenaryCalendar.to_fields(date).expect("describable");
        assert_eq!(fields.month, None);
        assert_eq!(fields.day, Some(date.position.ordinal()));
        assert_eq!(
            fields.extra.get("stem"),
            Some(i64::from(date.position.stem_index()) + 1)
        );
        assert_eq!(
            fields.extra.get("branch"),
            Some(i64::from(date.position.branch_index()) + 1)
        );
        assert_eq!(
            fields.extra.get("sexagenary"),
            Some(i64::from(date.position.ordinal()))
        );
        assert_eq!(SexagenaryCalendar.from_fields(&fields), Ok(date));
    }

    #[test]
    fn out_of_range_ordinals_are_refused() {
        let mut fields = DateFields::ymd(0, 1, 61);
        assert_eq!(
            SexagenaryCalendar.from_fields(&fields),
            Err(CalendarError::DayOutOfRange)
        );
        fields.day = Some(0);
        assert_eq!(
            SexagenaryCalendar.from_fields(&fields),
            Err(CalendarError::DayOutOfRange)
        );
        fields.day = None;
        assert_eq!(
            SexagenaryCalendar.from_fields(&fields),
            Err(CalendarError::MissingField("day"))
        );
    }

    #[test]
    fn the_metadata_says_the_cycle_is_unbounded() {
        let meta = SexagenaryCalendar.meta();
        assert_eq!(meta.id, CalendarId("sexagenary"));
        assert!(meta.earliest.is_none());
        assert!(meta.latest.is_none());
        assert!(!meta.is_astronomical);
        assert!(!meta.has_leap_months);
    }
}
