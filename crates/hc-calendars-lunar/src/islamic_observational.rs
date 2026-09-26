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
//! # The criteria
//!
//! [`VisibilityCriterion::SHAUKAT`] is the crescent-visibility
//! test of *Calendrical Calculations*, which the published code attributes
//! to S. K. Shaukat: at the moment the Sun is 4.5° below the horizon on the
//! evening before the day, the Moon must be past conjunction and short of
//! first quarter, its arc of light must be at least 10.6°, and it must be
//! more than 4.1° above the horizon. The arc of light is the true angular
//! separation from the Sun, `arccos(cos β · cos φ)` for lunar latitude β and
//! elongation φ. It is the criterion of the registered `islamic-rgsa`.
//!
//! `hc-astro` offers dusk only at the three standard twilight depressions,
//! so the 4.5° instant is interpolated linearly between sunset and civil
//! dusk, from the depression the Sun has at sunset. At Mecca's latitude the
//! depression grows close enough to linearly over that stretch for the
//! error to be well under a minute, which is far inside the tolerance of a
//! criterion whose real uncertainty is the weather; no test measures it.
//!
//! [`VisibilityCriterion::YALLOP`] is B. D. Yallop's *q*-test, read in his
//! NAO Technical Note 69 (1997): at Bruin's best time, five ninths of the
//! way from sunset to moonset ([`bruin_best_time`]), the arc of vision
//! ([`arc_of_vision`]) less a cubic in the topocentric width of the
//! crescent ([`crescent_width_arcminutes`]), divided by ten, is `q`
//! ([`yallop_q`]), and the crescent counts as visible when `q > −0.014`,
//! the lower limit of his type B. [`YallopVisibility`] carries all six of
//! his types. The functions reproduce his Table 4 to within 0.005 in `q`
//! from nothing but the date and the place. No calendar under this
//! criterion is registered, because no published calendar under it was
//! found to test one against; `IslamicObservationalCalendar::new` builds
//! one, and at Mecca it begins 21 of the 552 months of 1400–1445 AH a day
//! away from Shaukat's, 6 earlier and 15 later. `docs/systems/hijri.md`
//! says where the published code of *Calendrical Calculations* computes
//! the width differently.
//!
//! # The place
//!
//! [`MECCA`] for the registered `islamic-rgsa`; [`ObservationSite`] takes
//! any location, because the whole point of an observational calendar is
//! that the answer depends on where you stand. No type here has a
//! `Default`: a site and a criterion are named where they are chosen.
//!
//! # Range
//!
//! 1900-01-01 to 2100-12-31 Gregorian. The sunset and lunar models are good
//! well beyond that, but a two-century window is as far as it is honest to
//! carry a prediction of a human decision.

use hc_astro::riseset::{lunar_altitude, solar_altitude, sunrise_altitude_degrees};
use hc_astro::{Location, MEAN_SYNODIC_MONTH, Twilight, dusk, moonset, sunset};
use hc_calendar::fixed::Moment;
use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};
use hc_core::math::{RAD_TO_DEG, acos, cos_deg, floor, round, sin_deg};

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

/// Mecca: 21°25′24″N, 39°49′24″E, 298 m, the `mecca` constant of the
/// published code of *Calendrical Calculations* (`reingold2018code`), which
/// its observational Islamic calendar uses.
pub const MECCA: Location = Location::new(
    21.0 + 25.0 / 60.0 + 24.0 / 3_600.0,
    39.0 + 49.0 / 60.0 + 24.0 / 3_600.0,
    298.0,
);

/// Thresholds on the arc of light and the altitude, judged when the Sun
/// has reached a fixed depression — the shape of Shaukat's criterion.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ArcOfLightCriterion {
    /// How far the Sun must be below the horizon when the sky is judged.
    pub evaluation_depression_degrees: f64,
    /// The least arc of light, in degrees, that counts as visible.
    pub minimum_arc_of_light_degrees: f64,
    /// The least altitude of the Moon, in degrees, that counts as visible.
    pub minimum_altitude_degrees: f64,
}

/// A test on the arc of vision against a cubic in the width of the
/// crescent, judged at Bruin's best time — the shape of Yallop's *q*-test.
///
/// The test parameter is `q = (ARCV − f(W′)) / 10`, where ARCV is
/// [`arc_of_vision`], W′ is [`crescent_width_arcminutes`] and `f` is the
/// polynomial whose coefficients are carried here, constant term first.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QTestCriterion {
    /// The coefficients of `f(W′)`, the arc of vision in degrees at which
    /// `q = 0`, for W′ in minutes of arc; constant term first.
    pub coefficients: [f64; 4],
    /// The value `q` must exceed for the crescent to count as visible.
    pub minimum_q: f64,
}

