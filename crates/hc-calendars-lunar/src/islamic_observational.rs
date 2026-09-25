//! A *prediction* of the observational Hijri calendar — CLDR
//! `islamic-rgsa`.
//!
//! # Read this before using it
//!
//! The Hijri calendar as kept in religious practice is **announced, not
//! computed**: a month begins when a qualified observer reports the young
//! crescent and a competent authority accepts the report, and different
//! authorities reach different answers on the same evening every year.
//! What this module computes is *whether the crescent should have been
//! visible*, under one published criterion, from one place, in a clear sky.
//! That is a forecast of an observation, not a record of what was
//! proclaimed. **Do not use it to state when Ramadan began.** For Saudi
//! Arabia's civil calendar use [`crate::islamic_umalqura`], which reads the
//! published table; for an administrative calendar use
//! [`crate::islamic_civil`].
//!
//! Measured against the Umm al-Qura table over 1400–1445 AH, this module
//! puts the first of the month **one day later for 322 of 552 months** —
//! 58% — and never earlier, and never by more than one day. The figure is
//! asserted in the crate's tests; what it measures, why the difference runs
//! one way, and a named evening on which the table, the prediction and the
//! Saudi announcement can be compared are in `docs/systems/hijri.md`, with
//! the sources for the criterion and the Umm al-Qura rules.
//!
//! # The criterion
//!
//! [`VisibilityCriterion::SHAUKAT`] is the crescent-visibility
//! test of *Calendrical Calculations*, which the published code attributes
//! to S. K. Shaukat: at the moment the Sun is 4.5° below the horizon on the
//! evening before the day, the Moon must be past conjunction and short of
//! first quarter, its arc of light must be at least 10.6°, and it must be
//! more than 4.1° above the horizon. The arc of light is the true angular
//! separation from the Sun, `arccos(cos β · cos φ)` for lunar latitude β and
//! elongation φ.
//!
//! `hc-astro` offers dusk only at the three standard twilight depressions,
//! so the 4.5° instant is interpolated linearly between sunset and civil
//! dusk, from the depression the Sun has at sunset. At Mecca's latitude the
//! depression grows close enough to linearly over that stretch for the
//! error to be well under a minute, which is far inside the tolerance of a
//! criterion whose real uncertainty is the weather; no test measures it.
//!
//! # The place
//!
//! [`MECCA`] by default; [`ObservationSite`] takes any location, because the
//! whole point of an observational calendar is that the answer depends on
//! where you stand.
//!
//! # Range
//!
//! 1900-01-01 to 2100-12-31 Gregorian. The sunset and lunar models are good
//! well beyond that, but a two-century window is as far as it is honest to
//! carry a prediction of a human decision.

use hc_astro::riseset::{lunar_altitude, sunrise_altitude_degrees};
use hc_astro::{Location, MEAN_SYNODIC_MONTH, Twilight, dusk, sunset};
use hc_calendar::fixed::Moment;
use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};
use hc_core::math::{RAD_TO_DEG, acos, cos_deg, floor, round};

use crate::civil;
use crate::tabular::{CIVIL_EPOCH, ERA, IslamicDate};

/// The machine identifier CLDR uses for this calendar.
///
/// CLDR describes `islamic-rgsa` as "Hijri calendar, Saudi Arabia
/// sighting"; the identifier names the intent, and the caveats above say
/// what this module can honestly deliver against it.
pub const ID: CalendarId = CalendarId("islamic-rgsa");

/// The fixed day on which this calendar places 1 Muḥarram 1 AH, used only to
/// count elapsed months.
pub const EPOCH: Rd = CIVIL_EPOCH;

/// The earliest fixed day this calendar converts.
pub const EARLIEST: Rd = civil::to_rd(1900, 1, 1);

/// The latest fixed day this calendar converts.
pub const LATEST: Rd = civil::to_rd(2100, 12, 31);

/// Mecca: 21°25′21″N, 39°49′34″E, 298 m.
///
/// These are the crate's own figures for the Great Mosque, not taken from
/// a named source. The `mecca` constant of the published *Calendrical
/// Calculations* code is 21°25′24″N, 39°49′24″E, 298 m, a few hundred
/// metres away; the document records the difference.
pub const MECCA: Location = Location::new(21.422_5, 39.826_2, 298.0);

