//! Japan's lunisolar calendars before Tenpō-reki, on their own constants.
//!
//! The five Japanese systems are written up in
//! `docs/systems/japanese-lunisolar.md` in the repository: what each was and
//! who computed it, how 恒気, 定朔, 進朔 and the 里差 work with worked
//! examples, every constant with its source, what is carried and what is
//! not, and how the agreement with the published tables was measured. This
//! page summarises it and states the code's own facts.
//!
//! Japan ran eight lunisolar systems in succession between 604 and 1872.
//! [`crate::japanese_tenpo`] is the last. This module is the four that
//! between them cover 862 to 1844 — every Japanese day from the adoption of
//! Senmyō-reki to the eve of the Tenpō reform, the period over which
//! `hc-calendars-regional` dates the eras before Tenpō.
//!
//! | Module | System | In force | 歳実 (days) | 朔実 (days) | 中気 | 進朔 |
//! |---|---|---|---|---|---|---|
//! | [`senmyo`] | 宣明暦 Senmyō-reki | 862–1685 | 365.244643 | 29.530595 | 恒気 | yes |
//! | [`jokyo`] | 貞享暦 Jōkyō-reki | 1685–1755 | 365.241696 | 29.530590 | 恒気 | no |
//! | [`horyaku`] | 宝暦暦 Hōryaku-reki | 1755–1798 | 365.241556 | 29.530590 | 恒気 | no |
//! | [`kansei`] | 寛政暦 Kansei-reki | 1798–1844 | 365.242347 | 29.530584 | 恒気 | no |
//!
//! The ranges abut exactly: the last day of each is the day before the first
//! day of the next, and the last day of [`kansei`] is the day before
//! [`crate::japanese_tenpo::EARLIEST`]. The four systems that ran before 862
//! — 元嘉暦, 儀鳳暦, 大衍暦, 五紀暦 — are **not** here; see "What is missing"
//! below.
//!
//! # The point of the exercise
//!
//! **These calendars must not be run on modern solar theory.** Each was
//! computed from period constants fixed when it was adopted and never
//! revised, and the divergence between those constants and the sky is the
//! historical phenomenon the documents record.
//!
//! Senmyō-reki is the extreme case. Its tropical year, 3068055/8400 =
//! 365.24464 days, is 3.4 minutes too long. Japan kept it for 823 years —
//! far longer than China, which replaced it within seventy — and by the
//! seventeenth century its solar terms stood some two days from the Sun.
//! Shibukawa Harumi's demonstration of that error is why the Jōkyō reform
//! happened.
//!
//! That is not a story this module tells; it is a number it measures.
//! Against the published table, Senmyō-reki on its own 歳実 places the
//! intercalary month correctly in **93.7%** of its 823 years. Compared on a
//! common sample, the same code with modern apparent solar terms manages
//! **66.7%** against its own **92.1%**, and the same 恒気 rule on the modern
//! tropical year manages **64.2%**. The ninth-century constant is worth
//! twenty-five points of agreement, and a test in
//! `tests/japanese_historical.rs` asserts it so the claim cannot quietly stop
//! being true.
//!
//! # Where modern astronomy *is* used, and why
//!
//! The instant of the conjunction, and nothing else. See
//! [`ConjunctionMode::Apparent`]: a pre-modern 定朔 is a table lookup, this
//! crate has the tables for Senmyō-reki alone, and approximating such a table
//! by a single sine reproduces the published month starts about 90% of the
//! time where the true conjunction manages 94% for Senmyō-reki and 99% for
//! the Edo systems. Those bureaux computed conjunctions to within an hour or
//! two. It was their solar theory that was two days out.
//!
//! Each module therefore exposes two parameter sets: [`senmyo::PARAMETERS`],
//! the default, which takes the conjunction from `hc-astro`, and
//! [`senmyo::PARAMETERS_TABULATED`], which takes it from the system's own
//! 日躔 and 月離 amplitudes. Both are measured, and the system document
//! gives both numbers.
//!
//! # The two fitted scalars
//!
//! Everything above is sourced. Two things are not, and are marked as fitted
//! wherever they appear:
//!
//! * **The 暦元 winter solstice.** A system's solstice came from its 上元
//!   積年, an arithmetic chain reaching back millions of years, not from an
//!   observation in the adoption year. That chain is not recoverable from
//!   the sources this crate had, so the phase is fitted: one scalar per
//!   calendar, chosen to maximise agreement with the published tables over
//!   the calendar's whole life. The values are +0.20 days for Senmyō-reki
//!   and −0.30 to −0.50 days for the three Edo systems. That three
//!   independently derived Edo systems all want a solstice a third to half a
//!   day early is itself a finding: they determined it by gnomon shadow, and
//!   this is the size of that method's known bias.
//! * **Senmyō-reki's 進朔限**, for which see [`senmyo::MODEL`].
//!
//! Two scalars fitted against 300 592 days of independent data is
//! calibration rather than curve-fitting, but it is fitting, and it is
//! labelled.
//!
//! # The meridian
//!
//! All four are computed for Kyoto local mean time, UT+9:03:04, the meridian
//! [`crate::japanese_tenpo`] uses. For Jōkyō-reki onward this is simply
//! right: Shibukawa's 里差, the correction from the Chinese capital's
//! meridian to Kyoto's, was one of the Jōkyō reform's headline changes.
//! Senmyō-reki had no such correction — Japan applied the Chinese tables
//! unadjusted for eight centuries — so its true reckoning was Chang'an's,
//! 0.075 days west. That offset is not modelled as a meridian; it is one of
//! the things absorbed into the fitted 進朔限, and it is documented there.
//!
//! # Year numbering
//!
//! As in [`crate::japanese_tenpo`]: the Gregorian year in which the
//! lunisolar year begins. Historical dates were written with a nengō, and
//! nengō belong to `hc-calendars-regional`.
//!
//! # What is missing
//!
//! **元嘉暦 (604–697), 儀鳳暦 (697–764), 大衍暦 (764–862) and 五紀暦
//! (858–862) are not implemented.** Their period constants are recoverable —
//! 元嘉暦 is 日法 752 with 歳実 222070/608 and 朔実 22207/752; 儀鳳暦 and
//! 五紀暦 share 総法 1340 with 489428/1340 and 39571/1340; 大衍暦 is 通法
//! 3040 with 1110343/3040 and 89773/3040 — but the table this crate measures
//! itself against begins in 862, the adoption dates before then are contested
//! by years, 元嘉暦 used 平朔 where the others used 定朔, and their 進朔限
//! differ from system to system and from Senmyō-reki's. Four more calendars
//! that nothing could check would be worse than none, so they are absent
//! rather than approximate.

use hc_calendar::{Calendar, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd};

use crate::civil;
use crate::lunisolar::{
    CHINESE_EPOCH, ConjunctionMode, LunisolarCalendar, LunisolarDate, LunisolarParameters,
    MeanMotionModel, MeridianEra, SolarTermMode,
};

/// The meridian history shared by every Japanese lunisolar calendar.
///
/// Kyoto local mean time throughout; the 1888 row exists only so the table
/// states the whole history, exactly as in [`crate::japanese_tenpo`]. No
/// calendar in this module reaches 1888.
pub static MERIDIANS: [MeridianEra; 2] = [
    MeridianEra::from_longitude(
        i64::MIN / 4,
        135.0 + 46.0 / 60.0,
        "Kyoto local mean time, 135°46′E",
    ),
    MeridianEra::from_zone(1888, 9.0, "Japan Standard Time, 135°E"),
];

/// How far the Japanese year number falls below the continuous Chinese
/// count, as in [`crate::japanese_tenpo::YEAR_OFFSET`].
pub const YEAR_OFFSET: i64 = -2_637;

/// Degrees per day at which the Moon's elongation from the Sun grows, which
/// is what turns a correction in longitude into a correction in time.
const MEAN_ELONGATION_RATE: f64 = 360.0 / 29.530_59;

/// The Sun's contribution to 定朔 for a system whose own 日躔表 is not to
/// hand: the first-order equation of centre, 2·e·sin M with e = 0.0167,
/// giving 1.9148° (Meeus, *Astronomical Algorithms*, ch. 25).
///
/// Senmyō-reki does not use this — its own table is known, and is in
/// [`senmyo::MODEL`]. The three Edo systems do.
const MODERN_SOLAR_EQUATION_DAYS: f64 = 1.9148 / MEAN_ELONGATION_RATE;