impl QTestCriterion {
    /// Yallop's *q*-test (NAO Technical Note 69, 1997, equation 6.1), with
    /// the crescent counted as visible above the lower limit of his type B,
    /// "visible under perfect conditions", `q > −0.014` (Table 5).
    ///
    /// The published code of *Calendrical Calculations* writes the same
    /// limit unscaled, `ARCV > f(W) + e` with `e = −0.14`
    /// (`yallop-criterion`); multiplied by ten it is this one.
    pub const YALLOP: Self = Self {
        coefficients: [11.8371, -6.3226, 0.7319, -0.1018],
        minimum_q: YallopVisibility::LOWER_LIMITS[1],
    };

    /// The test parameter `q` for an arc of vision in degrees and a
    /// crescent width in minutes of arc.
    #[must_use]
    pub fn q(&self, arc_of_vision_degrees: f64, width_arcminutes: f64) -> f64 {
        let [c0, c1, c2, c3] = self.coefficients;
        let w = width_arcminutes;
        let threshold = c0 + w * (c1 + w * (c2 + w * c3));
        (arc_of_vision_degrees - threshold) / 10.0
    }
}

/// Yallop's six visibility types, A to F, by the range `q` falls in
/// (NAO Technical Note 69, 1997, Table 5).
///
/// The set is Yallop's and fixed by his table, so it is an `enum`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum YallopVisibility {
    /// (A) `q > +0.216`: easily visible.
    EasilyVisible,
    /// (B) `+0.216 ≥ q > −0.014`: visible under perfect conditions.
    VisibleUnderPerfectConditions,
    /// (C) `−0.014 ≥ q > −0.160`: may need optical aid to find the crescent.
    MayNeedOpticalAid,
    /// (D) `−0.160 ≥ q > −0.232`: will need optical aid to find it.
    NeedsOpticalAid,
    /// (E) `−0.232 ≥ q > −0.293`: not visible with a telescope.
    NotVisibleWithTelescope,
    /// (F) `−0.293 ≥ q`: not visible, below the Danjon limit.
    BelowDanjonLimit,
}

impl YallopVisibility {
    /// The exclusive lower limits of types A to E; F is everything at or
    /// below the last. Yallop, Table 5.
    pub const LOWER_LIMITS: [f64; 5] = [0.216, -0.014, -0.160, -0.232, -0.293];

    /// The type a value of `q` falls in.
    #[must_use]
    pub fn from_q(q: f64) -> Self {
        const ORDER: [YallopVisibility; 5] = [
            YallopVisibility::EasilyVisible,
            YallopVisibility::VisibleUnderPerfectConditions,
            YallopVisibility::MayNeedOpticalAid,
            YallopVisibility::NeedsOpticalAid,
            YallopVisibility::NotVisibleWithTelescope,
        ];
        for (limit, kind) in Self::LOWER_LIMITS.iter().zip(ORDER) {
            if q > *limit {
                return kind;
            }
        }
        Self::BelowDanjonLimit
    }

    /// Yallop's letter for the type, `'A'` to `'F'`.
    #[must_use]
    pub const fn letter(self) -> char {
        match self {
            Self::EasilyVisible => 'A',
            Self::VisibleUnderPerfectConditions => 'B',
            Self::MayNeedOpticalAid => 'C',
            Self::NeedsOpticalAid => 'D',
            Self::NotVisibleWithTelescope => 'E',
            Self::BelowDanjonLimit => 'F',
        }
    }
}

/// The test a young crescent must pass to count as visible.
///
/// Each variant is a *shape* of criterion — what is measured and when —
/// and each named criterion is a constant of one shape. A criterion of a
/// new shape needs new code to evaluate it, which is why the shapes are an
/// `enum`; a criterion of an existing shape with other thresholds is a
/// value, built from the variant's struct.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VisibilityCriterion {
    /// Arc of light and altitude at a fixed solar depression.
    ArcOfLight(ArcOfLightCriterion),
    /// Yallop's *q* at Bruin's best time.
    QTest(QTestCriterion),
}

