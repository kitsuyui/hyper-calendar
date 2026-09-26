//! Regional, cyclic and era calendars for `hyper-calendar`.
//!
//! The calendars here have one thing in common: **the day has a name before
//! it has a number.** A Maya day is *4 Ahau 8 Cumku* before it is the
//! 1 872 000th day of anything; a Balinese day is *Buda Kliwon Dungulan*,
//! a position in two of ten concurrent week cycles and in one of thirty
//! *wuku*; a Japanese day belongs to an era that a government proclaimed.
//! Those are not counts of years from an epoch with months cut out of them,
//! which is why they do not fit in [`hc_calendars_solar`] or
//! [`hc_calendars_lunar`]. The Burmese, Thai and Khmer lunar calendars are
//! such counts, and are here as regional calendars.
//!
//! | Module | Calendars |
//! | --- | --- |
//! | [`japanese`] | `japanese`, `japanese-northern`, `japanese-southern`, `japanese-proclaimed` — imperial eras (和暦), Gregorian from 1873 and lunisolar before it |
//! | [`maya`] | `maya-longcount`, `maya-tzolkin`, `maya-haab`, `maya-round`, and the same four under the GMT+2 correlation as `maya-longcount-gmt2`, `maya-tzolkin-gmt2`, `maya-haab-gmt2`, `maya-round-gmt2` |
//! | [`maya_819`] | `maya-819`, `maya-819-gmt2` — the 819-day count's stations and colour-directions over Linden and Bricker's twenty-station cycle of 16 380 days, under the two correlations |
//! | [`aztec`] | `aztec-tonalpohualli`, `aztec-xiuhpohualli` |
//! | [`zapotec`] | `zapotec-yza` — the Zapotec 365-day year of the Villa Alta calendars, its months in the order of Manuscript 85 and its years named by the day they begin on |
//! | [`balinese_pawukon`] | `balinese-pawukon` — thirty *wuku* and ten concurrent week cycles over 210 days |
//! | [`javanese_pasaran`] | `javanese-pasaran` — the five-day market week and the 35-day wetonan |
//! | [`akan`] | `akan` — the Akan six-day week and the 42-day Adaduanan it makes with the seven-day one |
//! | [`korean_regnal`] | `korean-regnal` — the three eras of the Korean Empire, 建陽, 光武 and 隆熙, on the Gregorian days of 1896–1910 |
//! | [`chinese_regnal`] | `chinese-regnal` — the Qing eras over the Chinese lunisolar calendar, 1645 to the abdication of 1912, with the Ming and Qing era table as data |
//! | [`burmese`] | `burmese` — the Myanmar Era's lunisolar calendar, its watat years and full moons by the Calendar Advisory Board's arithmetic and the record's exceptions |
//! | [`thai_lunar`] | `thai-lunar` — the Thai lunar calendar, its adhikamāsa and adhikavāra years carried as published for 2535–2570 BE (1992–2027) |
//! | [`khmer`] | `khmer` — the Khmer *Chhankitek*, its leap-month and leap-day years by the *suryayatra* rule as Cambodia applies it, 1900–2200 |
//! | [`southeast_asian`] | No calendar: the year layout `thai-lunar` and `khmer` share, and the *suryayatra* quantities of the solar New Year |
//! | [`sexagenary`] | `sexagenary` — 干支 over years, months and days |
//!
//! # Cyclic calendars and the round-trip contract
//!
//! [`hc_calendar::Calendar`] demands a bijection between dates and fixed
//! days, and a cycle is by construction not a bijection: the 260-day
//! tzolk'in names the same day every 260 days for ever. Every cyclic
//! calendar here therefore carries a **round number** — how many complete
//! cycles have elapsed since its epoch — alongside the position within the
//! cycle. `13.0.0.0.0` is a long count; *4 Ahau* is a position;
//! `(round, 4 Ahau)` is a date.
//!
//! The round number lands in [`hc_calendar::DateFields::year`], because that
//! is the only field every calendar has and because the round really is the
//! coarsest unit these calendars count in. Where a calendar has something
//! more year-like — the Haab's 365 days, the Pawukon's 30 *wuku* — that goes
//! in the month and day fields, and the concurrent cycles go in
//! [`hc_calendar::fields::ExtraFields`].
//!
//! # What this crate refuses to do
//!
//! It will not guess. Where a Japanese era's start date is disputed the data
//! says so; where only the month is known the day is absent rather than
//! invented; and where the Northern and Southern Courts ran two era systems
//! at once, [`nengo::era_at`] asks which court you mean instead of picking
//! one. Where a correlation constant is a scholarly choice rather than a
//! fact — the Maya GMT constant, the Aztec correlation — the module names
//! the constant it uses, names the alternative, and says what the difference
//! costs.
//!
//! # Example
//!
//! ```
//! use hc_calendar::Calendar;
//! use hc_calendars_regional::{JapaneseCalendar, MayaLongCountCalendar};
//! use hc_calendars_solar::gregorian;
//!
//! let end_of_the_thirteenth_baktun = gregorian::to_fixed(2012, 12, 21)?;
//! let long_count = MayaLongCountCalendar::GMT.from_fixed(end_of_the_thirteenth_baktun)?;
//! assert_eq!(long_count.to_string(), "13.0.0.0.0");
//!
//! let heisei = JapaneseCalendar::UNIFIED.from_fixed(end_of_the_thirteenth_baktun)?;
//! assert_eq!(heisei.era.kanji, "平成");
//! assert_eq!((heisei.year, heisei.month.ordinal, heisei.day), (24, 12, 21));
//! # Ok::<(), hc_calendar::CalendarError>(())
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