/// The Moon's contribution to 定朔 under the same substitution.
///
/// **Not** the Moon's equation of centre, 6.2886°, but that minus the
/// evection, 1.2740°. Evection is 1.2740°·sin(2D − M′) and a conjunction is
/// where D = 0, so at exactly the moment this correction is evaluated it
/// collapses to −1.2740°·sin(M′) and simply reduces the first harmonic; the
/// variation, 0.6583°·sin(2D), vanishes there outright. (Meeus, ch. 47.)
///
/// This is why a one-term conjunction model does better than its description
/// suggests, and it is what Senmyō-reki's own 月離 table was measuring:
/// 3225/8400 of a day is 5.13° at the Moon's mean daily motion, against 5.01°
/// for 6.2886 − 1.2740.
const MODERN_LUNAR_EQUATION_DAYS: f64 = (6.2886 - 1.2740) / MEAN_ELONGATION_RATE;

/// A calendar type delegating wholesale to a configured engine.
///
/// Four systems differ in nothing but their parameters, and four hand-typed
/// copies of the same five forwarding methods would be the duplication the
/// rest of this crate exists to avoid.
macro_rules! delegating_calendar {
    ($(#[$doc:meta])* $name:ident) => {
        $(#[$doc])*
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
        pub struct $name;

        impl Calendar for $name {
            type Date = LunisolarDate;

            /// In force from the day the system was adopted to the day
            /// before its successor's, which is also the range it converts.
            fn usage(&self) -> hc_calendar::Usage {
                hc_calendar::Usage::between(EARLIEST, LATEST, USAGE_SOURCE)
            }

            fn meta(&self) -> CalendarMeta {
                ENGINE.meta()
            }

            fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
                ENGINE.cycles()
            }

            fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
                PARAMETERS.is_leap_year(year)
            }

            fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
                ENGINE.to_fixed(date)
            }

            fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
                ENGINE.from_fixed(rd)
            }

            fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
                ENGINE.to_fields(date)
            }

            fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
                ENGINE.from_fields(fields)
            }
        }

        /// The fixed day of the lunisolar new year of `year`.
        ///
        /// # Errors
        ///
        /// Returns [`hc_calendar::CalendarError::YearOutOfRange`] outside the
        /// supported range.
        pub fn new_year(year: i64) -> CalendarResult<Rd> {
            PARAMETERS.new_year(year)
        }
    };
}

/// The two parameter sets and the two engines every system here exposes,
/// given its `MODEL` and `MODEL_TABULATED`.
macro_rules! parameter_sets {
    ($english:literal) => {
        /// The parameters, as a `const` so that the variant below can be
        /// spelled with struct update syntax.
        const TEMPLATE: LunisolarParameters = LunisolarParameters {
            id: ID,
            english_name: $english,
            meridians: &MERIDIANS,
            epoch: CHINESE_EPOCH,
            year_offset: YEAR_OFFSET,
            solar_term_mode: SolarTermMode::Mean,
            mean_motion: Some(MODEL),
            earliest: Some(EARLIEST),
            latest: Some(LATEST),
        };

        /// The parameters of this calendar, with the conjunction taken from
        /// modern astronomy and everything else from the system itself.
        pub static PARAMETERS: LunisolarParameters = TEMPLATE;

        /// The engine configured as this calendar.
        pub const ENGINE: LunisolarCalendar = LunisolarCalendar::new(&PARAMETERS);

        /// The same calendar with the conjunction taken from the system's own
        /// 日躔 and 月離 amplitudes instead.
        ///
        /// Historically the purer reconstruction and empirically the less
        /// accurate one, for the reason set out in
        /// [`ConjunctionMode::Apparent`]. It is kept, exported and measured
        /// rather than deleted, because the gap between the two is the most
        /// informative number this module produces.
        pub static PARAMETERS_TABULATED: LunisolarParameters = LunisolarParameters {
            mean_motion: Some(MODEL_TABULATED),
            ..TEMPLATE
        };

        /// The engine configured with the system's own conjunction tables.
        pub const ENGINE_TABULATED: LunisolarCalendar =
            LunisolarCalendar::new(&PARAMETERS_TABULATED);
    };
}

pub mod senmyo {
    //! 宣明暦 Senmyō-reki, 862–1685 — the longest-serving calendar in
    //! Japanese history.
    //!
    //! The system is written up in `docs/systems/japanese-lunisolar.md`,
    //! which reads its constants and its 進朔 rule from 新唐書 and the NAO
    //! 暦Wiki, works the intercalary month of 1200 by hand, and measures it
    //! against the published table. This page states the constants and the
    //! range.
    //!
    //! Xu Ang's 長慶宣明暦 of 822, adopted in Japan on 貞観4年1月1日 and kept
    //! for **823 years**, through the whole of the Heian, Kamakura, Muromachi
    //! and early Edo periods. China had moved on within seventy years; Japan
    //! did not, because after the Ōnin War no body at court retained the
    //! competence to compute a new one.
    //!
    //! # Constants
    //!
    //! The system divides the day into 統法 = 8400 parts and states its
    //! periods as whole numbers of them:
    //!
    //! | | Parts | Days |
    //! |---|---|---|
    //! | 章歳 (tropical year) | 3 068 055 | 365.244643 |
    //! | 章月 (synodic month) | 248 057 | 29.530595 |
    //! | 曆周 (anomalistic month) | 231 458.19 | 27.554546 |
    //! | 日躔 peak 朓朒 | 1 526 | 0.181667 |
    //! | 月離 peak 朓朒 | 3 225 | 0.383929 |
    //! | 進朔限 | 6 300 | 0.75 |
    //!
    //! The first three and the 進朔限 come from 国立天文台暦計算室, 暦Wiki
    //! 「宣明暦」 (<https://eco.mtk.nao.ac.jp/koyomi/wiki/C0EBCCC0CEF1.html>)
    //! and from the 長慶宣明曆 chapter of 新唐書 卷030上. **The two 朓朒 peaks
    //! are a reading of the 日躔 and 月離 tables in that chapter and are
    //! quoted literally by no secondary source this crate could reach.** The
    //! lunar one is corroborated indirectly: 暦Wiki「太陰太陽暦/月離」 models
    //! the encoded correction as (6.29° − 1.27°)·sin l over the Moon's daily
    //! motion, which peaks at 0.38 days.
    //!
    //! # The drift, which is the point
    //!
    //! 365.244643 exceeds the December-solstice year of the period by about
    //! 0.0018 days. Over 823 years that is a day and a half of accumulated
    //! error in the solar terms, on top of whatever the Chinese original
    //! already carried in 862 — the contemporary complaint 「天行二日を違う」.
    //! The synodic month is wrong by seven parts in a million, so the
    //! *months* stayed nearly right while the *seasons* slid. That asymmetry
    //! is exactly what this model reproduces, and it is why the reform of
    //! 1685 was about solar terms and not about new moons.
    //!
    //! # Range
    //!
    //! 貞観4年1月1日 = **862-02-03 Julian** (proleptic Gregorian 862-02-07) to
    //! 貞享元年12月30日 = **1685-02-03 Gregorian**, the day before Jōkyō-reki
    //! took effect.

    use super::*;

    /// The machine identifier this crate uses. CLDR has none.
    pub const ID: CalendarId = CalendarId("japanese-senmyo");

    /// 貞観4年1月1日, Julian 862-02-03, proleptic Gregorian 862-02-07.
    pub const EARLIEST: Rd = civil::to_rd(862, 2, 7);

    /// 貞享元年12月30日, Gregorian 1685-02-03 — the last day before the Jōkyō
    /// reform.
    pub const LATEST: Rd = civil::to_rd(1685, 2, 3);

    /// Where the period of use comes from.
    pub const USAGE_SOURCE: &str = "貞観4年1月1日 = 862-02-03 Julian, 862-02-07 proleptic Gregorian [wikipedia-ja-senmyo, nao-rekiwiki-senmyo], \\
        to 貞享元年12月30日 = 1685-02-03, the day before Jōkyō-reki [wikipedia-ja-jokyo, nao-rekiwiki-jokyo]";