impl VisibilityCriterion {
    /// S. K. Shaukat's criterion, as the published code of Reingold and
    /// Dershowitz, *Calendrical Calculations*, carries it under the name
    /// `shaukat-criterion` for their observational Islamic calendar
    /// (`reingold2018code` in `docs/references.bib`, where it was read):
    /// at a solar depression of 4.5°, an arc of light of at least 10.6° and
    /// an altitude of more than 4.1°.
    pub const SHAUKAT: Self = Self::ArcOfLight(ArcOfLightCriterion {
        evaluation_depression_degrees: 4.5,
        minimum_arc_of_light_degrees: 10.6,
        minimum_altitude_degrees: 4.1,
    });

    /// B. D. Yallop's *q*-test, judged at Bruin's best time, visible above
    /// the lower limit of type B: [`QTestCriterion::YALLOP`]. The published
    /// code of *Calendrical Calculations* offers it as
    /// `yallop-criterion`; the width of the crescent is computed here as
    /// Yallop defines it, which differs from that code (see
    /// `docs/systems/hijri.md`).
    pub const YALLOP: Self = Self::QTest(QTestCriterion::YALLOP);
}

/// The arc of light: the true angular separation of the Moon from the Sun,
/// `arccos(cos β · cos φ)` for lunar latitude β and elongation φ, in degrees
/// (`arc-of-light` in the published code of *Calendrical Calculations*).
#[must_use]
pub fn arc_of_light(moment: Moment) -> f64 {
    let elongation = hc_astro::lunar_phase(moment);
    let latitude = hc_astro::lunar::lunar_latitude(moment);
    acos(cos_deg(latitude) * cos_deg(elongation)) * RAD_TO_DEG
}

/// The arc of vision: the geocentric altitude of the Moon's centre less
/// the Sun's, without refraction, in degrees (Yallop, NAO Technical Note
/// 69, §2; `arc-of-vision` in the published code of *Calendrical
/// Calculations*).
#[must_use]
pub fn arc_of_vision(moment: Moment, location: Location) -> f64 {
    lunar_altitude(moment, location) - solar_altitude(moment, location)
}

/// The topocentric width of the crescent, W′, in minutes of arc (Yallop,
/// NAO Technical Note 69, equations 3.8 to 3.10): the semi-diameter
/// `SD = 0.27245 π` from the Moon's horizontal parallax π, made topocentric
/// as `SD′ = SD (1 + sin h sin π)` for the Moon's geocentric altitude h,
/// times `1 − cos ARCL`.
#[must_use]
pub fn crescent_width_arcminutes(moment: Moment, location: Location) -> f64 {
    let parallax = hc_astro::lunar::lunar_parallax(moment);
    let semi_diameter = 0.272_45 * parallax * 60.0;
    let altitude = lunar_altitude(moment, location);
    let topocentric = semi_diameter * (1.0 + sin_deg(altitude) * sin_deg(parallax));
    topocentric * (1.0 - cos_deg(arc_of_light(moment)))
}

/// Bruin's best time for seeing the young crescent on the evening of a
/// local day: `(5 Ts + 4 Tm) / 9`, five ninths of the way from sunset to
/// moonset (Yallop, NAO Technical Note 69, equation 4.1, from Bruin 1977;
/// `bruin-best-view` in the published code of *Calendrical Calculations*).
///
/// `None` when the Sun or the Moon does not set that day, or when the
/// Moon sets before the Sun, in which case there is no crescent in the
/// evening sky to look for. The published code substitutes an arbitrary
/// moment in the first two cases and does not test the third.
#[must_use]
pub fn bruin_best_time(day: Rd, location: Location) -> Option<Moment> {
    let set = sunset(day, location)?;
    let moon = moonset(day, location)?;
    if moon.0 <= set.0 {
        return None;
    }
    Some(Moment((5.0 * set.0 + 4.0 * moon.0) / 9.0))
}

/// Yallop's test parameter `q` at a moment and place
/// ([`QTestCriterion::YALLOP`]).
#[must_use]
pub fn yallop_q(moment: Moment, location: Location) -> f64 {
    QTestCriterion::YALLOP.q(
        arc_of_vision(moment, location),
        crescent_width_arcminutes(moment, location),
    )
}