#[cfg(feature = "alloc")]
extern crate alloc;

/// How far apart the days of a day-by-day sweep in this crate's tests are:
/// every day in a release build, every `sampled`th in a debug build.
///
/// The full sweeps take minutes under the coverage job's instrumentation,
/// so a debug or coverage run takes a fixed, deterministic sample of them
/// and CI's release-mode test job walks every day; the published anchors
/// are checked in full either way (docs/policy.md §7).
#[cfg(test)]
pub(crate) const fn sweep_stride(sampled: usize) -> usize {
    if cfg!(debug_assertions) { sampled } else { 1 }
}

pub mod akan;
pub mod aztec;
pub mod balinese_pawukon;
pub mod burmese;
pub mod chinese_regnal;
pub mod japanese;
pub mod javanese_pasaran;
pub mod khmer;
pub mod korean_regnal;
pub mod maya;
pub mod maya_819;
pub mod nengo;
pub mod sexagenary;
pub mod southeast_asian;
pub mod thai_lunar;
mod vague_year;
pub mod zapotec;

pub use akan::{AkanCalendar, AkanDate};
pub use aztec::{
    AztecTonalpohualliCalendar, AztecTonalpohualliDate, AztecXiuhpohualliCalendar,
    AztecXiuhpohualliDate,
};
pub use balinese_pawukon::{BalinesePawukonCalendar, PawukonDate};
pub use burmese::{BurmeseCalendar, BurmeseDate, MoonPhase, Thingyan, YearType};
pub use chinese_regnal::{ChineseEra, ChineseRegnalCalendar, ChineseRegnalDate, Dynasty};
pub use japanese::{JapaneseCalendar, JapaneseDate};
pub use javanese_pasaran::{JavanesePasaranCalendar, WetonDate};
pub use khmer::{KhmerCalendar, KhmerDate};
pub use korean_regnal::{KoreanEra, KoreanRegnalCalendar, KoreanRegnalDate};
pub use maya::{
    MayaCalendarRoundCalendar, MayaCalendarRoundDate, MayaHaabCalendar, MayaHaabDate,
    MayaLongCountCalendar, MayaLongCountDate, MayaTzolkinCalendar, MayaTzolkinDate,
};
pub use maya_819::{Maya819Calendar, Maya819Date};
pub use nengo::{Certainty, Court, Nengo, WesternScale};
pub use sexagenary::{SexagenaryCalendar, SexagenaryDayDate};
pub use thai_lunar::{ThaiLunarCalendar, ThaiLunarDate};
pub use zapotec::{ZapotecYzaCalendar, ZapotecYzaDate};

pub use hc_calendar;
pub use hc_calendars_lunar;
pub use hc_calendars_solar;

#[cfg(feature = "alloc")]
mod registration {
    use alloc::boxed::Box;

    use hc_calendar::{CalendarRegistry, DynAdapter};

    use crate::JapaneseCalendar;

