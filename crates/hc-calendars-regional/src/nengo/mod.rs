//! The nengō table: every Japanese era name from 大化 (645) to 令和.
//!
//! This module is **data**, not arithmetic. [`crate::japanese`] is the
//! calendar; this is the list it looks eras up in, kept separate so that a
//! caller who only wants to know when 元禄 began does not have to convert a
//! date to find out.
//!
//! # Where the data comes from
//!
//! Every row is taken from the Japanese Wikipedia article 元号一覧 (日本)
//! at <https://ja.wikipedia.org/wiki/元号一覧_(日本)>, which tabulates all
//! 248 eras with both the 和暦 changeover date and its Western equivalent,
//! and cites the primary chronologies — 『続史愚抄』, 『南朝公卿補任』,
//! 『七巻冊子』 — where those disagree. The five modern eras were checked
//! separately against that article's 明治以降 tables, which distinguish the
//! 公式 (retroactive, legal) dates from the 改元当時 (as proclaimed) ones;
//! [`Nengo::start`] says which this crate stores and why.
//!
//! # Julian or Gregorian?
//!
//! The Western dates a Japanese chronology quotes are **Julian** up to the
//! start of 天正 (元亀4年7月28日 = 1573-08-25) and **Gregorian** from the
//! end of 天正 (天正20年12月8日 = 1593-01-10) onwards, because the Catholic
//! reform of 1582 fell in the middle of that one era. No era begins inside
//! the gap, so the split is clean; each row records which calendar its
//! [`Nengo::western_year`], [`Nengo::western_month`] and
//! [`Nengo::western_day`] belong to, and [`Nengo::start`] resolves both to
//! a fixed day so that a caller never has to care.
//!
//! # The Northern and Southern Courts
//!
//! From 1331 to 1392 Japan had two imperial courts and two era systems
//! running at once, and days in between genuinely carry two era names. This
//! crate refuses to pick one: every era carries a [`Court`] and [`era_at`]
//! takes the court as an argument. Asking for [`Court::Unified`] inside
//! [`NANBOKUCHO_START`]..[`NANBOKUCHO_END`] returns
//! [`CalendarError::UnknownEra`] rather than an answer.
//!
//! # Identifiers
//!
//! [`Nengo::id`] is the era's reading romanised in Hepburn, mechanically
//! from the kana in the source, with macrons dropped and apostrophes
//! removed — 昭和 is `showa`, 安永 is `anei`. Twelve pairs of eras collide
//! under that rule; where they do, the Western start year is appended to
//! the older one, so 正和 (1312) is `showa-1312` and 昭和 (1926) keeps
//! `showa`. The kanji, being unique across the table, is always an
//! unambiguous key: see [`by_kanji`].

mod table;

use hc_calendar::{CalendarError, CalendarResult, Rd};

pub use table::ALL;

/// Which Western calendar a [`Nengo`]'s tabulated start date is expressed
/// in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WesternScale {
    /// The Julian calendar, as Japanese chronologies use before 1582.
    Julian,
    /// The Gregorian calendar.
    Gregorian,
}

/// Which imperial court proclaimed an era.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Court {
    /// A single court: everything outside 1331–1392, plus 建武, which both
    /// courts used until the Southern court replaced it in 1336.
    Unified,
    /// The Northern Court (北朝), the Jimyōin line installed by the
    /// Ashikaga. Its era 明徳 survived the reunification of 1392, because
    /// the union took the form of the Southern emperor abdicating.
    Northern,
    /// The Southern Court (南朝), the Daikakuji line of Go-Daigo, which
    /// later historiography treats as the legitimate one.
    Southern,
}

/// How firmly the sources fix an era's start.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Certainty {
    /// The chronologies agree on the day.
    Attested,
    /// The chronologies disagree, or the promulgation date is inferred.
    /// [`Nengo::start`] holds the conventional reading and is not a fact.
    Disputed,
    /// Only the month is known. [`Nengo::start`] is [`None`] and
    /// [`Nengo::western_day`] is zero; this crate will not invent a day.
    MonthOnly,
}