/// The moment on a local day at which the Sun's centre reaches a depression
/// between the sunset depression and civil dusk, by linear interpolation
/// between the two.
fn depression_moment(rd: Rd, location: Location, depression_degrees: f64) -> Option<Moment> {
    let set = sunset(rd, location)?;
    let civil_dusk = dusk(rd, location, Twilight::Civil)?;
    // The Sun's depression at sunset is not zero: the upper limb is on the
    // horizon, refraction has lifted it, and the horizon itself dips with
    // the observer's height.
    let at_sunset = -sunrise_altitude_degrees(location.elevation_metres);
    let span = Twilight::Civil.depression_degrees() - at_sunset;
    if span <= 0.0 {
        return None;
    }
    let fraction = (depression_degrees - at_sunset) / span;
    Some(Moment(set.0 + fraction * (civil_dusk.0 - set.0)))
}

/// Where the crescent is looked for, and by what standard.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ObservationSite {
    /// The observer's position.
    pub location: Location,
    /// The test applied.
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
    /// judged: when the Sun reaches the criterion's depression, for an
    /// arc-of-light criterion, or at Bruin's best time, for a *q*-test.
    ///
    /// `None` when there is no such moment that day — the Sun does not set
    /// or twilight does not end, as at high latitudes, or for a *q*-test
    /// the Moon does not set after the Sun. That is a real answer: no
    /// observation is possible.
    #[must_use]
    pub fn evaluation_moment(&self, rd: Rd) -> Option<Moment> {
        match self.criterion {
            VisibilityCriterion::ArcOfLight(criterion) => {
                depression_moment(rd, self.location, criterion.evaluation_depression_degrees)
            }
            VisibilityCriterion::QTest(_) => bruin_best_time(rd, self.location),
        }
    }

    /// Whether the crescent should have been visible on the evening that
    /// begins the day `rd`.
    ///
    /// That evening is the dusk of `rd - 1`, because the day begins at
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
        match self.criterion {
            VisibilityCriterion::ArcOfLight(criterion) => {
                let arc = arc_of_light(moment);
                if !(criterion.minimum_arc_of_light_degrees..=90.0).contains(&arc) {
                    return false;
                }
                lunar_altitude(moment, self.location) > criterion.minimum_altitude_degrees
            }
            VisibilityCriterion::QTest(criterion) => {
                criterion.q(
                    arc_of_vision(moment, self.location),
                    crescent_width_arcminutes(moment, self.location),
                ) > criterion.minimum_q
            }
        }
    }

    /// The first day on or after `rd` whose eve carried a visible crescent —
    /// the first day of the next month to begin, or `rd` itself when a
    /// month begins on it (`phasis-on-or-after` in the published code of
    /// *Calendrical Calculations*).
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::AstronomicalModelFailure`] when no visible
    /// crescent is found within a lunation and a half.
    pub fn month_start_on_or_after(&self, rd: Rd) -> CalendarResult<Rd> {
        let moon = floor(hc_astro::new_moon_before(Moment(rd.0 as f64)).0) as i64;
        // Four days past the conjunction, or with the crescent already seen
        // the evening before, this lunation's month has begun and the next
        // one is wanted.
        let first = if rd.0 - moon >= 4 || self.crescent_visible_on_the_eve_of(Rd(rd.0 - 1)) {
            moon + 29
        } else {
            rd.0
        };
        for step in 0..45 {
            let candidate = Rd(first + step);
            if self.crescent_visible_on_the_eve_of(candidate) {
                return Ok(candidate);
            }
        }
        Err(CalendarError::AstronomicalModelFailure)
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

impl Calendar for IslamicObservationalCalendar {
    type Date = IslamicDate;

    /// Unrecorded: this is a prediction of sightings under one criterion,
    /// never a calendar any authority announced, so there is no period in
    /// which it was in force.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::UNRECORDED
    }

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
    /// with a crescent seen after one. It is named by the civil day it ends
    /// on: the crescent is looked for "on eve of" the month's first day, at
    /// the sunset of the civil day before (Reingold and Dershowitz,
    /// `calendar-code2`, `phasis-on-or-before` and `saudi-criterion`).
    fn day_boundary(&self) -> hc_calendar::DayBoundary {
        hc_calendar::DayBoundary::Sunset(hc_calendar::DayNaming::ByEnd)
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
            native_locales: &["ar"],
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
        // Every day in a release build, every eleventh in a debug one.
        for offset in (0..1_500i64).step_by(crate::sweep_stride(11)) {
            let rd = Rd(start.0 + offset);
            let date = calendar.from_fixed(rd).expect("in range");
            assert_eq!(calendar.to_fixed(date), Ok(rd), "RD {rd}");
        }
    }

    #[test]
    fn the_calendar_round_trips_at_both_ends_of_its_range() {
        let calendar = IslamicObservationalCalendar::MECCA;
        for start in [EARLIEST.0 + 40, LATEST.0 - 840] {
            // Every day in a release build, every eleventh in a debug one.
            for offset in (0..800i64).step_by(crate::sweep_stride(11)) {
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

    /// Rows of Yallop's Table 4 (NAO Technical Note 69, 1997): the number,
    /// the evening of the observation, latitude and longitude (east
    /// positive), ARCL, ARCV, the parallax π in minutes of arc, W′ in
    /// minutes of arc, `q`, and the type his test assigns (column BDY).
    /// Evenings only, one or two of each type, 1921 to 1996.
    #[allow(clippy::type_complexity)]
    #[rustfmt::skip]
    const YALLOP_TABLE_4: [(u16, (i64, u8, u8), f64, f64, f64, f64, f64, f64, f64, char); 10] = [
        (284, (1991, 3, 17), 39.0, -76.8, 21.7, 21.7, 58.4, 1.1, 1.617, 'A'),
        (287, (1992, 3, 5), 39.0, -76.8, 16.9, 16.9, 55.2, 0.65, 0.894, 'A'),
        (186, (1987, 4, 28), 30.0, -90.1, 11.9, 11.9, 55.8, 0.33, 0.210, 'B'),
        (91, (1921, 2, 8), 42.3, -71.1, 11.0, 11.0, 54.5, 0.27, 0.081, 'B'),
        (96, (1921, 2, 8), 38.8, -9.1, 9.3, 9.2, 54.4, 0.20, -0.141, 'C'),
        (234, (1988, 6, 14), 37.2, -84.1, 9.3, 9.2, 55.9, 0.20, -0.141, 'C'),
        (241, (1989, 5, 5), 30.3, -97.0, 9.4, 9.0, 60.4, 0.22, -0.146, 'C'),
        (294, (1996, 1, 20), 34.1, -118.3, 8.9, 8.8, 61.2, 0.20, -0.184, 'D'),
        (271, (1984, 9, 25), 15.6, 35.6, 8.4, 8.2, 61.4, 0.18, -0.248, 'E'),
        (256, (1984, 1, 3), 15.6, 35.6, 5.5, 4.2, 55.0, 0.07, -0.720, 'F'),
    ];

    #[test]
    fn yallops_q_follows_from_his_own_arcs_and_widths() {
        // Equation 6.1 alone, fed Yallop's printed ARCV and W′: the printed
        // q to within what rounding ARCV to 0.1° and W′ to two figures
        // allows.
        for (number, _, _, _, _, arcv, _, width, q, _) in YALLOP_TABLE_4 {
            let computed = QTestCriterion::YALLOP.q(arcv, width);
            assert!((computed - q).abs() < 0.012, "No. {number}: {computed}");
        }
        // Yallop's own cut-off for type A: ARCL = 12°, DAZ = 0°, with
        // Bruin's geocentric width W = 15′ (1 − cos ARCL), gives q = +0.216.
        let width = 15.0 * (1.0 - cos_deg(12.0));
        assert!((QTestCriterion::YALLOP.q(12.0, width) - 0.216).abs() < 0.0005);
        // The width is in minutes of arc. Taken in degrees, as the
        // published code of *Calendrical Calculations* computes it, the
        // cubic is 11.8° whatever the crescent, and No. 91, a naked-eye
        // sighting at Boston on 8 February 1921 that Yallop puts in type B,
        // would fail by a wide margin.
        let (_, _, _, _, _, arcv, _, width, _, _) = YALLOP_TABLE_4[3];
        assert!(QTestCriterion::YALLOP.q(arcv, width) > QTestCriterion::YALLOP.minimum_q);
        assert!(QTestCriterion::YALLOP.q(arcv, width / 60.0) < -0.08);
    }

    #[test]
    fn the_q_test_at_bruins_best_time_reproduces_yallops_table_4() {
        // The whole computation from the date and the place: best time,
        // parallax, arcs, width and q, against his table.
        for (
            number,
            (year, month, day),
            latitude,
            longitude,
            arcl,
            arcv,
            parallax,
            width,
            q,
            kind,
        ) in YALLOP_TABLE_4
        {
            let location = Location::new(latitude, longitude, 0.0);
            let evening = civil::to_rd(year, month, day);
            let best = bruin_best_time(evening, location).expect("the Moon sets after the Sun");
            let computed_parallax = hc_astro::lunar::lunar_parallax(best) * 60.0;
            assert!(
                (arc_of_light(best) - arcl).abs() < 0.06,
                "No. {number} ARCL"
            );
            assert!(
                (arc_of_vision(best, location) - arcv).abs() < 0.06,
                "No. {number} ARCV"
            );
            assert!(
                (computed_parallax - parallax).abs() < 0.06,
                "No. {number} π"
            );
            assert!(
                (crescent_width_arcminutes(best, location) - width).abs() < 0.03,
                "No. {number} W′"
            );
            let computed = yallop_q(best, location);
            assert!((computed - q).abs() < 0.005, "No. {number}: q {computed}");
            assert_eq!(
                YallopVisibility::from_q(computed).letter(),
                kind,
                "No. {number}"
            );
            // The criterion counts types A and B as visible, and the
            // evening judged is the eve of the next day.
            let site = ObservationSite::new(location, VisibilityCriterion::YALLOP);
            assert_eq!(
                site.crescent_visible_on_the_eve_of(Rd(evening.0 + 1)),
                kind <= 'B',
                "No. {number}"
            );
        }
    }

    #[test]
    fn bruins_best_time_is_four_ninths_of_the_lag_after_sunset() {
        let evening = civil::to_rd(2024, 3, 11);
        let set = sunset(evening, MECCA).expect("the Sun sets");
        let moon = moonset(evening, MECCA).expect("the Moon sets");
        let best = bruin_best_time(evening, MECCA).expect("after sunset");
        assert!((best.0 - (set.0 + 4.0 / 9.0 * (moon.0 - set.0))).abs() < 1e-9);
        // The evening of the conjunction at Mecca, 10 March 2024: the Moon
        // set thirteen minutes after the Sun, so there is still a best
        // time, and it is under six minutes after sunset.
        let conjunction = civil::to_rd(2024, 3, 10);
        let early = bruin_best_time(conjunction, MECCA).expect("the Moon set after the Sun");
        let set = sunset(conjunction, MECCA).expect("the Sun sets");
        assert!((early.0 - set.0) * 1_440.0 < 6.0);
    }

    #[test]
    fn yallops_types_run_from_a_to_f_at_his_limits() {
        let cases = [
            (1.0, 'A'),
            (0.216, 'B'),
            (0.0, 'B'),
            (-0.014, 'C'),
            (-0.160, 'D'),
            (-0.2, 'D'),
            (-0.232, 'E'),
            (-0.293, 'F'),
            (-1.0, 'F'),
        ];
        for (q, letter) in cases {
            assert_eq!(YallopVisibility::from_q(q).letter(), letter, "q = {q}");
        }
        assert_eq!(QTestCriterion::YALLOP.minimum_q, -0.014);
    }

    #[test]
    fn yallops_test_and_shaukats_disagree_at_mecca_and_the_crate_says_how_often() {
        // The same 552 months as the Umm al-Qura comparison, judged by the
        // two criteria of the published code at the same place.
        let shaukat = IslamicObservationalCalendar::MECCA;
        let yallop = IslamicObservationalCalendar::new(ObservationSite::new(
            MECCA,
            VisibilityCriterion::YALLOP,
        ));
        let mut months = 0u32;
        let mut earlier = 0u32;
        let mut later = 0u32;
        let mut worst = 0i64;
        for year in 1_400..1_446i64 {
            for month in 1..=12u8 {
                let (Ok(by_shaukat), Ok(by_yallop)) = (
                    shaukat.compose(year, month, 1),
                    yallop.compose(year, month, 1),
                ) else {
                    continue;
                };
                months += 1;
                let difference = by_yallop.0 - by_shaukat.0;
                worst = worst.max(difference.abs());
                if difference < 0 {
                    earlier += 1;
                }
                if difference > 0 {
                    later += 1;
                }
            }
        }
        assert_eq!(months, 552);
        assert_eq!(worst, 1, "the two never differ by more than a day");
        // 21 of 552: Yallop's test begins the month a day earlier in 6 and
        // a day later in 15. Stated exactly, as the Umm al-Qura figure is.
        assert_eq!((earlier, later), (6, 15));
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
            VisibilityCriterion::ArcOfLight(ArcOfLightCriterion {
                evaluation_depression_degrees: 4.5,
                minimum_arc_of_light_degrees: 14.0,
                minimum_altitude_degrees: 6.0,
            }),
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
        assert_eq!(IslamicObservationalCalendar::MECCA.site().location, MECCA);
    }
}