    /// The period constants of Senmyō-reki.
    ///
    /// The conjunction epoch is the conjunction of the adoption day read at
    /// Kyoto; it serves only to number the months, since this model takes the
    /// conjunction itself from [`ConjunctionMode::Apparent`]. The solstice
    /// epoch is the true December solstice of 861 **plus 0.20 days**, and
    /// that 0.20 is fitted — see the module documentation.
    pub const MODEL: MeanMotionModel = MeanMotionModel {
        // 章歳 3_068_055 / 統法 8_400
        tropical_year: 3_068_055.0 / 8_400.0,
        // 章月 248_057 / 統法 8_400
        synodic_month: 248_057.0 / 8_400.0,
        // 曆周 231_458.19 / 8_400, written with 秒母 100
        anomalistic_month: 23_145_819.0 / 840_000.0,
        // The December solstice of 861 at Kyoto is 314_464.358_529; this is
        // that plus the fitted 0.20-day phase of the system's 暦元 冬至.
        solstice_epoch: 314_464.558_529,
        conjunction_epoch: 314_512.395_403,
        perigee_epoch: 314_530.997_910,
        conjunction_mode: ConjunctionMode::Apparent,
        solar_equation_days: 1_526.0 / 8_400.0,
        lunar_equation_days: 3_225.0 / 8_400.0,
        // 進朔限 is 6300/8400 = 0.75 of a day in the system's own reckoning,
        // and 0.75 is what the sources give. Measured against the published
        // Japanese month starts with the conjunction read at Kyoto, the limit
        // that reproduces them is 0.80; the extra 0.05 days is Senmyō-reki's
        // conjunctions running early against Kyoto true time, because Japan
        // applied the Chinese tables with no 里差 at all and Chang'an is
        // 0.075 days west. The sourced 0.75 with a Chang'an meridian gives
        // 95.6% day agreement against this arrangement's 96.4%; both are
        // measured in the integration tests, so the trade is visible rather
        // than asserted.
        advance_limit: Some(0.80),
    };

    /// Senmyō-reki with its own 日躔 and 月離 tables driving the conjunction.
    pub const MODEL_TABULATED: MeanMotionModel = MeanMotionModel {
        conjunction_mode: ConjunctionMode::True,
        ..MODEL
    };

    parameter_sets!("Japanese Senmyō (lunisolar)");

    delegating_calendar! {
        /// 宣明暦, the calendar of Japan from 862 to 1685.
        SenmyoCalendar
    }
}

pub mod jokyo {
    //! 貞享暦 Jōkyō-reki, 1685–1755 — the first calendar computed in Japan.
    //!
    //! The system is written up in `docs/systems/japanese-lunisolar.md`,
    //! which works 貞享2年1月1日 = 1685-02-04 by hand from this model's
    //! constants and names the sources for them. This page states the
    //! constants and the range.
    //!
    //! Shibukawa Harumi (渋川春海) spent twenty years demonstrating that
    //! Senmyō-reki's solar terms were two days wrong, and in 1684 his third
    //! proposal was accepted. Jōkyō-reki is a reworking of the Yuan 授時暦
    //! with two changes that matter: fresh period constants, and a 里差 — a
    //! correction from the Chinese capital's meridian to Kyoto's. He also
    //! abolished 進朔, calling it groundless.
    //!
    //! # Constants
    //!
    //! | | Days |
    //! |---|---|
    //! | 恒星年 (sidereal year) | 365.256696 |
    //! | 歳実 (tropical year) | 365.241696 |
    //! | 朔策 (synodic month) | 29.530590 |
    //! | 近点月 | 27.554600 |
    //!
    //! Source: 国立天文台暦計算室, 暦Wiki「貞享暦」
    //! (<https://eco.mtk.nao.ac.jp/koyomi/wiki/C4E7B5FDCEF1.html>). The
    //! difference between the sidereal and the tropical figure, exactly 0.015
    //! days, is the system's constant of precession.
    //!
    //! # Range
    //!
    //! 貞享2年1月1日 = **1685-02-04** to 宝暦4年12月30日 = **1755-02-10**,
    //! both Gregorian.

    use super::*;

    /// The machine identifier this crate uses. CLDR has none.
    pub const ID: CalendarId = CalendarId("japanese-jokyo");

    /// 貞享2年1月1日, Gregorian 1685-02-04 — the first day of the first
    /// calendar computed in Japan.
    pub const EARLIEST: Rd = civil::to_rd(1685, 2, 4);

    /// 宝暦4年12月30日, Gregorian 1755-02-10.
    pub const LATEST: Rd = civil::to_rd(1755, 2, 10);

    /// Where the period of use comes from.
    pub const USAGE_SOURCE: &str = "貞享2年1月1日 = 1685-02-04, by imperial proclamation [wikipedia-ja-jokyo, nao-rekiwiki-jokyo], to \\
        宝暦4年12月30日 = 1755-02-10, the day before Hōryaku-reki [nao-rekiwiki-horyaku, wikipedia-ja-horyaku]";

    /// The period constants of Jōkyō-reki.
    ///
    /// The solstice epoch is the true December solstice of 1684 at Kyoto
    /// **minus 0.40 days**, and that 0.40 is fitted. All three Edo systems
    /// want a solstice a third to half a day early, which is the size of the
    /// known bias of the gnomon-shadow method they determined it by.
    pub const MODEL: MeanMotionModel = MeanMotionModel {
        tropical_year: 365.241_696,
        synodic_month: 29.530_590,
        anomalistic_month: 27.554_600,
        // True solstice of 1684 at Kyoto, 615_059.228_909, less the fitted
        // 0.40-day phase.
        solstice_epoch: 615_058.828_909,
        conjunction_epoch: 615_104.433_935,
        perigee_epoch: 615_123.881_278,
        conjunction_mode: ConjunctionMode::Apparent,
        solar_equation_days: MODERN_SOLAR_EQUATION_DAYS,
        lunar_equation_days: MODERN_LUNAR_EQUATION_DAYS,
        // Shibukawa abolished 進朔; no calendar after Senmyō-reki holds a
        // conjunction over to the following day.
        advance_limit: None,
    };

    /// Jōkyō-reki with a tabulated conjunction rather than an apparent one.
    pub const MODEL_TABULATED: MeanMotionModel = MeanMotionModel {
        conjunction_mode: ConjunctionMode::True,
        ..MODEL
    };

    parameter_sets!("Japanese Jōkyō (lunisolar)");

    delegating_calendar! {
        /// 貞享暦, the calendar of Japan from 1685 to 1755.
        JokyoCalendar
    }
}

pub mod horyaku {
    //! 宝暦暦 Hōryaku-reki, 1755–1798 — the reform that went backwards.
    //!
    //! The system is written up in `docs/systems/japanese-lunisolar.md`,
    //! with the sources for its constants and for the 1771 revision this
    //! module does not carry. This page states the constants and the range.
    //!
    //! Produced by the court in Kyoto rather than by the shogunate's
    //! astronomers, and generally judged worse than the calendar it replaced:
    //! it failed to predict the solar eclipse of 1763, which the amateur
    //! Asada Gōryū did predict, and was patched in 1771 (修正宝暦暦). This
    //! module implements the system as promulgated in 1755, with the tropical
    //! year 365.241556; the 1771 revision moved it to 365.241626, a change of
    //! six seconds a year and four minutes over the calendar's whole life,
    //! far below the resolution of a day boundary.
    //!
    //! # Constants
    //!
    //! | | Days |
    //! |---|---|
    //! | 恒星年 | 365.256556 |
    //! | 歳実 (tropical year) | 365.241556 |
    //! | 朔策 (synodic month) | 29.530590 |
    //! | 近点月 | 27.554600 |
    //!
    //! Source: 国立天文台暦計算室, 暦Wiki「宝暦暦」
    //! (<https://eco.mtk.nao.ac.jp/koyomi/wiki/CAF5CEF1CEF1.html>).
    //!
    //! # Range
    //!
    //! 宝暦5年1月1日 = **1755-02-11** to 寛政9年12月30日 = **1798-02-15**.

    use super::*;

    /// The machine identifier this crate uses. CLDR has none.
    pub const ID: CalendarId = CalendarId("japanese-horyaku");

    /// 宝暦5年1月1日, Gregorian 1755-02-11.
    pub const EARLIEST: Rd = civil::to_rd(1755, 2, 11);

    /// 寛政9年12月30日, Gregorian 1798-02-15.
    pub const LATEST: Rd = civil::to_rd(1798, 2, 15);

    /// Where the period of use comes from.
    pub const USAGE_SOURCE: &str = "宝暦5年1月1日 = 1755-02-11 [nao-rekiwiki-horyaku, wikipedia-ja-horyaku] to 寛政9年12月30日 = \\
        1798-02-15, the day before Kansei-reki [nao-rekiwiki-kansei, wikipedia-ja-kansei]";