/// One Japanese era name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Nengo {
    /// The machine identifier: the Hepburn reading, lower case, with
    /// colliding pairs disambiguated by Western start year.
    pub id: &'static str,
    /// The era name in kanji, for example `"令和"`.
    pub kanji: &'static str,
    /// The reading in hiragana, for example `"れいわ"`.
    ///
    /// Pre-Meiji readings were never officially fixed and the source lists
    /// several for some eras — 白雉 is はくち, びゃくち and しらきぎす —
    /// so this is the first reading the source gives, not the reading.
    pub reading: &'static str,
    /// The reading in Hepburn romanisation with an initial capital, for
    /// example `"Reiwa"`. Long vowels are written without macrons.
    pub romaji: &'static str,
    /// Which court proclaimed the era.
    pub court: Court,
    /// The Japanese calendar year that this era's **year 1** (元年)
    /// occupies.
    ///
    /// Japanese years are numbered here by the Western year in which they
    /// begin, which for a lunisolar year is the year containing its first
    /// month. That is why this is not always [`Nengo::western_year`]: 安政
    /// was proclaimed on 嘉永7年11月27日, which is 1855-01-15 in the West,
    /// but the lunisolar year it fell in had begun in 1854, so 安政元年 is
    /// year 1854 here.
    ///
    /// Almost every pre-Meiji era change was 年初改元 — proclaimed part way
    /// through a year but backdated to its first day — so the era's year 1
    /// really is the whole of this year.
    pub start_year: i64,
    /// The Western-calendar year of the changeover, in [`Nengo::scale`].
    pub western_year: i64,
    /// The Western-calendar month of the changeover.
    pub western_month: u8,
    /// The Western-calendar day of the changeover, or `0` when
    /// [`Nengo::certainty`] is [`Certainty::MonthOnly`].
    pub western_day: u8,
    /// Which Western calendar the three fields above are expressed in.
    pub scale: WesternScale,
    /// How firmly the sources fix the changeover.
    pub certainty: Certainty,
    /// The fixed day of the changeover, absent when the day is unknown.
    ///
    /// For 明治 through 令和 this is the 公式 date — the one a Japanese
    /// government document uses, under which an era ends the day before the
    /// next begins. 明治 therefore starts at 明治元年1月1日 = 1868-01-25
    /// rather than at the proclamation of 明治元年9月8日 = 1868-10-23, and
    /// 大正 starts on 1912-07-30, 明治 having officially ended on
    /// 1912-07-29. The 改元当時 convention, under which the changeover day
    /// belongs to both eras, is a different table and this crate does not
    /// carry it.
    pub start: Option<Rd>,
}

impl Nengo {
    /// The fixed day the era began, when it is known.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::UnknownEra`] for an era whose start is only
    /// known to the month.
    pub const fn require_start(&self) -> CalendarResult<Rd> {
        match self.start {
            Some(rd) => Ok(rd),
            None => Err(CalendarError::UnknownEra),
        }
    }

    /// Whether this era belongs to the stream a given court used.
    ///
    /// An era proclaimed by one court is not in the other court's stream;
    /// a [`Court::Unified`] era is in every stream.
    #[must_use]
    pub const fn used_by(&self, court: Court) -> bool {
        matches!(
            (self.court, court),
            (Court::Unified, _)
                | (Court::Northern, Court::Northern)
                | (Court::Southern, Court::Southern)
        )
    }
}

/// 元弘元年 (元徳3年8月9日) = 1331-09-11 Julian, when Go-Daigo proclaimed
/// 元弘 and the Kamakura shogunate refused to recognise it.
pub const NANBOKUCHO_START: Rd = Rd(486_034);

/// 元中9年閏10月5日 = 1392-11-19 Julian, when Go-Kameyama abdicated and the
/// two courts were reunited under the Northern court's era 明徳.
pub const NANBOKUCHO_END: Rd = Rd(508_384);

/// The index of 明治 in [`ALL`].
pub const MEIJI: usize = 243;
/// The index of 大正 in [`ALL`].
pub const TAISHO: usize = 244;
/// The index of 昭和 in [`ALL`].
pub const SHOWA: usize = 245;
/// The index of 平成 in [`ALL`].
pub const HEISEI: usize = 246;
/// The index of 令和 in [`ALL`].
pub const REIWA: usize = 247;

/// 明治, in force from 1868-01-25 to 1912-07-29.
#[must_use]
pub fn meiji() -> &'static Nengo {
    &ALL[MEIJI]
}