/// The thresholds a young crescent must clear to count as visible.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VisibilityCriterion {
    /// How far the Sun must be below the horizon when the sky is judged.
    pub evaluation_depression_degrees: f64,
    /// The least arc of light, in degrees, that counts as visible.
    pub minimum_arc_of_light_degrees: f64,
    /// The least altitude of the Moon, in degrees, that counts as visible.
    pub minimum_altitude_degrees: f64,
}

impl VisibilityCriterion {
    /// S. K. Shaukat's criterion, as the published code of Reingold and
    /// Dershowitz, *Calendrical Calculations*, carries it under the name
    /// `shaukat-criterion` for their observational Islamic calendar
    /// (`reingold2018code` in `docs/references.bib`, where it was read).
    /// The same code offers Yallop's criterion as an alternative, which
    /// this crate does not carry.
    pub const SHAUKAT: Self = Self {
        evaluation_depression_degrees: 4.5,
        minimum_arc_of_light_degrees: 10.6,
        minimum_altitude_degrees: 4.1,
    };
}

impl Default for VisibilityCriterion {
    fn default() -> Self {
        Self::SHAUKAT
    }
}

/// Where the crescent is looked for, and by what standard.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ObservationSite {
    /// The observer's position.
    pub location: Location,
    /// The thresholds applied.
    pub criterion: VisibilityCriterion,
}

impl ObservationSite {
    /// An observation site.
    #[must_use]
    pub const fn new(location: Location, criterion: VisibilityCriterion) -> Self {
        Self {
            location,
            criterion,
        }
    }

    /// Mecca, judged by Shaukat's criterion.
    pub const MECCA: Self = Self::new(MECCA, VisibilityCriterion::SHAUKAT);

    /// The moment, in Universal Time, at which the evening of `rd` is
    /// judged — when the Sun reaches the criterion's depression.
    ///
    /// `None` when the Sun does not set or twilight does not end on that day,
    /// which happens at high latitudes and is a real answer: no observation
    /// is possible.
    #[must_use]
    pub fn evaluation_moment(&self, rd: Rd) -> Option<Moment> {
        let set = sunset(rd, self.location)?;
        let civil_dusk = dusk(rd, self.location, Twilight::Civil)?;
        // The Sun's depression at sunset is not zero: the upper limb is on
        // the horizon, refraction has lifted it, and the horizon itself dips
        // with the observer's height.
        let at_sunset = -sunrise_altitude_degrees(self.location.elevation_metres);
        let span = Twilight::Civil.depression_degrees() - at_sunset;
        if span <= 0.0 {
            return None;
        }
        let fraction = (self.criterion.evaluation_depression_degrees - at_sunset) / span;
        Some(Moment(set.0 + fraction * (civil_dusk.0 - set.0)))
    }

    /// Whether the crescent should have been visible on the evening that
    /// begins the Hijri day `rd`.
    ///
    /// That evening is the dusk of `rd - 1`, because the Hijri day begins at
    /// sunset.
    #[must_use]
    pub fn crescent_visible_on_the_eve_of(&self, rd: Rd) -> bool {
        let Some(moment) = self.evaluation_moment(Rd(rd.0 - 1)) else {
            return false;
        };
        let elongation = hc_astro::lunar_phase(moment);
        // Past conjunction and short of first quarter: an old moon in the
        // morning sky satisfies every other test and is not a new month.
        if !(0.0..90.0).contains(&elongation) {
            return false;
        }
        let latitude = hc_astro::lunar::lunar_latitude(moment);
        let arc_of_light = acos(cos_deg(latitude) * cos_deg(elongation)) * RAD_TO_DEG;
        if !(self.criterion.minimum_arc_of_light_degrees..=90.0).contains(&arc_of_light) {
            return false;
        }
        lunar_altitude(moment, self.location) > self.criterion.minimum_altitude_degrees
    }

    /// The most recent day whose eve carried a visible crescent, on or before
    /// `rd` — that is, the first day of the Hijri month containing `rd`.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::AstronomicalModelFailure`] when no visible
    /// crescent is found within a lunation and a half, which at Mecca means
    /// the model has broken rather than that the Moon has.
    pub fn month_start_on_or_before(&self, rd: Rd) -> CalendarResult<Rd> {
        // The day of the last mean conjunction, from the Moon's elongation.
        let elongation = hc_astro::lunar_phase(Moment(rd.0 as f64));
        let mean = rd.0 - floor(elongation / 360.0 * MEAN_SYNODIC_MONTH) as i64;
        // Within three days of the conjunction the crescent may not yet have
        // been seen, in which case the month began a lunation earlier.
        let first = if rd.0 - mean <= 3 && !self.crescent_visible_on_the_eve_of(rd) {
            mean - 30
        } else {
            mean - 2
        };
        for step in 0..45 {
            let candidate = Rd(first + step);
            if self.crescent_visible_on_the_eve_of(candidate) {
                return Ok(candidate);
            }
        }
        Err(CalendarError::AstronomicalModelFailure)
    }
}