    /// The period constants of Hōryaku-reki as promulgated.
    ///
    /// The solstice epoch is the true December solstice of 1754 at Kyoto
    /// **minus 0.50 days**, fitted as in [`super::jokyo::MODEL`].
    pub const MODEL: MeanMotionModel = MeanMotionModel {
        tropical_year: 365.241_556,
        synodic_month: 29.530_590,
        anomalistic_month: 27.554_600,
        // True solstice of 1754 at Kyoto, 640_626.218_005, less 0.50.
        solstice_epoch: 640_625.718_005,
        conjunction_epoch: 640_677.786_600,
        perigee_epoch: 640_694.627_012,
        conjunction_mode: ConjunctionMode::Apparent,
        solar_equation_days: MODERN_SOLAR_EQUATION_DAYS,
        lunar_equation_days: MODERN_LUNAR_EQUATION_DAYS,
        advance_limit: None,
    };

    /// Hōryaku-reki with a tabulated conjunction rather than an apparent one.
    pub const MODEL_TABULATED: MeanMotionModel = MeanMotionModel {
        conjunction_mode: ConjunctionMode::True,
        ..MODEL
    };

    parameter_sets!("Japanese Hōryaku (lunisolar)");

    delegating_calendar! {
        /// 宝暦暦, the calendar of Japan from 1755 to 1798.
        HoryakuCalendar
    }
}

pub mod kansei {
    //! 寛政暦 Kansei-reki, 1798–1844 — Japanese calendrics meets Kepler.
    //!
    //! The system is written up in `docs/systems/japanese-lunisolar.md`,
    //! which names the sources for its constants and how the agreement was
    //! measured. This page states the constants and the range.
    //!
    //! Takahashi Yoshitoki (高橋至時) and Hazama Shigetomi built it on
    //! 暦象考成後編, the Chinese translation of Western tables deriving from
    //! Lalande and, behind him, from Kepler. It is the first Japanese
    //! calendar in which the Sun and the Moon move on ellipses rather than on
    //! tabulated increments, and its tropical year, 365.242347 days, is within
    //! eleven seconds of the truth — a hundred times better than
    //! Senmyō-reki's.
    //!
    //! # Constants
    //!
    //! | | Days |
    //! |---|---|
    //! | 歳実 (tropical year) | 365.242347 |
    //! | 朔策 (synodic month) | 29.530584 |
    //! | 近点月 (anomalistic month) | 27.554570 |
    //!
    //! Source: 国立天文台暦計算室, 暦Wiki「寛政暦」
    //! (<https://eco.mtk.nao.ac.jp/koyomi/wiki/B4B2C0AFCEF1.html>,
    //! `nao-rekiwiki-kansei` in `docs/references.bib`), which gives the year
    //! as 365.242347071 and derives the two months from the daily mean
    //! motions: 29.530584 = 360 / (13.1763981114 − 0.9856469352) and
    //! 27.554570 = 360 / (13.1763981114 − 0.1114147178), the Moon's mean
    //! motion less the Sun's and less the apogee's. An earlier revision of
    //! this model used the modern anomalistic month, 27.554550, saying the
    //! page gave none; the page does, and the model now carries the system's
    //! own value, as the crate's policy for these calendars requires. The
    //! change is 2 × 10⁻⁵ days in a quantity that enters only as the phase
    //! of a sine, and re-measuring left every agreement figure in
    //! `tests/japanese_historical.rs` unchanged to two decimals.
    //!
    //! It kept the 恒気 rule for the major solar terms, because Kansei-reki
    //! did; the move to 定気 was Tenpō-reki's innovation six years later, and
    //! is the reason Tenpō-reki gets a module of its own.
    //!
    //! # Range
    //!
    //! 寛政10年1月1日 = **1798-02-16** to 天保14年12月29日 = **1844-02-17**,
    //! the day before [`crate::japanese_tenpo::EARLIEST`].

    use super::*;

    /// The machine identifier this crate uses. CLDR has none.
    pub const ID: CalendarId = CalendarId("japanese-kansei");

    /// 寛政10年1月1日, Gregorian 1798-02-16.
    pub const EARLIEST: Rd = civil::to_rd(1798, 2, 16);

    /// 天保14年12月29日, Gregorian 1844-02-17.
    pub const LATEST: Rd = civil::to_rd(1844, 2, 17);

    /// Where the period of use comes from.
    pub const USAGE_SOURCE: &str = "寛政10年1月1日 = 1798-02-16 [nao-rekiwiki-kansei, wikipedia-ja-kansei] to 天保14年12月29日 = \\
        1844-02-17, the day before Tenpō-reki [nao-rekiwiki-tenpo, wikipedia-ja-tenpo]";

    /// The period constants of Kansei-reki.
    ///
    /// The solstice epoch is the true December solstice of 1797 at Kyoto
    /// **minus 0.30 days**, fitted as in [`super::jokyo::MODEL`].
    pub const MODEL: MeanMotionModel = MeanMotionModel {
        tropical_year: 365.242_347,
        synodic_month: 29.530_584,
        // 360 / (13.1763981114 − 0.1114147178), as the NAO page derives it.
        anomalistic_month: 27.554_570,
        // True solstice of 1797 at Kyoto, 656_331.661_712, less 0.30.
        solstice_epoch: 656_331.361_712,
        conjunction_epoch: 656_388.155_468,
        perigee_epoch: 656_402.566_245,
        conjunction_mode: ConjunctionMode::Apparent,
        solar_equation_days: MODERN_SOLAR_EQUATION_DAYS,
        lunar_equation_days: MODERN_LUNAR_EQUATION_DAYS,
        advance_limit: None,
    };

    /// Kansei-reki with a tabulated conjunction rather than an apparent one.
    ///
    /// The substitution costs more here than anywhere else in the module, for
    /// an unexpected reason: Kansei-reki solved Kepler's equation, so a single
    /// sine reproduces *it* worse than it reproduces the ninth-century system
    /// it is nine hundred years younger than.
    pub const MODEL_TABULATED: MeanMotionModel = MeanMotionModel {
        conjunction_mode: ConjunctionMode::True,
        ..MODEL
    };

    parameter_sets!("Japanese Kansei (lunisolar)");