/// 大正, in force from 1912-07-30 to 1926-12-24.
#[must_use]
pub fn taisho() -> &'static Nengo {
    &ALL[TAISHO]
}

/// 昭和, in force from 1926-12-25 to 1989-01-07.
#[must_use]
pub fn showa() -> &'static Nengo {
    &ALL[SHOWA]
}

/// 平成, in force from 1989-01-08 to 2019-04-30.
#[must_use]
pub fn heisei() -> &'static Nengo {
    &ALL[HEISEI]
}

/// 令和, in force since 2019-05-01.
#[must_use]
pub fn reiwa() -> &'static Nengo {
    &ALL[REIWA]
}

/// Whether `rd` falls in the period when two courts ran two era systems.
#[must_use]
pub const fn is_nanbokucho(rd: Rd) -> bool {
    rd.0 >= NANBOKUCHO_START.0 && rd.0 < NANBOKUCHO_END.0
}

/// Look an era up by its identifier, kanji, hiragana reading or Hepburn
/// romanisation.
///
/// A bare romanisation or reading that collides — `"Showa"` and `"しょうわ"`
/// are both 正和 (1312) and 昭和 (1926) — resolves to whichever era kept the
/// plain identifier, here 昭和; use the kanji or the year-suffixed
/// identifier (`showa-1312`) to name the other one.
///
/// Some collisions have no winner: 弘安 and 康安 are both `Koan`, and the
/// table gives both a year suffix, so neither kept a plain `koan`. There the
/// earlier era answers and the caller has to be explicit.
///
/// The passes matter. Identifiers and kanji are unique across the whole
/// table, so they are searched first and completely; only then are the
/// ambiguous romanisations and readings tried. Searching all four fields in
/// one pass over a chronologically ordered table would hand every collision
/// to the *earlier* era, which for 昭和 is six centuries wrong.
#[must_use]
pub fn find(name: &str) -> Option<&'static Nengo> {
    if let Some(exact) = ALL.iter().find(|era| era.id == name || era.kanji == name) {
        return Some(exact);
    }
    let mut ambiguous = ALL
        .iter()
        .filter(|era| era.reading == name || era.romaji == name);
    let first = ambiguous.next()?;
    if !is_year_suffixed(first.id) {
        return Some(first);
    }
    // The first match carries a year suffix, so it is the era that lost the
    // plain identifier. Look for the one that kept it before settling.
    Some(
        ambiguous
            .find(|era| !is_year_suffixed(era.id))
            .unwrap_or(first),
    )
}

/// Whether an identifier carries the `-YYYY` suffix that marks the era which
/// lost a romanisation collision, as `showa-1312` did to `showa`.
fn is_year_suffixed(id: &str) -> bool {
    match id.rsplit_once('-') {
        Some((_, tail)) => tail.len() == 4 && tail.bytes().all(|byte| byte.is_ascii_digit()),
        None => false,
    }
}

/// Look an era up by its kanji, which is unique across the whole table.
#[must_use]
pub fn by_kanji(kanji: &str) -> Option<&'static Nengo> {
    ALL.iter().find(|era| era.kanji == kanji)
}

/// Every era one court used, in chronological order.
pub fn stream(court: Court) -> impl Iterator<Item = &'static Nengo> {
    ALL.iter().filter(move |era| era.used_by(court))
}