impl Default for ObservationSite {
    fn default() -> Self {
        Self::MECCA
    }
}

/// The predicted observational Hijri calendar.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IslamicObservationalCalendar {
    site: ObservationSite,
}

impl IslamicObservationalCalendar {
    /// The calendar as observed from a given site.
    #[must_use]
    pub const fn new(site: ObservationSite) -> Self {
        Self { site }
    }

    /// The calendar as observed from Mecca.
    pub const MECCA: Self = Self::new(ObservationSite::MECCA);

    /// The site this calendar observes from.
    #[must_use]
    pub const fn site(&self) -> ObservationSite {
        self.site
    }

    /// Range check.
    fn check_range(rd: Rd) -> CalendarResult<()> {
        if rd < EARLIEST {
            return Err(CalendarError::BeforeEpoch);
        }
        if rd > LATEST {
            return Err(CalendarError::AfterSupportedRange);
        }
        Ok(())
    }

    /// The predicted Hijri year, month and day of a fixed day.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::BeforeEpoch`] or
    /// [`CalendarError::AfterSupportedRange`] outside the supported range,
    /// and [`CalendarError::AstronomicalModelFailure`] when the crescent
    /// search does not converge.
    pub fn decompose(&self, rd: Rd) -> CalendarResult<(i64, u8, u8)> {
        Self::check_range(rd)?;
        let start = self.site.month_start_on_or_before(rd)?;
        let elapsed_months = round((start.0 - EPOCH.0) as f64 / MEAN_SYNODIC_MONTH) as i64;
        let year = elapsed_months.div_euclid(12) + 1;
        let month = elapsed_months.rem_euclid(12) as u8 + 1;
        let day = (rd.0 - start.0 + 1) as u8;
        Ok((year, month, day))
    }

    /// The fixed day of a predicted Hijri date.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::MonthOutOfRange`],
    /// [`CalendarError::DayOutOfRange`], the range errors, or
    /// [`CalendarError::AstronomicalModelFailure`].
    pub fn compose(&self, year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
        if month == 0 || month > 12 {
            return Err(CalendarError::MonthOutOfRange);
        }
        if day == 0 || day > 30 {
            return Err(CalendarError::DayOutOfRange);
        }
        let elapsed_months = (year - 1) * 12 + month as i64 - 1;
        let midmonth = EPOCH.0 + floor((elapsed_months as f64 + 0.5) * MEAN_SYNODIC_MONTH) as i64;
        let start = self.site.month_start_on_or_before(Rd(midmonth))?;
        let rd = Rd(start.0 + day as i64 - 1);
        Self::check_range(rd)?;
        // The length of an observed month is not knowable in advance, so the
        // date is validated by reading it back.
        let (round_year, round_month, round_day) = self.decompose(rd)?;
        if (round_year, round_month) != (year, month) {
            return Err(CalendarError::MonthOutOfRange);
        }
        if round_day != day {
            return Err(CalendarError::DayOutOfRange);
        }
        Ok(rd)
    }
}

impl Default for IslamicObservationalCalendar {
    fn default() -> Self {
        Self::MECCA
    }
}