    /// Register every calendar in this crate with `registry`.
    ///
    /// Inserting is idempotent: a second call replaces rather than
    /// duplicates, since [`CalendarRegistry::insert`] keys on the
    /// calendar's identifier.
    ///
    /// The four Maya calendars are registered separately because they are
    /// four calendars and not four views of one: a Maya scribe who wrote a
    /// Calendar Round date had said something a long count date does not
    /// say, and vice versa. Each is registered under both correlation
    /// constants, so that the cycles read beside `maya-longcount-gmt2` are
    /// anchored as it is.
    pub fn register_all(registry: &mut CalendarRegistry) {
        registry.insert(Box::new(DynAdapter::new(JapaneseCalendar::UNIFIED)));
        registry.insert(Box::new(DynAdapter::new(JapaneseCalendar::NORTHERN)));
        registry.insert(Box::new(DynAdapter::new(JapaneseCalendar::SOUTHERN)));
        registry.insert(Box::new(DynAdapter::new(JapaneseCalendar::PROCLAIMED)));
        registry.insert(Box::new(DynAdapter::new(crate::MayaLongCountCalendar::GMT)));
        registry.insert(Box::new(DynAdapter::new(
            crate::MayaLongCountCalendar::GMT_PLUS_TWO,
        )));
        registry.insert(Box::new(DynAdapter::new(crate::MayaTzolkinCalendar::GMT)));
        registry.insert(Box::new(DynAdapter::new(crate::MayaHaabCalendar::GMT)));
        registry.insert(Box::new(DynAdapter::new(
            crate::MayaCalendarRoundCalendar::GMT,
        )));
        registry.insert(Box::new(DynAdapter::new(
            crate::MayaTzolkinCalendar::GMT_PLUS_TWO,
        )));
        registry.insert(Box::new(DynAdapter::new(
            crate::MayaHaabCalendar::GMT_PLUS_TWO,
        )));
        registry.insert(Box::new(DynAdapter::new(
            crate::MayaCalendarRoundCalendar::GMT_PLUS_TWO,
        )));
        registry.insert(Box::new(DynAdapter::new(crate::Maya819Calendar::GMT)));
        registry.insert(Box::new(DynAdapter::new(
            crate::Maya819Calendar::GMT_PLUS_TWO,
        )));
        registry.insert(Box::new(DynAdapter::new(crate::AztecTonalpohualliCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::AztecXiuhpohualliCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::ZapotecYzaCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::BalinesePawukonCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::JavanesePasaranCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::AkanCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::KoreanRegnalCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::ChineseRegnalCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::BurmeseCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::SexagenaryCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::ThaiLunarCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::KhmerCalendar)));
    }
}

#[cfg(feature = "alloc")]
pub use registration::register_all;

/// How many calendars [`register_all`] inserts.
#[cfg(test)]
const CALENDAR_COUNT: usize = 26;

#[cfg(test)]
mod tests {
    use hc_calendar::{Calendar, Rd};
    use hc_calendars_solar::gregorian;

    use super::*;
    use hc_calendars_lunar::japanese_historical;

    /// Round-trip one fixed day through every calendar that supports it,
    /// returning how many did.
    ///
    /// A macro rather than a collection of trait objects so that the test
    /// exercises the *static* interface, where each calendar has its own
    /// date type, rather than only the erased one.
    macro_rules! round_trip_every_calendar {
        ($rd:expr, $($calendar:expr),+ $(,)?) => {{
            let mut checked = 0;
            $(
                {
                    let calendar = $calendar;
                    if calendar.meta().supports($rd) {
                        let date = calendar.from_fixed($rd).unwrap_or_else(|error| {
                            panic!("{} rejected {} in range: {error}", calendar.meta().id, $rd)
                        });
                        assert_eq!(
                            calendar.to_fixed(date),
                            Ok($rd),
                            "{} failed to round-trip {}",
                            calendar.meta().id,
                            $rd
                        );
                        let fields = calendar.to_fields(date).unwrap();
                        assert_eq!(
                            calendar.from_fields(&fields),
                            Ok(date),
                            "{} failed to round-trip fields at {}",
                            calendar.meta().id,
                            $rd
                        );
                        checked += 1;
                    }
                }
            )+
            checked
        }};
    }