/// The era in force on `rd` according to `court`.
///
/// # Errors
///
/// Returns [`CalendarError::UnknownEra`] when `court` is
/// [`Court::Unified`] inside the Northern-and-Southern-Courts period, where
/// there is no single answer — ask for [`Court::Northern`] or
/// [`Court::Southern`] instead. Returns [`CalendarError::BeforeEpoch`] for
/// a day before 大化.
///
/// # After the reunification
///
/// The union of 1392 took the form of the Southern emperor abdicating to
/// the Northern one, so it was the Northern court's era 明徳 that carried
/// on. A [`Court::Unified`] lookup after [`NANBOKUCHO_END`] therefore reads
/// the Northern stream; from 応永 (1394) onward the two are the same list.
///
/// # A warning about the answer
///
/// That an era was in force on a day is not the same as the day being
/// convertible. This function will say that 延暦 was in force in 800 CE;
/// [`crate::japanese::JapaneseCalendar`] still refuses to say which day of
/// which month that was, because the lunisolar calendar Japan used then is
/// not implemented anywhere in this workspace. See that module for exactly
/// where the line falls.
pub fn era_at(rd: Rd, court: Court) -> CalendarResult<&'static Nengo> {
    let court = match court {
        Court::Unified if is_nanbokucho(rd) => return Err(CalendarError::UnknownEra),
        Court::Unified if rd >= NANBOKUCHO_END => Court::Northern,
        other => other,
    };
    let mut found: Option<&'static Nengo> = None;
    for era in stream(court) {
        match era.start {
            Some(start) if start <= rd => found = Some(era),
            Some(_) => break,
            // An era whose day is unknown cannot bound anything, so the era
            // before it stays in force until the next dated one.
            None => {}
        }
    }
    found.ok_or(CalendarError::BeforeEpoch)
}