impl Calendar for IslamicObservationalCalendar {
    type Date = IslamicDate;

    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        hc_calendar::shape::SOLAR_TWELVE
    }

    /// A year of 355 days: seven of its twelve predicted months ran to
    /// thirty. Nothing is intercalated by rule, so the answer is read off
    /// the crescents like everything else here.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        let start = self.compose(year, 1, 1)?;
        let next = self.compose(year + 1, 1, 1)?;
        Ok(next.0 - start.0 == 355)
    }

    /// The Islamic day begins at sunset, which is also why the month begins
    /// with a crescent seen after one.
    fn day_boundary(&self) -> hc_calendar::DayBoundary {
        hc_calendar::DayBoundary::Sunset
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: ID,
            english_name: "Hijri (observational, predicted)",
            year_kind: YearKind::EpochForward,
            has_leap_months: false,
            is_astronomical: true,
            earliest: Some(EARLIEST),
            latest: Some(LATEST),
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        self.compose(date.year, date.month, date.day)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (year, month, day) = self.decompose(rd)?;
        Ok(IslamicDate { year, month, day })
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
        let date = IslamicDate {
            year: fields.year,
            month: month.ordinal,
            day: fields.require_day()?,
        };
        self.compose(date.year, date.month, date.day)?;
        Ok(date)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::islamic_umalqura;

    #[test]
    fn the_evaluation_moment_falls_between_sunset_and_civil_dusk() {
        let site = ObservationSite::MECCA;
        for offset in (0..365i64).step_by(17) {
            let rd = Rd(civil::to_rd(2024, 1, 1).0 + offset);
            let set = sunset(rd, MECCA).expect("the Sun sets at Mecca");
            let civil_dusk = dusk(rd, MECCA, Twilight::Civil).expect("twilight ends at Mecca");
            let judged = site.evaluation_moment(rd).expect("both exist");
            assert!(judged.0 > set.0, "before sunset at RD {rd}");
            assert!(judged.0 < civil_dusk.0, "after civil dusk at RD {rd}");
        }
    }

    #[test]
    fn the_crescent_is_invisible_around_conjunction_and_visible_in_a_run_after_it() {
        // The criterion is not "this is the first day of the month": it is
        // "the crescent was up and lit", which stays true every evening from
        // first sighting until first quarter. Picking the *first* evening of
        // each run is `month_start_on_or_before`'s job, and this test pins
        // down the shape the search relies on: a run of five to eight
        // visible evenings, then a long gap around the next conjunction.
        let site = ObservationSite::MECCA;
        let start = civil::to_rd(2024, 3, 1);
        let mut visible = [false; 90];
        for (offset, seen) in visible.iter_mut().enumerate() {
            *seen = site.crescent_visible_on_the_eve_of(Rd(start.0 + offset as i64));
        }
        let mut runs = 0;
        let mut longest_run = 0;
        let mut longest_gap = 0;
        let mut run = 0;
        let mut gap = 0;
        for seen in visible {
            let seen: bool = seen;
            if seen {
                run += 1;
                longest_gap = longest_gap.max(gap);
                if gap > 0 {
                    runs += 1;
                }
                gap = 0;
            } else {
                gap += 1;
                longest_run = longest_run.max(run);
                run = 0;
            }
        }
        assert!((2..=4).contains(&runs), "{runs} runs of visible evenings");
        assert!(
            (4..=9).contains(&longest_run),
            "longest run was {longest_run} evenings"
        );
        assert!(
            (18..=26).contains(&longest_gap),
            "longest gap was {longest_gap} evenings"
        );
    }

    #[test]
    fn a_month_start_is_followed_by_twenty_nine_or_thirty_days() {
        let site = ObservationSite::MECCA;
        let mut cursor = site
            .month_start_on_or_before(civil::to_rd(2020, 1, 15))
            .expect("converges");
        for _ in 0..40 {
            let next = site
                .month_start_on_or_before(Rd(cursor.0 + 32))
                .expect("converges");
            let length = next.0 - cursor.0;
            assert!(
                (29..=30).contains(&length),
                "month at {cursor} ran {length} days"
            );
            cursor = next;
        }
    }

    #[test]
    fn the_calendar_round_trips_over_four_years_of_days() {
        let calendar = IslamicObservationalCalendar::MECCA;
        let start = civil::to_rd(2021, 6, 1);
        for offset in 0..1_500i64 {
            let rd = Rd(start.0 + offset);
            let date = calendar.from_fixed(rd).expect("in range");
            assert_eq!(calendar.to_fixed(date), Ok(rd), "RD {rd}");
        }
    }

    #[test]
    fn the_calendar_round_trips_at_both_ends_of_its_range() {
        let calendar = IslamicObservationalCalendar::MECCA;
        for start in [EARLIEST.0 + 40, LATEST.0 - 840] {
            for offset in 0..800i64 {
                let rd = Rd(start + offset);
                let date = calendar.from_fixed(rd).expect("in range");
                assert_eq!(calendar.to_fixed(date), Ok(rd), "RD {rd}");
            }
        }
    }

    #[test]
    fn years_run_to_twelve_months_of_twenty_nine_or_thirty_days() {
        let calendar = IslamicObservationalCalendar::MECCA;
        for year in 1_440..1_445i64 {
            let mut total = 0i64;
            for month in 1..=12u8 {
                let first = calendar.compose(year, month, 1).expect("the month exists");
                let next = if month == 12 {
                    calendar.compose(year + 1, 1, 1).expect("the month exists")
                } else {
                    calendar
                        .compose(year, month + 1, 1)
                        .expect("the month exists")
                };
                let length = next.0 - first.0;
                assert!((29..=30).contains(&length), "{year}-{month} ran {length}");
                total += length;
            }
            assert!((354..=356).contains(&total), "year {year} ran {total}");
        }
    }

    #[test]
    fn the_prediction_disagrees_with_the_saudi_table_and_the_crate_says_by_how_much() {
        // The honest number. A prediction of a sighting is not a record of a
        // proclamation, and this test exists to keep the difference visible.
        let calendar = IslamicObservationalCalendar::MECCA;
        let mut months = 0u32;
        let mut disagreements = 0u32;
        let mut later = 0u32;
        let mut worst = 0i64;
        for year in 1_400..1_446i64 {
            for month in 1..=12u8 {
                let Ok(table) = islamic_umalqura::to_fixed(year, month, 1) else {
                    continue;
                };
                if table < EARLIEST || table > LATEST {
                    continue;
                }
                let Ok(predicted) = calendar.compose(year, month, 1) else {
                    continue;
                };
                months += 1;
                let difference = predicted.0 - table.0;
                worst = worst.max(difference.abs());
                if difference != 0 {
                    disagreements += 1;
                }
                if difference > 0 {
                    later += 1;
                }
            }
        }
        assert_eq!(months, 552);
        assert_eq!(worst, 1, "the two never differ by more than a day");
        assert_eq!(
            later, disagreements,
            "the prediction is never the earlier one"
        );
        // 322 of 552, 58%. Stated exactly, because a range would let the
        // number drift without anyone noticing.
        assert_eq!(disagreements, 322);
    }

    #[test]
    fn the_site_is_configurable_and_the_answer_depends_on_it() {
        // The whole point of an observational calendar: a crescent low in
        // the west at Mecca can be already set, or comfortably up, elsewhere.
        let jakarta = ObservationSite::new(
            Location::new(-6.2, 106.8, 8.0),
            VisibilityCriterion::SHAUKAT,
        );
        let mecca = ObservationSite::MECCA;
        let mut differences = 0;
        for offset in 0..500i64 {
            let rd = Rd(civil::to_rd(2020, 1, 1).0 + offset);
            if jakarta.crescent_visible_on_the_eve_of(rd)
                != mecca.crescent_visible_on_the_eve_of(rd)
            {
                differences += 1;
            }
        }
        assert!(differences > 0, "the location made no difference at all");
    }

    #[test]
    fn a_stricter_criterion_sees_fewer_crescents() {
        let strict = ObservationSite::new(
            MECCA,
            VisibilityCriterion {
                evaluation_depression_degrees: 4.5,
                minimum_arc_of_light_degrees: 14.0,
                minimum_altitude_degrees: 6.0,
            },
        );
        let standard = ObservationSite::MECCA;
        let count = |site: &ObservationSite| {
            (0..400i64)
                .filter(|offset| {
                    site.crescent_visible_on_the_eve_of(Rd(civil::to_rd(2022, 1, 1).0 + offset))
                })
                .count()
        };
        assert!(count(&strict) < count(&standard));
    }

    #[test]
    fn the_range_is_refused_rather_than_extrapolated() {
        let calendar = IslamicObservationalCalendar::MECCA;
        assert_eq!(
            calendar.from_fixed(Rd(EARLIEST.0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(
            calendar.from_fixed(Rd(LATEST.0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
    }

    #[test]
    fn out_of_range_fields_name_the_field_that_is_wrong() {
        let calendar = IslamicObservationalCalendar::MECCA;
        assert_eq!(
            calendar.compose(1_445, 13, 1),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            calendar.compose(1_445, 0, 1),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            calendar.compose(1_445, 1, 0),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            calendar.compose(1_445, 1, 31),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            calendar.from_fields(&DateFields::ymd_leap_month(1_445, 1, 1)),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            calendar.from_fields(&DateFields::ymd(1_445, 1, 1).with_era("AD")),
            Err(CalendarError::UnknownEra)
        );
    }

    #[test]
    fn the_metadata_admits_the_calendar_is_astronomical() {
        let meta = IslamicObservationalCalendar::MECCA.meta();
        assert_eq!(meta.id, CalendarId("islamic-rgsa"));
        assert!(meta.is_astronomical);
        assert!(!meta.has_leap_months);
        assert_eq!(
            IslamicObservationalCalendar::default().site().location,
            MECCA
        );
    }
}