    #[test]
    fn every_calendar_round_trips_every_day_of_a_shared_range() {
        // The step is deliberately coprime with 210, 260, 365 and 60 so that
        // the sweep does not land on the same cycle position every time.
        for rd in (-1_100_000..=800_000).step_by(1_009) {
            round_trip_every_calendar!(
                Rd(rd),
                MayaLongCountCalendar::GMT,
                MayaLongCountCalendar::GMT_PLUS_TWO,
                MayaTzolkinCalendar::GMT,
                MayaHaabCalendar::GMT,
                MayaCalendarRoundCalendar::GMT,
                MayaTzolkinCalendar::GMT_PLUS_TWO,
                MayaHaabCalendar::GMT_PLUS_TWO,
                MayaCalendarRoundCalendar::GMT_PLUS_TWO,
                AztecTonalpohualliCalendar,
                AztecXiuhpohualliCalendar,
                ZapotecYzaCalendar,
                BalinesePawukonCalendar,
                JavanesePasaranCalendar,
                AkanCalendar,
                KoreanRegnalCalendar,
                ChineseRegnalCalendar,
                BurmeseCalendar,
                SexagenaryCalendar,
                ThaiLunarCalendar,
                KhmerCalendar,
            );
        }
    }

    /// Round-trip one day through every representation this calendar has.
    fn check_japanese_day(rd: Rd) {
        let date = JapaneseCalendar::UNIFIED.from_fixed(rd).expect("in range");
        assert_eq!(JapaneseCalendar::UNIFIED.to_fixed(date), Ok(rd), "{rd}");
        let fields = JapaneseCalendar::UNIFIED
            .to_fields(date)
            .expect("describable");
        assert_eq!(
            JapaneseCalendar::UNIFIED.from_fields(&fields),
            Ok(date),
            "{rd}"
        );
    }

    #[test]
    fn the_japanese_calendar_round_trips_exhaustively_where_it_can_break() {
        // This crate is responsible for era attribution and for choosing the
        // calendar that was in force. The lunisolar arithmetic underneath is
        // `hc-calendars-lunar`'s, and is already validated there against the
        // first day and length of all 12 146 months from 862 to 1843. Walking
        // all 452 000 days again here re-tests that crate's work at several
        // minutes a run, so this sweeps exhaustively at every seam and
        // samples the flat stretches between them.
        let meta = JapaneseCalendar::UNIFIED.meta();
        let first = meta.earliest.expect("bounded below");
        let last = gregorian::to_fixed(2100, 12, 31).expect("in range");

        // A debug build keeps every day within a few of each seam and
        // samples the rest (`crate::sweep_stride`); a release build walks
        // all of it.
        let sampled = crate::sweep_stride(5) as i64;

        // Every era boundary, from forty days before to forty days after.
        // An era change lands mid-month and sometimes mid-intercalary-month,
        // which is where the year-within-era arithmetic goes wrong.
        for era in crate::nengo::stream(crate::nengo::Court::Unified) {
            let Some(start) = era.start else { continue };
            for offset in
                (-40..=40).filter(|offset: &i64| offset.abs() <= 3 || offset % sampled == 0)
            {
                let rd = Rd(start.0 + offset);
                if rd < first || rd > last || crate::nengo::is_nanbokucho(rd) {
                    continue;
                }
                check_japanese_day(rd);
            }
        }

        // Every changeover between the five calendars, a year either side,
        // because a lunisolar year can straddle one.
        for seam in [
            japanese_historical::jokyo::EARLIEST,
            japanese_historical::horyaku::EARLIEST,
            japanese_historical::kansei::EARLIEST,
            hc_calendars_lunar::japanese_tenpo::EARLIEST,
            crate::japanese::GREGORIAN_ADOPTION,
        ] {
            for offset in
                (-400..=400).filter(|offset: &i64| offset.abs() <= 31 || offset % sampled == 0)
            {
                let rd = Rd(seam.0 + offset);
                if rd < first || rd > last || crate::nengo::is_nanbokucho(rd) {
                    continue;
                }
                check_japanese_day(rd);
            }
        }

        // The Gregorian half in full (every fifth day in a debug build): it
        // is what callers actually ask for.
        for rd in (crate::japanese::GREGORIAN_ADOPTION.0..=last.0).step_by(crate::sweep_stride(5)) {
            check_japanese_day(Rd(rd));
        }

        // And a stride through the lunisolar centuries, 31 days (157 in a
        // debug build): coprime with 29 and 30, so it does not keep landing
        // on the same position in the month.
        let mut rd = first.0;
        while rd < crate::japanese::GREGORIAN_ADOPTION.0 {
            let day = Rd(rd);
            if !crate::nengo::is_nanbokucho(day) {
                check_japanese_day(day);
            }
            rd += if sampled == 1 { 31 } else { 157 };
        }
    }