    delegating_calendar! {
        /// 寛政暦, the calendar of Japan from 1798 to 1844.
        KanseiCalendar
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hc_calendar::{CalendarError, Month};

    /// The four systems in force order, with the identifier each registers
    /// under and the period it covers.
    fn systems() -> [(&'static str, &'static LunisolarParameters); 4] {
        [
            ("senmyo", &senmyo::PARAMETERS),
            ("jokyo", &jokyo::PARAMETERS),
            ("horyaku", &horyaku::PARAMETERS),
            ("kansei", &kansei::PARAMETERS),
        ]
    }

    fn model_of(parameters: &LunisolarParameters) -> MeanMotionModel {
        parameters
            .mean_motion
            .expect("every historical system carries its own constants")
    }

    #[test]
    fn senmyo_states_its_constants_as_the_eight_thousand_four_hundredths_it_used() {
        // 統法 8400 with 章歳 3068055 and 朔実 248057, from the NAO 暦Wiki
        // entry cited in the module documentation.
        let model = senmyo::MODEL;
        assert!((model.tropical_year - 3_068_055.0 / 8_400.0).abs() < 1e-12);
        assert!((model.synodic_month - 248_057.0 / 8_400.0).abs() < 1e-12);
        // The decimals the sources quote.
        assert!((model.tropical_year - 365.244_64).abs() < 5e-6);
        assert!((model.synodic_month - 29.530_595).abs() < 5e-7);
        // 曆周 231458.19/8400, the system's own anomalistic month.
        assert!((model.anomalistic_month - 27.554_546).abs() < 1e-6);
    }

    #[test]
    fn each_reform_moved_the_tropical_year_closer_to_the_truth() {
        // Not monotonically *shorter*: Senmyō-reki's year was far too long
        // and Jōkyō-reki and Hōryaku-reki overshot slightly short, so
        // Kansei-reki had to lengthen it again. Monotonically *closer* is
        // the claim the history supports, and this is it measured against
        // the mean tropical year of the period, 365.2422 days.
        let error = |model: MeanMotionModel| (model.tropical_year - 365.2422).abs() * 86_400.0;
        let senmyo = error(senmyo::MODEL);
        let jokyo = error(jokyo::MODEL);
        let horyaku = error(horyaku::MODEL);
        let kansei = error(kansei::MODEL);
        assert!(senmyo > jokyo, "{senmyo} s vs {jokyo} s");
        assert!(jokyo > kansei, "{jokyo} s vs {kansei} s");
        assert!(
            kansei < 20.0,
            "Kansei-reki is out by {kansei} seconds a year"
        );
        assert!(
            senmyo > 180.0,
            "Senmyō-reki is out by {senmyo} seconds a year"
        );
        // Hōryaku-reki, the reform widely judged a step backwards, did move
        // its year further from the truth than the calendar it replaced.
        assert!(horyaku > jokyo, "{horyaku} s vs {jokyo} s");
        // Kansei-reki is the only one of the four that is too long rather
        // than too short, having corrected an overshoot.
        const {
            assert!(kansei::MODEL.tropical_year > jokyo::MODEL.tropical_year);
            assert!(senmyo::MODEL.tropical_year > kansei::MODEL.tropical_year);
        }
    }

    #[test]
    fn every_system_uses_the_mean_solar_terms_that_were_in_force_before_1844() {
        for (name, parameters) in systems() {
            assert_eq!(
                parameters.solar_term_mode,
                SolarTermMode::Mean,
                "{name} must use 恒気; 定気 was Tenpō-reki's innovation"
            );
        }
        assert_eq!(
            crate::japanese_tenpo::PARAMETERS.solar_term_mode,
            SolarTermMode::Apparent
        );
    }

    #[test]
    fn every_system_computes_a_true_conjunction_one_way_or_the_other() {
        // 定朔 reached Japan with Gihō-reki in 697, so no system here uses
        // the mean conjunction. Which of the two true reconstructions is the
        // default is a measured choice, not a historical one.
        for (name, parameters) in systems() {
            assert_eq!(
                model_of(parameters).conjunction_mode,
                ConjunctionMode::Apparent,
                "{name}"
            );
        }
        for model in [
            senmyo::MODEL_TABULATED,
            jokyo::MODEL_TABULATED,
            horyaku::MODEL_TABULATED,
            kansei::MODEL_TABULATED,
        ] {
            assert_eq!(model.conjunction_mode, ConjunctionMode::True);
        }
    }

    #[test]
    fn senmyo_states_its_conjunction_tables_in_eight_thousand_four_hundredths() {
        // The 日躔 and 月離 peaks, and the 進朔限 they are used with.
        let model = senmyo::MODEL;
        assert!((model.solar_equation_days - 1_526.0 / 8_400.0).abs() < 1e-12);
        assert!((model.lunar_equation_days - 3_225.0 / 8_400.0).abs() < 1e-12);
        // The lunar peak, read back as an angle at the Moon's mean daily
        // motion, is the 5.13° the sources describe.
        let degrees = model.lunar_equation_days * 13.369;
        assert!((degrees - 5.13).abs() < 0.02, "{degrees} degrees");
        // The Edo systems use the substituted amplitudes instead.
        for parameters in [
            &jokyo::PARAMETERS,
            &horyaku::PARAMETERS,
            &kansei::PARAMETERS,
        ] {
            let model = model_of(parameters);
            assert!((model.solar_equation_days - MODERN_SOLAR_EQUATION_DAYS).abs() < 1e-12);
            assert!((model.lunar_equation_days - MODERN_LUNAR_EQUATION_DAYS).abs() < 1e-12);
        }
    }

    #[test]
    fn no_historical_system_consults_modern_astronomy() {
        // The guarantee the module exists to make: a calendar with its own
        // constants never falls through to hc-astro.
        for (name, parameters) in systems() {
            assert!(parameters.mean_motion.is_some(), "{name}");
        }
        assert!(crate::japanese_tenpo::PARAMETERS.mean_motion.is_none());
        assert!(crate::chinese::PARAMETERS.mean_motion.is_none());
    }

    #[test]
    fn the_four_systems_tile_the_years_862_to_1844_without_a_gap() {
        let mut previous: Option<Rd> = None;
        for (name, parameters) in systems() {
            let meta = parameters.meta();
            let first = meta.earliest.expect("bounded");
            if let Some(last) = previous {
                assert_eq!(first.0, last.0 + 1, "{name} does not abut its predecessor");
            }
            previous = Some(meta.latest.expect("bounded"));
        }
        // And Kansei-reki hands over to Tenpō-reki on the next day.
        assert_eq!(
            previous.map(|last| last.0 + 1),
            Some(crate::japanese_tenpo::EARLIEST.0)
        );
    }

    #[test]
    fn senmyo_took_effect_on_the_third_of_february_862_in_the_julian_calendar() {
        // 貞観4年1月1日. The Julian date is 862-02-03; this crate's civil
        // helper is proleptic Gregorian, in which the same day is 862-02-07.
        assert_eq!(civil::from_rd(senmyo::EARLIEST), (862, 2, 7));
        assert_eq!(senmyo::EARLIEST.0, 314_512);
        assert_eq!(senmyo::new_year(862), Ok(senmyo::EARLIEST));
        assert_eq!(
            senmyo::SenmyoCalendar.from_fixed(senmyo::EARLIEST),
            Ok(LunisolarDate::new(862, Month::regular(1), 1))
        );
    }

    #[test]
    fn senmyo_names_the_day_of_the_honnoji_incident() {
        // 天正10年6月2日, Julian 1582-06-21 — nineteen days before the
        // Gregorian reform, so proleptic Gregorian 1582-07-01.
        let rd = civil::to_rd(1582, 7, 1);
        assert_eq!(
            senmyo::SenmyoCalendar.from_fixed(rd),
            Ok(LunisolarDate::new(1_582, Month::regular(6), 2))
        );
        assert_eq!(
            senmyo::SenmyoCalendar.to_fixed(LunisolarDate::new(1_582, Month::regular(6), 2)),
            Ok(rd)
        );
    }

    #[test]
    fn senmyo_names_the_day_of_sekigahara() {
        // 慶長5年9月15日 = 1600-10-21 Gregorian (1600-10-11 Julian).
        assert_eq!(
            senmyo::SenmyoCalendar.from_fixed(civil::to_rd(1600, 10, 21)),
            Ok(LunisolarDate::new(1_600, Month::regular(9), 15))
        );
    }

    #[test]
    fn senmyo_ends_on_the_day_before_the_jokyo_reform() {
        // 貞享元年12月30日 = 1685-02-03, and the next day is 貞享2年1月1日.
        assert_eq!(civil::from_rd(senmyo::LATEST), (1685, 2, 3));
        assert_eq!(
            senmyo::SenmyoCalendar.from_fixed(senmyo::LATEST),
            Ok(LunisolarDate::new(1_684, Month::regular(12), 30))
        );
        assert_eq!(
            senmyo::SenmyoCalendar.from_fixed(Rd(senmyo::LATEST.0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
    }

    #[test]
    fn senmyo_ran_for_eight_hundred_and_twenty_three_years() {
        let days = senmyo::LATEST.0 - senmyo::EARLIEST.0 + 1;
        // 823 years of about 365.2422 days.
        assert_eq!(days, 300_592);
        let years = days as f64 / 365.2422;
        assert!((years - 823.0).abs() < 1.0, "{years} years");
    }

    #[test]
    fn jokyo_took_effect_on_the_fourth_of_february_1685() {
        assert_eq!(civil::from_rd(jokyo::EARLIEST), (1685, 2, 4));
        assert_eq!(jokyo::new_year(1_685), Ok(jokyo::EARLIEST));
        assert_eq!(
            jokyo::JokyoCalendar.from_fixed(jokyo::EARLIEST),
            Ok(LunisolarDate::new(1_685, Month::regular(1), 1))
        );
    }

    #[test]
    fn jokyo_names_the_night_of_the_forty_seven_ronin() {
        // 元禄15年12月14日 = 1703-01-30 Gregorian. The lunisolar year had
        // begun in 1702, so this crate numbers it 1702.
        assert_eq!(
            jokyo::JokyoCalendar.from_fixed(civil::to_rd(1703, 1, 30)),
            Ok(LunisolarDate::new(1_702, Month::regular(12), 14))
        );
    }

    #[test]
    fn horyaku_took_effect_on_the_eleventh_of_february_1755() {
        assert_eq!(civil::from_rd(horyaku::EARLIEST), (1755, 2, 11));
        assert_eq!(horyaku::new_year(1_755), Ok(horyaku::EARLIEST));
        assert_eq!(
            horyaku::HoryakuCalendar.from_fixed(horyaku::EARLIEST),
            Ok(LunisolarDate::new(1_755, Month::regular(1), 1))
        );
        // The day before belongs to Jōkyō-reki: 宝暦4年12月30日.
        assert_eq!(
            jokyo::JokyoCalendar.from_fixed(Rd(horyaku::EARLIEST.0 - 1)),
            Ok(LunisolarDate::new(1_754, Month::regular(12), 30))
        );
    }

    #[test]
    fn kansei_took_effect_on_the_sixteenth_of_february_1798() {
        assert_eq!(civil::from_rd(kansei::EARLIEST), (1798, 2, 16));
        assert_eq!(kansei::new_year(1_798), Ok(kansei::EARLIEST));
        assert_eq!(
            kansei::KanseiCalendar.from_fixed(kansei::EARLIEST),
            Ok(LunisolarDate::new(1_798, Month::regular(1), 1))
        );
    }

    #[test]
    fn kansei_ends_on_the_day_before_tenpo_takes_over() {
        // 天保14年12月29日 = 1844-02-17, and 天保15年1月1日 = 1844-02-18.
        assert_eq!(civil::from_rd(kansei::LATEST), (1844, 2, 17));
        assert_eq!(
            kansei::KanseiCalendar.from_fixed(kansei::LATEST),
            Ok(LunisolarDate::new(1_843, Month::regular(12), 29))
        );
        assert_eq!(
            crate::JapaneseTenpoCalendar.from_fixed(Rd(kansei::LATEST.0 + 1)),
            Ok(LunisolarDate::new(1_844, Month::regular(1), 1))
        );
    }

    #[test]
    fn senmyos_solar_terms_drift_a_day_and_a_half_away_from_the_sun() {
        // The reason for the Jōkyō reform, measured. Senmyō-reki's year is
        // 0.0018 days longer than the December-solstice year of the period,
        // so its 冬至 slides later and later; by the 1680s it stands most of
        // a day and a half past the true solstice. Contemporaries put the
        // total error at 「天行二日を違う」 — two days — which includes the
        // error the Chinese original already carried in 862 and which this
        // model, phased on the sky of 862, does not reproduce.
        let model = senmyo::MODEL;
        let kyoto = MERIDIANS[0].offset_hours / 24.0;
        let drift = |year: i64| {
            let truth = hc_astro::solstice(year, hc_astro::solar::Solstice::December).0 + kyoto;
            let index = ((truth - model.solstice_epoch) / model.tropical_year).round() as i64;
            model.winter_solstice(index) - truth
        };
        // At the epoch the model sits on its fitted 0.20-day phase, and
        // everything after that is the constant's own excess accumulating.
        assert!((0.15..0.25).contains(&drift(862)), "{}", drift(862));
        assert!((1.1..1.3).contains(&drift(1400)), "{}", drift(1400));
        assert!((1.5..1.9).contains(&drift(1684)), "{}", drift(1684));
        // The accumulation alone, with the fitted phase taken out, is the
        // day and a half the constants predict: 823 years times 0.0018 days.
        let accumulated = drift(1684) - drift(862);
        assert!((1.3..1.7).contains(&accumulated), "{accumulated}");
        // Monotone, because the error is a fixed excess per year.
        assert!(drift(1000) < drift(1200));
        assert!(drift(1200) < drift(1400));
        assert!(drift(1400) < drift(1684));
    }

    #[test]
    fn each_reform_placed_the_solstice_closer_to_the_sun_than_the_last() {
        // At the moment Jōkyō-reki took over, its solstice is right and
        // Senmyō-reki's is a day and a half late.
        let kyoto = MERIDIANS[0].offset_hours / 24.0;
        let truth = hc_astro::solstice(1684, hc_astro::solar::Solstice::December).0 + kyoto;
        let error = |model: MeanMotionModel| {
            let index = ((truth - model.solstice_epoch) / model.tropical_year).round() as i64;
            (model.winter_solstice(index) - truth).abs()
        };
        assert!(error(senmyo::MODEL) > 1.0);
        // Jōkyō-reki's solstice carries a fitted −0.40-day phase, so it is
        // not at zero; it is an order of magnitude closer than the calendar
        // it replaced, which is the claim.
        assert!(error(jokyo::MODEL) < 0.5);
        assert!(error(jokyo::MODEL) * 3.0 < error(senmyo::MODEL));
        // And in 1843, at the end, Kansei-reki is the best of the four.
        let truth = hc_astro::solstice(1843, hc_astro::solar::Solstice::December).0 + kyoto;
        let error = |model: MeanMotionModel| {
            let index = ((truth - model.solstice_epoch) / model.tropical_year).round() as i64;
            (model.winter_solstice(index) - truth).abs()
        };
        assert!(error(kansei::MODEL) < error(jokyo::MODEL));
        assert!(error(kansei::MODEL) < error(horyaku::MODEL));
        assert!(error(kansei::MODEL) < 0.5);
    }

    #[test]
    fn every_system_round_trips_over_the_days_it_covers() {
        // Exhaustive for the three Edo systems, which are short, and every
        // seventh day across Senmyō-reki's 823 years, which is 43 000
        // samples and enough to catch a systematic fault. The integration
        // tests walk every single day of all four against the published
        // table.
        for (name, parameters) in systems() {
            let engine = LunisolarCalendar::new(parameters);
            let first = parameters.earliest.expect("bounded");
            let last = parameters.latest.expect("bounded");
            let stride = if parameters.id == senmyo::ID { 7 } else { 1 };
            let mut rd = first.0;
            while rd <= last.0 {
                let date = engine.from_fixed(Rd(rd)).expect("in range");
                assert_eq!(engine.to_fixed(date), Ok(Rd(rd)), "{name} RD {rd}");
                rd += stride;
            }
            // Both ends exactly, whatever the stride landed on.
            for edge in [first, last] {
                let date = engine.from_fixed(edge).expect("in range");
                assert_eq!(engine.to_fixed(date), Ok(edge), "{name} edge {edge}");
            }
        }
    }

    #[test]
    fn every_month_of_every_system_is_twenty_nine_or_thirty_days() {
        for (name, parameters) in systems() {
            let first = parameters.earliest.expect("bounded");
            let last = parameters.latest.expect("bounded");
            let mut cursor = parameters.new_moon_on_or_after(first);
            let mut months = 0;
            while cursor.0 + 30 <= last.0 {
                let next = parameters.new_moon_on_or_after(Rd(cursor.0 + 1));
                let length = next.0 - cursor.0;
                assert!((29..=30).contains(&length), "{name} at {cursor}: {length}");
                assert_eq!(
                    LunisolarCalendar::new(parameters)
                        .from_fixed(cursor)
                        .expect("in range")
                        .day,
                    1,
                    "{name} at {cursor}"
                );
                months += 1;
                cursor = next;
            }
            assert!(months > 10, "{name} examined only {months} months");
        }
    }

    #[test]
    fn a_true_conjunction_month_is_sometimes_thirty_days_twice_running() {
        // The signature of 定朔 against 平朔: mean conjunctions give a
        // strict 30-29 alternation, true ones occasionally give two long
        // months in a row (連大) and, more rarely, two short ones. Finding
        // both is how this test knows the correction is actually applied.
        let parameters = &senmyo::PARAMETERS;
        let mut long_run = false;
        let mut short_run = false;
        let mut cursor = parameters.new_moon_on_or_after(civil::to_rd(1000, 1, 1));
        let end = civil::to_rd(1100, 1, 1);
        let mut previous = 0;
        while cursor < end {
            let next = parameters.new_moon_on_or_after(Rd(cursor.0 + 1));
            let length = next.0 - cursor.0;
            if previous == 30 && length == 30 {
                long_run = true;
            }
            if previous == 29 && length == 29 {
                short_run = true;
            }
            previous = length;
            cursor = next;
        }
        assert!(long_run, "no 連大 in a century of Senmyō-reki");
        assert!(short_run, "no 連小 in a century of Senmyō-reki");
    }

    #[test]
    fn the_mean_conjunction_rule_alternates_strictly_and_the_true_one_does_not() {
        // The same constants under 平朔, to show the modes really differ.
        static MEAN: LunisolarParameters = LunisolarParameters {
            id: CalendarId("japanese-senmyo-heisaku"),
            english_name: "Senmyō-reki with mean conjunctions",
            meridians: &MERIDIANS,
            epoch: CHINESE_EPOCH,
            year_offset: YEAR_OFFSET,
            solar_term_mode: SolarTermMode::Mean,
            mean_motion: Some(MeanMotionModel {
                conjunction_mode: ConjunctionMode::Mean,
                advance_limit: None,
                ..senmyo::MODEL
            }),
            earliest: None,
            latest: None,
        };
        // A mean-conjunction calendar alternates 30, 29, 30, 29 almost
        // perfectly, because the months are 29.53 days and nothing displaces
        // them; a true-conjunction one does not.
        let mut mean_alternations = 0;
        let mut true_alternations = 0;
        let mut months = 0;
        let mut mean_cursor = MEAN.new_moon_on_or_after(civil::to_rd(1000, 1, 1));
        let mut true_cursor = senmyo::PARAMETERS.new_moon_on_or_after(civil::to_rd(1000, 1, 1));
        let mut mean_previous = 0;
        let mut true_previous = 0;
        for _ in 0..1_200 {
            let mean_next = MEAN.new_moon_on_or_after(Rd(mean_cursor.0 + 1));
            let true_next = senmyo::PARAMETERS.new_moon_on_or_after(Rd(true_cursor.0 + 1));
            let mean_length = mean_next.0 - mean_cursor.0;
            let true_length = true_next.0 - true_cursor.0;
            if months > 0 {
                if mean_length != mean_previous {
                    mean_alternations += 1;
                }
                if true_length != true_previous {
                    true_alternations += 1;
                }
            }
            mean_previous = mean_length;
            true_previous = true_length;
            mean_cursor = mean_next;
            true_cursor = true_next;
            months += 1;
        }
        assert!(
            mean_alternations > 1_100,
            "mean months alternated only {mean_alternations} times in {months}"
        );
        assert!(
            true_alternations < 1_000,
            "true months alternated {true_alternations} times, too regularly"
        );
    }

    #[test]
    fn the_tabulated_and_apparent_conjunctions_disagree_on_some_months() {
        // The two reconstructions of 定朔 this module offers really are two
        // reconstructions: they place a substantial minority of month
        // boundaries on different days, which is why the README reports both
        // agreement rates rather than one.
        let mut differing = 0;
        let start = civil::to_rd(1200, 1, 1);
        for offset in 0..3_000 {
            let rd = Rd(start.0 + offset);
            if senmyo::PARAMETERS_TABULATED.new_moon_before(rd)
                != senmyo::PARAMETERS.new_moon_before(rd)
            {
                differing += 1;
            }
        }
        assert!(
            (100..2_000).contains(&differing),
            "{differing} days of 3000 differed"
        );
    }

    #[test]
    fn the_years_of_every_system_hold_twelve_or_thirteen_months() {
        for (name, parameters) in systems() {
            let first = civil::year_from_rd(parameters.earliest.expect("bounded")) + 1;
            let last = civil::year_from_rd(parameters.latest.expect("bounded")) - 1;
            for year in first..=last {
                let months = parameters.months_in_year(year).expect("in range");
                assert!(months == 12 || months == 13, "{name} year {year}: {months}");
            }
        }
    }

    #[test]
    fn leap_months_come_seven_times_in_nineteen_years() {
        // The Metonic ratio, which the no-中気 rule reproduces without being
        // told about it: 7/19 = 36.8 leap years a century.
        for (name, start) in [("senmyo early", 900i64), ("senmyo late", 1_500)] {
            let leaps = (start..start + 100)
                .filter(|&year| senmyo::PARAMETERS.is_leap_year(year) == Ok(true))
                .count();
            assert!((33..=41).contains(&leaps), "{name}: {leaps} in a century");
        }
    }

    #[test]
    fn the_mean_solar_terms_let_a_leap_first_month_happen() {
        // Under 定気 the intervals between 中気 are unequal, the longest fall
        // in northern summer, and a leap first month is nearly impossible.
        // Under 恒気 the intervals are all equal, so the leap month is spread
        // evenly over the year and 閏正月 is ordinary. Reproducing that
        // difference is one of the reasons the term mode is a parameter.
        let found = (863..1_684)
            .filter(|&year| senmyo::PARAMETERS.leap_month(year) == Ok(Some(1)))
            .count();
        assert!(found > 20, "only {found} leap first months in 823 years");
    }

    #[test]
    fn a_leap_month_immediately_follows_the_month_it_repeats() {
        for (name, parameters) in systems() {
            let first = civil::year_from_rd(parameters.earliest.expect("bounded")) + 1;
            let last = civil::year_from_rd(parameters.latest.expect("bounded")) - 1;
            for year in first..=last {
                let Ok(Some(ordinal)) = parameters.leap_month(year) else {
                    continue;
                };
                let regular = parameters
                    .to_fixed(year, Month::regular(ordinal), 1)
                    .expect("the repeated month exists");
                let leap = parameters
                    .to_fixed(year, Month::leap(ordinal), 1)
                    .expect("the leap month exists");
                let length = i64::from(
                    parameters
                        .days_in_month(year, Month::regular(ordinal))
                        .expect("exists"),
                );
                assert_eq!(leap.0 - regular.0, length, "{name} year {year}");
            }
        }
    }

    #[test]
    fn a_leap_month_a_year_does_not_have_is_refused() {
        for year in [1_000i64, 1_300, 1_600] {
            let leap = senmyo::PARAMETERS.leap_month(year).expect("in range");
            for ordinal in 1..=12u8 {
                if leap != Some(ordinal) {
                    assert_eq!(
                        senmyo::PARAMETERS.to_fixed(year, Month::leap(ordinal), 1),
                        Err(CalendarError::MonthOutOfRange),
                        "year {year} leap {ordinal}"
                    );
                }
            }
        }
    }

    #[test]
    fn out_of_range_months_and_days_are_refused_by_every_system() {
        for (name, parameters) in systems() {
            let year = civil::year_from_rd(parameters.earliest.expect("bounded")) + 1;
            assert_eq!(
                parameters.to_fixed(year, Month::regular(0), 1),
                Err(CalendarError::MonthOutOfRange),
                "{name}"
            );
            assert_eq!(
                parameters.to_fixed(year, Month::regular(13), 1),
                Err(CalendarError::MonthOutOfRange),
                "{name}"
            );
            assert_eq!(
                parameters.to_fixed(year, Month::regular(1), 0),
                Err(CalendarError::DayOutOfRange),
                "{name}"
            );
            assert_eq!(
                parameters.to_fixed(year, Month::regular(1), 31),
                Err(CalendarError::DayOutOfRange),
                "{name}"
            );
        }
    }

    #[test]
    fn every_system_refuses_the_days_outside_its_period_of_use() {
        for (name, parameters) in systems() {
            let engine = LunisolarCalendar::new(parameters);
            let first = parameters.earliest.expect("bounded");
            let last = parameters.latest.expect("bounded");
            assert_eq!(
                engine.from_fixed(Rd(first.0 - 1)),
                Err(CalendarError::BeforeEpoch),
                "{name}"
            );
            assert_eq!(
                engine.from_fixed(Rd(last.0 + 1)),
                Err(CalendarError::AfterSupportedRange),
                "{name}"
            );
            assert!(engine.from_fixed(first).is_ok(), "{name}");
            assert!(engine.from_fixed(last).is_ok(), "{name}");
        }
    }

    #[test]
    fn a_year_outside_the_period_of_use_is_refused_rather_than_extrapolated() {
        assert_eq!(senmyo::new_year(861), Err(CalendarError::YearOutOfRange));
        assert_eq!(senmyo::new_year(1_686), Err(CalendarError::YearOutOfRange));
        assert_eq!(jokyo::new_year(1_684), Err(CalendarError::YearOutOfRange));
        assert_eq!(jokyo::new_year(1_756), Err(CalendarError::YearOutOfRange));
        assert_eq!(horyaku::new_year(1_754), Err(CalendarError::YearOutOfRange));
        assert_eq!(kansei::new_year(1_797), Err(CalendarError::YearOutOfRange));
        assert_eq!(kansei::new_year(1_845), Err(CalendarError::YearOutOfRange));
    }

    #[test]
    fn the_new_year_never_leaves_the_second_half_of_january_or_february() {
        // The lunisolar year starts at the second new moon after the winter
        // solstice, which pins it between about 21 January and 21 February.
        for year in 863..1_684i64 {
            let rd = senmyo::new_year(year).expect("in range");
            let (_, month, day) = civil::from_rd(rd);
            let within = (month == 1 && day >= 20) || (month == 2 && day <= 23);
            assert!(within, "year {year} began on {month}-{day}");
        }
    }

    #[test]
    fn month_eleven_contains_the_winter_solstice_in_every_system() {
        for (name, parameters) in systems() {
            let first = civil::year_from_rd(parameters.earliest.expect("bounded")) + 1;
            let last = civil::year_from_rd(parameters.latest.expect("bounded")) - 1;
            for year in first..=last {
                let solstice = parameters.winter_solstice_day(year);
                let (_, month, _) = parameters.from_fixed(solstice).expect("in range");
                assert_eq!(month.ordinal, 11, "{name} solstice of {year}");
                assert!(!month.leap, "{name} solstice of {year}");
            }
        }
    }

    #[test]
    fn the_metadata_bounds_each_system_to_its_period_of_use() {
        for (name, parameters) in systems() {
            let meta = parameters.meta();
            assert!(meta.has_leap_months, "{name}");
            assert!(meta.is_astronomical, "{name}");
            assert!(meta.earliest.is_some(), "{name}");
            assert!(meta.latest.is_some(), "{name}");
            assert!(
                !meta.supports(Rd(meta.latest.expect("bounded").0 + 1)),
                "{name}"
            );
            assert!(
                !meta.supports(Rd(meta.earliest.expect("bounded").0 - 1)),
                "{name}"
            );
        }
    }

    #[test]
    fn every_system_registers_under_its_own_identifier() {
        let ids = [
            senmyo::ID,
            jokyo::ID,
            horyaku::ID,
            kansei::ID,
            crate::japanese_tenpo::ID,
        ];
        for (index, id) in ids.iter().enumerate() {
            assert!(id.0.starts_with("japanese-"), "{id}");
            for other in &ids[index + 1..] {
                assert_ne!(id, other);
            }
        }
    }

    #[test]
    fn the_sexagenary_year_of_a_historical_date_matches_the_shared_cycle() {
        // 天正10年 (1582) is 壬午, the Water Horse; the cycle is the shared
        // East Asian one and does not depend on which system was in force.
        let cycle = senmyo::PARAMETERS.sexagenary_year(1_582);
        assert_eq!(cycle.stem_name(), "ren");
        assert_eq!(cycle.branch_name(), "wu");
        assert_eq!(cycle.zodiac_animal(), "horse");
        // 慶長5年 (1600) is 庚子.
        let cycle = senmyo::PARAMETERS.sexagenary_year(1_600);
        assert_eq!(cycle.stem_name(), "geng");
        assert_eq!(cycle.branch_name(), "zi");
    }

    #[test]
    fn the_generic_field_interface_round_trips_a_historical_date() {
        let calendar = senmyo::SenmyoCalendar;
        let date = calendar
            .from_fixed(civil::to_rd(1582, 7, 1))
            .expect("in range");
        let fields = calendar.to_fields(date).expect("describable");
        assert_eq!(fields.year, 1_582);
        assert_eq!(fields.month, Some(Month::regular(6)));
        assert_eq!(fields.day, Some(2));
        assert_eq!(calendar.from_fields(&fields), Ok(date));
    }

    #[test]
    fn the_model_finds_the_solstice_bracketing_any_day() {
        let model = senmyo::MODEL;
        for offset in 0..800i64 {
            let rd = Rd(civil::to_rd(1200, 1, 1).0 + offset);
            let solstice = model.winter_solstice_on_or_before(rd);
            assert!(solstice <= rd);
            let gap = rd.0 - solstice.0;
            assert!(gap < 366, "gap of {gap} days at {rd}");
        }
    }

    #[test]
    fn the_model_finds_the_conjunction_bracketing_any_day() {
        let parameters = &senmyo::PARAMETERS;
        for offset in 0..400i64 {
            let rd = Rd(civil::to_rd(1300, 6, 1).0 + offset);
            let before = parameters.new_moon_before(rd);
            let after = parameters.new_moon_on_or_after(rd);
            assert!(before < rd);
            assert!(after >= rd);
            assert!((1..=30).contains(&(after.0 - before.0)), "{rd}");
        }
    }

    #[test]
    fn shinsaku_holds_a_late_conjunction_over_to_the_next_day() {
        // 進朔 by itself, with no astronomy: a conjunction before the limit
        // starts its month that day and one after it starts the next.
        let model = senmyo::MODEL;
        let limit = model.advance_limit.expect("Senmyō-reki holds over");
        assert_eq!(model.day_of_conjunction(1_000.0), Rd(1_000));
        assert_eq!(model.day_of_conjunction(1_000.0 + limit - 0.001), Rd(1_000));
        assert_eq!(model.day_of_conjunction(1_000.0 + limit + 0.001), Rd(1_001));
        assert_eq!(model.day_of_conjunction(1_000.999), Rd(1_001));
        // The later systems abolished it, so every moment stays on its day.
        for parameters in [
            &jokyo::PARAMETERS,
            &horyaku::PARAMETERS,
            &kansei::PARAMETERS,
        ] {
            let model = model_of(parameters);
            assert_eq!(model.advance_limit, None);
            assert_eq!(model.day_of_conjunction(1_000.999), Rd(1_000));
        }
    }

    #[test]
    fn the_major_solar_terms_run_one_to_twelve_and_back() {
        let model = senmyo::MODEL;
        let mut seen = [false; 13];
        let start = civil::to_rd(1500, 1, 1);
        let mut previous = model.major_solar_term(start);
        for offset in 0..400i64 {
            let term = model.major_solar_term(Rd(start.0 + offset));
            assert!((1..=12).contains(&term));
            seen[term as usize] = true;
            // The index only ever advances by one, wrapping 12 to 1.
            assert!(
                term == previous || term == previous % 12 + 1,
                "{previous} -> {term}"
            );
            previous = term;
        }
        assert!(seen[1..].iter().all(|&s| s), "not every term appeared");
    }

    #[test]
    fn the_day_after_the_solstice_carries_the_eleventh_major_solar_term() {
        // 冬至 *is* term 11, which is why month 11 must contain it — but the
        // term index is read at local midnight and reports the last term
        // already passed, so the solstice's own day still reads 10 and the
        // day after reads 11. The leap rule compares the index at two month
        // boundaries, so the off-by-a-midnight is exactly right for it.
        let model = senmyo::MODEL;
        for index in 0..800i64 {
            let day = model.winter_solstice_day(index);
            assert_eq!(model.major_solar_term(day), 10, "solstice index {index}");
            assert_eq!(
                model.major_solar_term(Rd(day.0 + 1)),
                11,
                "solstice index {index}"
            );
        }
    }

    #[test]
    fn the_epochs_are_the_sky_of_the_adoption_year_and_nothing_later() {
        // The models are phased on the sky at adoption and then left alone,
        // which is what makes the drift accumulate as it historically did.
        // This test pins the phasing so that a stray edit to a constant
        // cannot pass unnoticed.
        let kyoto = MERIDIANS[0].offset_hours / 24.0;
        for (name, parameters, year) in [
            ("senmyo", &senmyo::PARAMETERS, 861i64),
            ("jokyo", &jokyo::PARAMETERS, 1_684),
            ("horyaku", &horyaku::PARAMETERS, 1_754),
            ("kansei", &kansei::PARAMETERS, 1_797),
        ] {
            let model = model_of(parameters);
            let truth = hc_astro::solstice(year, hc_astro::solar::Solstice::December).0 + kyoto;
            // The fitted phase is at most half a day either way. Anything
            // larger would mean a constant had been edited into nonsense.
            let fitted = model.solstice_epoch - truth;
            assert!(
                fitted.abs() < 0.55,
                "{name}: solstice epoch is {fitted} days from the sky of {year}"
            );
            // And the mean conjunction the months are numbered from falls on
            // the calendar's first day.
            let first = parameters.earliest.expect("bounded");
            assert_eq!(
                Rd(model.mean_conjunction(0).floor() as i64),
                first,
                "{name}: conjunction epoch is not the adoption day"
            );
        }
    }

    #[test]
    fn the_omitted_lunar_inequalities_are_the_size_the_documentation_claims() {
        // The model carries one lunar term where the true Moon has several.
        // Measuring the residual against hc-astro is how the README's
        // accuracy claim is kept honest: it is a few tenths of a day, never
        // more than about half.
        let model = senmyo::MODEL_TABULATED;
        let mut worst: f64 = 0.0;
        let mut total = 0.0;
        let mut count = 0.0;
        let kyoto = MERIDIANS[0].offset_hours / 24.0;
        for index in 0..400i64 {
            let ours = model.conjunction(index * 25);
            let truth =
                hc_astro::new_moon_before(hc_calendar::fixed::Moment(ours + 14.0)).0 + kyoto;
            let error = ours - truth;
            worst = worst.max(error.abs());
            total += error.abs();
            count += 1.0;
        }
        let mean = total / count;
        assert!(mean < 0.25, "mean conjunction error {mean} days");
        assert!(worst < 0.7, "worst conjunction error {worst} days");
    }
}