/// The next dated era after `era` in `court`'s stream, if any.
///
/// Undated eras are skipped, because an era with no start day cannot mark
/// the end of the one before it.
#[must_use]
pub fn next_in_stream(era: &Nengo, court: Court) -> Option<&'static Nengo> {
    let mut seen = false;
    for candidate in stream(court) {
        if seen && candidate.start.is_some() {
            return Some(candidate);
        }
        if candidate.kanji == era.kanji && candidate.court == era.court {
            seen = true;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use hc_calendars_solar::{gregorian, julian};

    use super::*;

    /// The identifier and kanji namespaces have to be unique for `find` to
    /// be able to resolve a collision at all, so that is checked here rather
    /// than assumed by the function that relies on it.
    #[test]
    fn identifiers_and_kanji_are_unique_across_the_table() {
        for (index, era) in ALL.iter().enumerate() {
            for other in &ALL[index + 1..] {
                assert_ne!(era.id, other.id, "duplicate id {}", era.id);
                assert_ne!(era.kanji, other.kanji, "duplicate kanji {}", era.kanji);
            }
        }
    }

    /// Twelve romanisations and twelve readings are shared by two eras each.
    /// A single pass over the chronologically ordered table would resolve
    /// every one of them to the earlier era.
    #[test]
    fn a_colliding_romanisation_resolves_to_the_era_that_kept_the_plain_id() {
        // 昭和 (1926) keeps `showa`; 正和 (1312) is `showa-1312`.
        let showa = find("Showa").expect("Showa is an era");
        assert_eq!(showa.kanji, "昭和");
        assert_eq!(showa.id, "showa");
        assert_eq!(find("しょうわ").map(|era| era.kanji), Some("昭和"));
        assert_eq!(find("showa-1312").map(|era| era.kanji), Some("正和"));
        assert_eq!(find("正和").map(|era| era.id), Some("showa-1312"));

        // 弘安 (1278) and 康安 (1361) are both `Koan` and the table suffixes
        // both, so there is no plain identifier to prefer and the earlier era
        // answers. Asserted so that giving one of them a plain `koan` later
        // is a deliberate change rather than a silent one.
        assert_eq!(find("Koan").map(|era| era.id), Some("koan-1278"));
        assert_eq!(find("康安").map(|era| era.id), Some("koan-1361"));
    }

    /// The property the two passes are for, over the whole table: a bare id
    /// always finds the era that owns it, whatever else shares its
    /// romanisation.
    #[test]
    fn every_era_is_findable_by_its_own_identifier() {
        for era in ALL {
            assert_eq!(
                find(era.id).map(|found| found.kanji),
                Some(era.kanji),
                "{} did not find itself",
                era.id
            );
            assert_eq!(find(era.kanji).map(|found| found.id), Some(era.id));
        }
    }

    #[test]
    fn the_table_holds_all_two_hundred_and_forty_eight_eras() {
        // ja.wikipedia's 元号一覧 (日本) states that 令和 is the 248th era.
        assert_eq!(ALL.len(), 248);
        assert_eq!(ALL[0].kanji, "大化");
        assert_eq!(ALL[ALL.len() - 1].kanji, "令和");
    }

    #[test]
    fn every_identifier_is_distinct_and_ascii() {
        for (index, era) in ALL.iter().enumerate() {
            assert!(
                era.id.is_ascii() && !era.id.is_empty(),
                "{} has a non-ascii id",
                era.kanji
            );
            assert!(era.romaji.is_ascii(), "{}", era.kanji);
            for other in &ALL[index + 1..] {
                assert_ne!(era.id, other.id, "duplicate id {}", era.id);
                assert_ne!(era.kanji, other.kanji, "duplicate kanji {}", era.kanji);
            }
        }
    }

    #[test]
    fn every_stored_fixed_day_matches_its_tabulated_western_date() {
        // The point of storing both: the fixed day is what the code uses,
        // the Western date is what a reader can check against the source,
        // so if they disagree the table is wrong.
        for era in &ALL {
            match era.start {
                Some(start) => {
                    let computed = match era.scale {
                        WesternScale::Gregorian => gregorian::to_fixed(
                            era.western_year,
                            era.western_month,
                            era.western_day,
                        ),
                        WesternScale::Julian => {
                            julian::to_fixed(era.western_year, era.western_month, era.western_day)
                        }
                    };
                    assert_eq!(computed, Ok(start), "{} disagrees", era.kanji);
                }
                None => {
                    assert_eq!(era.certainty, Certainty::MonthOnly, "{}", era.kanji);
                    assert_eq!(era.western_day, 0, "{}", era.kanji);
                }
            }
        }
    }

    #[test]
    fn the_julian_gregorian_switch_happens_where_the_source_says() {
        let tensho = by_kanji("天正").expect("in table");
        assert_eq!(tensho.scale, WesternScale::Julian);
        assert_eq!(
            (
                tensho.western_year,
                tensho.western_month,
                tensho.western_day
            ),
            (1573, 8, 25)
        );
        let bunroku = by_kanji("文禄").expect("in table");
        assert_eq!(bunroku.scale, WesternScale::Gregorian);
        assert_eq!(
            (
                bunroku.western_year,
                bunroku.western_month,
                bunroku.western_day
            ),
            (1593, 1, 10)
        );
        for era in &ALL {
            let julian = era.scale == WesternScale::Julian;
            assert_eq!(julian, era.western_year <= 1573, "{}", era.kanji);
        }
    }

    #[test]
    fn each_court_stream_runs_forward_in_time() {
        for court in [Court::Unified, Court::Northern, Court::Southern] {
            let mut previous: Option<Rd> = None;
            for era in stream(court) {
                if let Some(start) = era.start {
                    if let Some(earlier) = previous {
                        assert!(earlier < start, "{} goes backwards", era.kanji);
                    }
                    previous = Some(start);
                }
            }
        }
    }

    #[test]
    fn the_modern_eras_start_on_the_days_the_government_says() {
        for (era, (year, month, day)) in [
            (meiji(), (1868, 1, 25)),
            (taisho(), (1912, 7, 30)),
            (showa(), (1926, 12, 25)),
            (heisei(), (1989, 1, 8)),
            (reiwa(), (2019, 5, 1)),
        ] {
            assert_eq!(era.scale, WesternScale::Gregorian);
            assert_eq!(era.certainty, Certainty::Attested);
            assert_eq!(
                era.start,
                Some(gregorian::to_fixed(year, month, day).expect("valid"))
            );
            assert_eq!(era.start_year, year);
            assert_eq!(era.court, Court::Unified);
        }
        assert_eq!(reiwa().kanji, "令和");
        assert_eq!(reiwa().romaji, "Reiwa");
        assert_eq!(showa().kanji, "昭和");
        assert_eq!(meiji().kanji, "明治");
        assert_eq!(taisho().kanji, "大正");
        assert_eq!(heisei().kanji, "平成");
    }

    #[test]
    fn the_first_era_is_the_one_whose_start_is_disputed() {
        // 『扶桑略記』 gives the seventh month, 『元亨釈書』 the first and
        // 『国史大辞典』 the 19th of the sixth; the table takes the
        // conventional 大化元年7月1日 and flags it as disputed.
        let taika = &ALL[0];
        assert_eq!(taika.kanji, "大化");
        assert_eq!(taika.certainty, Certainty::Disputed);
        assert_eq!(taika.start_year, 645);
        assert_eq!(taika.start, julian::to_fixed(645, 7, 29).ok());
    }

    #[test]
    fn the_one_era_known_only_to_the_month_has_no_start_day() {
        let bunchu = by_kanji("文中").expect("in table");
        assert_eq!(bunchu.certainty, Certainty::MonthOnly);
        assert_eq!(bunchu.court, Court::Southern);
        assert_eq!((bunchu.western_year, bunchu.western_month), (1372, 5));
        assert_eq!(bunchu.start, None);
        assert_eq!(bunchu.require_start(), Err(CalendarError::UnknownEra));
        assert_eq!(
            ALL.iter().filter(|era| era.start.is_none()).count(),
            1,
            "only 文中 should be undated"
        );
    }

    #[test]
    fn the_southern_court_eras_after_kentoku_are_all_flagged_uncertain() {
        // The source says outright that no promulgation records survive for
        // 建徳 onward and that it is following 『続史愚抄』 and
        // 『南朝公卿補任』 by convention.
        for kanji in ["建徳", "文中", "天授", "弘和", "元中"] {
            let era = by_kanji(kanji).expect("in table");
            assert_ne!(era.certainty, Certainty::Attested, "{kanji}");
            assert_eq!(era.court, Court::Southern);
        }
    }

    #[test]
    fn the_two_courts_are_both_represented() {
        assert_eq!(stream(Court::Unified).count(), 222);
        assert_eq!(stream(Court::Northern).count(), 222 + 17);
        assert_eq!(stream(Court::Southern).count(), 222 + 9);
        assert_eq!(by_kanji("正慶").expect("in table").court, Court::Northern);
        assert_eq!(by_kanji("元弘").expect("in table").court, Court::Southern);
        assert_eq!(by_kanji("建武").expect("in table").court, Court::Unified);
        assert_eq!(by_kanji("明徳").expect("in table").court, Court::Northern);
    }

    #[test]
    fn the_unified_stream_refuses_the_two_court_period() {
        let midway = julian::to_fixed(1360, 1, 1).expect("valid");
        assert!(is_nanbokucho(midway));
        assert_eq!(
            era_at(midway, Court::Unified),
            Err(CalendarError::UnknownEra)
        );
        assert_eq!(era_at(midway, Court::Northern).map(|e| e.kanji), Ok("延文"));
        assert_eq!(era_at(midway, Court::Southern).map(|e| e.kanji), Ok("正平"));
    }

    #[test]
    fn the_two_court_period_starts_and_ends_where_the_sources_say() {
        assert_eq!(
            NANBOKUCHO_START,
            julian::to_fixed(1331, 9, 11).expect("valid")
        );
        assert_eq!(
            NANBOKUCHO_END,
            julian::to_fixed(1392, 11, 19).expect("valid")
        );
        assert!(!is_nanbokucho(Rd(NANBOKUCHO_START.0 - 1)));
        assert!(is_nanbokucho(NANBOKUCHO_START));
        assert!(!is_nanbokucho(NANBOKUCHO_END));
        // The Northern court's era carried on through the union.
        assert_eq!(
            era_at(NANBOKUCHO_END, Court::Unified).map(|e| e.kanji),
            Ok("明徳")
        );
        assert_eq!(
            era_at(Rd(NANBOKUCHO_END.0 - 1), Court::Southern).map(|e| e.kanji),
            Ok("元中")
        );
    }

    #[test]
    fn era_lookup_finds_the_era_in_force() {
        let cases = [
            ((2026, 9, 21), "令和"),
            ((2019, 5, 1), "令和"),
            ((2019, 4, 30), "平成"),
            ((1989, 1, 8), "平成"),
            ((1989, 1, 7), "昭和"),
            ((1926, 12, 25), "昭和"),
            ((1926, 12, 24), "大正"),
            ((1912, 7, 30), "大正"),
            ((1912, 7, 29), "明治"),
            ((1868, 1, 25), "明治"),
            ((1868, 1, 24), "慶応"),
            ((1873, 1, 1), "明治"),
        ];
        for ((year, month, day), kanji) in cases {
            let rd = gregorian::to_fixed(year, month, day).expect("valid");
            assert_eq!(
                era_at(rd, Court::Unified).map(|era| era.kanji),
                Ok(kanji),
                "{year}-{month}-{day}"
            );
        }
    }

    #[test]
    fn days_before_the_first_era_have_no_era() {
        let before = Rd(ALL[0].start.expect("dated").0 - 1);
        assert_eq!(
            era_at(before, Court::Unified),
            Err(CalendarError::BeforeEpoch)
        );
    }

    #[test]
    fn eras_can_be_found_by_every_spelling_they_carry() {
        assert_eq!(find("reiwa").map(|era| era.kanji), Some("令和"));
        assert_eq!(find("令和").map(|era| era.kanji), Some("令和"));
        assert_eq!(find("れいわ").map(|era| era.kanji), Some("令和"));
        assert_eq!(find("Reiwa").map(|era| era.kanji), Some("令和"));
        assert_eq!(find("no-such-era"), None);
        assert_eq!(by_kanji("正和").map(|era| era.id), Some("showa-1312"));
        assert_eq!(by_kanji("昭和").map(|era| era.id), Some("showa"));
        assert_eq!(by_kanji("no-such-era"), None);
    }

    #[test]
    fn the_next_era_in_a_stream_is_the_next_dated_one() {
        assert_eq!(
            next_in_stream(heisei(), Court::Unified).map(|era| era.kanji),
            Some("令和")
        );
        assert_eq!(next_in_stream(reiwa(), Court::Unified), None);
        // 文中 is undated, so for lookup purposes 建徳's successor is the
        // next Southern era that does have a day.
        assert_eq!(
            next_in_stream(by_kanji("建徳").expect("in table"), Court::Southern)
                .map(|era| era.kanji),
            Some("天授")
        );
    }

    #[test]
    fn era_year_numbering_follows_the_lunisolar_year_not_the_western_one() {
        // 安政 was proclaimed on 嘉永7年11月27日 = 1855-01-15, but that day
        // belongs to the lunisolar year that began in 1854.
        let ansei = by_kanji("安政").expect("in table");
        assert_eq!(ansei.western_year, 1855);
        assert_eq!(ansei.start_year, 1854);
        let koka = by_kanji("弘化").expect("in table");
        assert_eq!((koka.western_year, koka.western_month), (1845, 1));
        assert_eq!(koka.start_year, 1844);
        // Where the change fell early in the year the two agree.
        let kaei = by_kanji("嘉永").expect("in table");
        assert_eq!(kaei.western_year, 1848);
        assert_eq!(kaei.start_year, 1848);
    }

    #[test]
    fn every_era_carries_a_reading_and_a_romanisation() {
        for era in &ALL {
            assert!(!era.reading.is_empty(), "{}", era.kanji);
            assert!(!era.romaji.is_empty(), "{}", era.kanji);
            assert!(
                era.romaji.starts_with(|c: char| c.is_ascii_uppercase()),
                "{} romanises as {}",
                era.kanji,
                era.romaji
            );
            // The identifier is the romanisation lower-cased and stripped
            // of apostrophes, possibly with a disambiguating year appended.
            let mut actual = era.id.chars();
            for expected in era
                .romaji
                .chars()
                .filter(|c| *c != '\'')
                .map(|c| c.to_ascii_lowercase())
            {
                assert_eq!(
                    actual.next(),
                    Some(expected),
                    "{} has id {} but romaji {}",
                    era.kanji,
                    era.id,
                    era.romaji
                );
            }
            let suffix = actual.as_str();
            assert!(
                suffix.is_empty() || suffix.starts_with('-'),
                "{} has id {}",
                era.kanji,
                era.id
            );
            assert!(!era.kanji.is_empty());
            assert!(era.start_year > 600, "{}", era.kanji);
        }
    }

    #[test]
    fn a_nengo_is_in_its_own_courts_stream_and_not_the_others() {
        let genko = by_kanji("元弘").expect("in table");
        assert!(genko.used_by(Court::Southern));
        assert!(!genko.used_by(Court::Northern));
        assert!(!genko.used_by(Court::Unified));
        let kenmu = by_kanji("建武").expect("in table");
        assert!(kenmu.used_by(Court::Southern));
        assert!(kenmu.used_by(Court::Northern));
        assert!(kenmu.used_by(Court::Unified));
    }
}

hc_core::catalogue_tests! {
    type: Nengo,
    id: |era| era.id,
    tests: era_table_tests,
    all: &ALL,
    lookup: find,
}