    #[test]
    fn bounded_calendars_refuse_days_outside_their_range() {
        for meta in [
            JapaneseCalendar::UNIFIED.meta(),
            MayaLongCountCalendar::GMT.meta(),
            MayaCalendarRoundCalendar::GMT.meta(),
            MayaCalendarRoundCalendar::GMT_PLUS_TWO.meta(),
            ThaiLunarCalendar.meta(),
        ] {
            let first = meta.earliest.expect("bounded below");
            let last = meta.latest.expect("bounded above");
            assert!(meta.supports(first) && meta.supports(last), "{}", meta.id);
            assert!(!meta.supports(Rd(first.0 - 1)), "{}", meta.id);
            assert!(!meta.supports(Rd(last.0 + 1)), "{}", meta.id);
            assert!(meta.check_range(Rd(first.0 - 1)).is_err());
            assert!(meta.check_range(Rd(last.0 + 1)).is_err());
        }
        assert!(JapaneseCalendar::UNIFIED.from_fixed(Rd(0)).is_err());
        assert!(
            MayaLongCountCalendar::GMT
                .from_fixed(Rd(-2_000_000))
                .is_err()
        );
    }

    #[test]
    fn the_unbounded_cycles_name_a_day_arbitrarily_far_back() {
        // A cycle has no epoch in the sense a calendar does, so the only
        // honest bound is the arithmetic one.
        let deep = Rd(-5_000_000);
        assert!(MayaTzolkinCalendar::GMT.from_fixed(deep).is_ok());
        assert!(BalinesePawukonCalendar.from_fixed(deep).is_ok());
        assert!(SexagenaryCalendar.from_fixed(deep).is_ok());
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn the_registry_holds_every_calendar_under_a_distinct_identifier() {
        use alloc::vec::Vec;
        use hc_calendar::{CalendarId, CalendarRegistry};

        let mut registry = CalendarRegistry::new();
        register_all(&mut registry);
        assert_eq!(registry.len(), CALENDAR_COUNT);

        let ids: Vec<CalendarId> = registry.metas().map(|meta| meta.id).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), ids.len(), "identifiers must be distinct");

        assert!(registry.get_by_name("japanese").is_some());
        assert!(registry.get_by_name("maya-longcount").is_some());
        assert!(registry.get_by_name("nonexistent").is_none());

        register_all(&mut registry);
        assert_eq!(registry.len(), CALENDAR_COUNT);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn one_day_renders_in_every_registered_calendar() {
        use hc_calendar::{CalendarId, CalendarRegistry};

        let mut registry = CalendarRegistry::new();
        register_all(&mut registry);

        let rd = gregorian::to_fixed(2026, 9, 21).expect("in range");
        let rendered = registry.describe_day(rd);
        // Every calendar answers; the ones that refuse the day are the
        // Korean Empire's, kept only from 1896 to 1910, and the Qing eras'.
        assert_eq!(rendered.len(), registry.len());
        let converted = rendered.iter().filter(|(_, fields)| fields.is_ok()).count();
        let supporting = registry.metas().filter(|meta| meta.supports(rd)).count();
        assert_eq!(converted, supporting);
        let refusing: Vec<&str> = rendered
            .iter()
            .filter(|(_, fields)| fields.is_err())
            .map(|(id, _)| id.0)
            .collect();
        assert_eq!(refusing, ["korean-regnal", "chinese-regnal"]);

        let japanese = registry
            .get(CalendarId("japanese"))
            .expect("registered")
            .fixed_to_fields(rd)
            .expect("in range");
        assert_eq!(japanese.era, Some("reiwa"));
        assert_eq!(japanese.year, 8);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn every_registered_calendar_reports_usable_metadata() {
        use hc_calendar::CalendarRegistry;

        let mut registry = CalendarRegistry::new();
        register_all(&mut registry);
        for meta in registry.metas() {
            assert!(!meta.id.as_str().is_empty());
            assert!(!meta.english_name.is_empty());
            // Only the era calendars over a lunisolar year carry intercalary
            // months — the Japanese, in the lunisolar half of its range, and
            // the Qing eras over the Chinese calendar — and the three Theravada
            // lunisolar calendars.
            assert!(
                !meta.has_leap_months
                    || meta.id.as_str().starts_with("japanese")
                    || meta.id.as_str() == "chinese-regnal"
                    || meta.id.as_str() == "burmese"
                    || meta.id.as_str() == "thai-lunar"
                    || meta.id.as_str() == "khmer"
            );
            if let (Some(first), Some(last)) = (meta.earliest, meta.latest) {
                assert!(first < last, "{} has an empty range", meta.id);
            }
        }
    }

    /// The calendars that declare a month cycle and still have no counted
    /// year: the Haab, the xiuhpohualli and the Zapotec yza run eighteen
    /// months and the five days, the calendar round carries the Haab's, and the Pawukon declares
    /// its thirty wuku as its month cycle; in each the `year` field is a
    /// position in a round, not a count, and nothing is ever intercalated
    /// into it.
    const YEAR_IS_A_ROUND_POSITION: &[&str] = &[
        "maya-haab",
        "maya-round",
        "maya-haab-gmt2",
        "maya-round-gmt2",
        "aztec-xiuhpohualli",
        "zapotec-yza",
        "balinese-pawukon",
    ];

    /// Every registered calendar answers whether a year is leap, or says
    /// that its dates have no year; nothing falls through to an inference.
    #[cfg(feature = "alloc")]
    #[test]
    fn every_registered_calendar_says_whether_a_year_is_leap() {
        use hc_calendar::{CalendarError, CalendarRegistry, Rd, shape::MONTH};

        let mut registry = CalendarRegistry::new();
        register_all(&mut registry);
        for meta in registry.metas() {
            let calendar = registry.get(meta.id).unwrap();
            let inside = match (meta.earliest, meta.latest) {
                (Some(first), Some(last)) => Rd((first.0 + last.0) / 2),
                _ => Rd(738_000),
            };
            // An era-relative calendar's fields carry the year within the
            // era; its `is_leap_year` takes the era-less year, which for the
            // regnal calendars is the Gregorian one.
            let year = match meta.year_kind {
                hc_calendar::YearKind::EraRelative => {
                    hc_calendar::gregorian::year_from_fixed(inside)
                }
                _ => calendar.fixed_to_fields(inside).unwrap().year,
            };
            match calendar.is_leap_year(year) {
                Ok(_) => {}
                Err(CalendarError::UnsupportedField("year")) => assert!(
                    !calendar.cycles().iter().any(|cycle| cycle.kind == MONTH)
                        || YEAR_IS_A_ROUND_POSITION.contains(&meta.id.as_str()),
                    "{} has months and so a year",
                    meta.id
                ),
                Err(error) => panic!("{} could not answer for {year}: {error}", meta.id),
            }
        }
    }

    /// The dynamic answer is the module's own rule: a watat year, a year the
    /// Thai table marks, the Gregorian rule from 1873 in Japan.
    #[test]
    fn the_dynamic_leap_year_is_the_module_rule() {
        use hc_calendar::{CalendarError, DynAdapter, DynCalendar};

        for year in (1_300..=1_400).step_by(3) {
            assert_eq!(
                DynAdapter::new(BurmeseCalendar).is_leap_year(year),
                Ok(burmese::year_info(year).year_type.has_watat())
            );
        }
        for year in thai_lunar::FIRST_YEAR..=thai_lunar::LAST_YEAR {
            assert_eq!(
                DynAdapter::new(ThaiLunarCalendar).is_leap_year(year),
                Ok(thai_lunar::year_type(year) != Some(thai_lunar::YearType::Normal))
            );
        }
        let japanese = DynAdapter::new(JapaneseCalendar::UNIFIED);
        for year in (1_873..=2_100).step_by(7) {
            assert_eq!(
                japanese.is_leap_year(year),
                Ok(gregorian::is_leap_year(year))
            );
        }
        for year in 1_845..=1_872 {
            assert_eq!(
                japanese.is_leap_year(year),
                hc_calendars_lunar::japanese_tenpo::PARAMETERS.is_leap_year(year)
            );
        }
        assert_eq!(
            DynAdapter::new(MayaLongCountCalendar::GMT).is_leap_year(13),
            Err(CalendarError::UnsupportedField("year"))
        );
        assert_eq!(
            DynAdapter::new(BalinesePawukonCalendar).is_leap_year(1),
            Err(CalendarError::UnsupportedField("year"))
        );
    }
}
