//! Repeating named cycles: the sexagenary stems and branches, the four
//! pillars built from them, and the general machinery behind any other
//! cyclic naming scheme.
//!
//! Cycles are the clearest case of the data/algorithm split this library is
//! built on. "Which of sixty names does this year have" is one modulo; the
//! sixty names themselves are data that varies by culture and script. So the
//! arithmetic lives here, and the *locale-tagged* names live in `hc-i18n`.
//!
//! # The sixty-term cycle
//!
//! Ten Heavenly Stems (十干) and twelve Earthly Branches (十二支) advance in
//! step. Because ten and twelve share a factor of two, only sixty of the 120
//! possible pairs occur, and a pair repeats every sixty steps — the 干支
//! (*ganzhi*, *kanshi*, *eto*, *gapja*, *can chi*) cycle that East Asia has
//! used to name years, months, days and hours for more than three thousand
//! years.
//!
//! # The four pillars
//!
//! 四柱 (four pillars), also 八字 (eight characters, being four stems plus
//! four branches), is one stem-branch pair each for the year, the month, the
//! day and the double-hour of a moment. [`FourPillars`] holds all four.
//!
//! What makes the four pillars hard is not the naming but the *boundaries*.
//! Each pillar changes at a different instant, and none of them changes at
//! midnight on 1 January:
//!
//! | Pillar | Changes at | Computed by |
//! |---|---|---|
//! | 年柱 year | 立春, or the lunar new year, or 1 January — three rival conventions | [`sexagenary_year`], [`sexagenary_year_from_solar_term_year`], [`sexagenary_year_from_gregorian_year`] |
//! | 月柱 month | the twelve 節気 (立春, 驚蟄, 清明 …), *not* the new moon | [`month_pillar`] |
//! | 日柱 day | 23:00 under the majority school, midnight under the other | [`pillar_day`] |
//! | 時柱 hour | every two hours from 23:00 | [`hour_pillar`] |
//!
//! The day pillar is the only one of the four that needs no calendar at all:
//! the sixty-day cycle has run without interruption for longer than any
//! surviving calendar, so [`sexagenary_day`] is a function of [`Rd`] alone.
//!
//! # What this module deliberately does not do
//!
//! It computes no astronomy. The month pillar is fixed by the solar terms and
//! the four-pillar year by 立春, and both need the apparent longitude of the
//! Sun. `hc-calendar` depends on nothing but `hc-core` and is not going to
//! grow an ephemeris, so every function that needs a solar term takes the
//! term as an argument, and the documentation says which `hc-seasons` call
//! produces it. That crate depends on this one; the arrow cannot be reversed.
//!
//! It also knows nothing about time zones or about the meridian a tradition
//! reckons by. Pillars are computed from *local* civil time — which local
//! time is a question for `hc-tz` and for the calendar's own meridian table
//! (see `hc-calendars-lunar`'s `MeridianEra`), and a birth chart computed on
//! the wrong meridian is wrong by up to an hour, which is a whole pillar.
//!
//! # Names and scripts
//!
//! The sixty names are spelled differently in every language that uses
//! them, and the list of spellings has no end. [`readings`] is the
//! catalogue: one [`readings::Reading`] per script or romanisation — the
//! characters, pinyin with and without tones, the Japanese 訓読み and
//! 音読み, Hangul and its romanisation, Vietnamese — each holding exactly
//! ten stems and twelve branches. [`Sexagenary`]'s own `stem_name` and
//! `branch_name` are the toneless pinyin, this library's convention for
//! English text. `hc-i18n` chooses a reading per locale, and holds the
//! zodiac animals, which are words of a language rather than readings of
//! the cycle.

use core::fmt;

use crate::error::{CalendarError, CalendarResult};
use crate::fixed::Rd;
use crate::time::{CivilDateTime, CivilTime};

pub mod readings;

/// The twelve zodiac animals, in branch order, in English.
///
/// English only, and deliberately: the animals are words of a language —
/// 兔 in traditional Chinese, 兎 in Japanese, 토끼 in Korean, and the cat
/// rather than the rabbit in Vietnamese — so every other spelling is the
/// locale's and lives in `hc-i18n`. The characters for the stems, branches
/// and phases are shared by every language that uses them, which is why
/// those are here and these are not.
pub const ZODIAC_ANIMALS: [&str; 12] = [
    "rat", "ox", "tiger", "rabbit", "dragon", "snake", "horse", "goat", "monkey", "rooster", "dog",
    "pig",
];

/// The five phases, in stem-pair order.
pub const FIVE_PHASES: [&str; 5] = ["wood", "fire", "earth", "metal", "water"];

/// The five phases (五行) in characters, in stem-pair order.
pub const FIVE_PHASES_CJK: [&str; 5] = ["木", "火", "土", "金", "水"];

/// The five phases in Hanyu Pinyin with tone marks.
pub const FIVE_PHASES_PINYIN: [&str; 5] = ["mù", "huǒ", "tǔ", "jīn", "shuǐ"];

/// The five phases in their Japanese 音読み, Hepburn romanised.
///
/// These are the 呉音 readings used when the five are recited as a set
/// (もく・か・ど・ごん・すい); 金 in isolation, as in 金曜日, is *kin*.
pub const FIVE_PHASES_JAPANESE_ON: [&str; 5] = ["moku", "ka", "do", "gon", "sui"];

/// The five phases in their Japanese 訓読み, Hepburn romanised.
///
/// This is the table that builds the stem readings: phase plus 兄 or 弟 gives
/// [`readings::JAPANESE_KUN`] exactly.
pub const FIVE_PHASES_JAPANESE_KUN: [&str; 5] = ["ki", "hi", "tsuchi", "ka", "mizu"];

/// The classical names of the twelve double-hours (十二時辰), in branch
/// order from 子.
///
/// These are the descriptive names used in Han-period and later texts —
/// 夜半 "midnight", 雞鳴 "cockcrow", 平旦 "dawn" — rather than the branch
/// names, and they are what an almanac prints beside the hour. Traditional
/// characters.
pub const DOUBLE_HOUR_CLASSICAL_NAMES_CJK: [&str; 12] = [
    "夜半", "雞鳴", "平旦", "日出", "食時", "隅中", "日中", "日昳", "晡時", "日入", "黃昏", "人定",
];

/// Yin or yang (陰陽): the polarity of a stem, a branch or a pillar.
///
/// Polarity is nothing but index parity. Stems and branches alternate
/// strictly — 甲 yang, 乙 yin, 丙 yang — so the even positions are yang and
/// the odd ones yin, and because a stem and a branch advance together, the
/// two halves of a pillar always agree. That agreement is the reason only
/// sixty of the 120 pairs exist.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Polarity {
    /// 陽, yang: the even positions, and the 兄 ("elder brother") half of a
    /// Japanese stem reading.
    Yang,
    /// 陰, yin: the odd positions, and the 弟 ("younger brother") half.
    Yin,
}

impl Polarity {
    /// The polarity of a zero-based position in any of these cycles.
    #[must_use]
    pub const fn of_index(index: u8) -> Self {
        if index.is_multiple_of(2) {
            Self::Yang
        } else {
            Self::Yin
        }
    }

    /// Whether this is yang.
    #[must_use]
    pub const fn is_yang(self) -> bool {
        matches!(self, Self::Yang)
    }

    /// The romanised name.
    #[must_use]
    pub const fn english_name(self) -> &'static str {
        match self {
            Self::Yang => "yang",
            Self::Yin => "yin",
        }
    }

    /// The character, 陽 or 陰.
    #[must_use]
    pub const fn cjk_name(self) -> &'static str {
        match self {
            Self::Yang => "陽",
            Self::Yin => "陰",
        }
    }

    /// The Japanese 訓読み suffix, 兄 (*e*) or 弟 (*to*).
    #[must_use]
    pub const fn japanese_kun(self) -> &'static str {
        match self {
            Self::Yang => "e",
            Self::Yin => "to",
        }
    }
}

impl fmt::Display for Polarity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.english_name())
    }
}

/// A position in the sixty-term sexagenary cycle (干支 / ganzhi / eto).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sexagenary {
    /// The zero-based index within the sixty-term cycle.
    index: u8,
}

impl Sexagenary {
    /// Build from a zero-based index within the cycle.
    ///
    /// Indices outside `0..60` wrap, because a cycle position is by
    /// definition modular.
    #[must_use]
    pub const fn from_index(index: i64) -> Self {
        Self {
            index: index.rem_euclid(60) as u8,
        }
    }

    /// Build from a one-based stem and branch, as the pair is usually cited.
    ///
    /// Only 60 of the 120 stem-branch pairs occur: the stem and the branch
    /// advance together, so their indices always share a parity. Returns
    /// `None` for an impossible pair such as "jia-chou".
    #[must_use]
    pub const fn from_stem_branch(stem: u8, branch: u8) -> Option<Self> {
        if stem == 0 || stem > 10 || branch == 0 || branch > 12 {
            return None;
        }
        Self::from_stem_branch_index(stem - 1, branch - 1)
    }

    /// Build from zero-based stem and branch indices.
    ///
    /// The index is the Chinese-remainder solution of `x ≡ stem (mod 10)` and
    /// `x ≡ branch (mod 12)`, which is `6·stem − 5·branch` modulo 60 whenever
    /// the two indices share a parity, and undefined — `None` here — when
    /// they do not.
    #[must_use]
    pub const fn from_stem_branch_index(stem_index: u8, branch_index: u8) -> Option<Self> {
        if stem_index > 9 || branch_index > 11 || stem_index % 2 != branch_index % 2 {
            return None;
        }
        let index = (6 * stem_index as i64 - 5 * branch_index as i64).rem_euclid(60);
        Some(Self { index: index as u8 })
    }

    /// The zero-based index within the cycle.
    #[must_use]
    pub const fn index(self) -> u8 {
        self.index
    }

    /// The one-based ordinal, as tables usually number it.
    #[must_use]
    pub const fn ordinal(self) -> u8 {
        self.index + 1
    }

    /// The zero-based Heavenly Stem.
    #[must_use]
    pub const fn stem_index(self) -> u8 {
        self.index % 10
    }

    /// The zero-based Earthly Branch.
    #[must_use]
    pub const fn branch_index(self) -> u8 {
        self.index % 12
    }

    /// The stem in toneless pinyin, [`readings::PINYIN`]. Every other
    /// spelling is a [`readings::Reading`].
    #[must_use]
    pub const fn stem_name(self) -> &'static str {
        readings::PINYIN.stem(self)
    }

    /// The branch in toneless pinyin, [`readings::PINYIN`].
    #[must_use]
    pub const fn branch_name(self) -> &'static str {
        readings::PINYIN.branch(self)
    }

    /// The zodiac animal associated with the branch.
    #[must_use]
    pub const fn zodiac_animal(self) -> &'static str {
        ZODIAC_ANIMALS[(self.index % 12) as usize]
    }

    /// The five-phase element associated with the stem.
    #[must_use]
    pub const fn five_phase(self) -> &'static str {
        FIVE_PHASES[((self.index % 10) / 2) as usize]
    }

    /// The five-phase character associated with the stem.
    #[must_use]
    pub const fn five_phase_cjk(self) -> &'static str {
        FIVE_PHASES_CJK[((self.index % 10) / 2) as usize]
    }

    /// The zero-based five-phase index of the stem.
    #[must_use]
    pub const fn five_phase_index(self) -> u8 {
        (self.index % 10) / 2
    }

    /// The polarity of the stem.
    #[must_use]
    pub const fn stem_polarity(self) -> Polarity {
        Polarity::of_index(self.stem_index())
    }

    /// The polarity of the branch.
    ///
    /// Always equal to [`Sexagenary::stem_polarity`]; both are offered
    /// because a caller reasoning about one half of a pillar should not have
    /// to know that.
    #[must_use]
    pub const fn branch_polarity(self) -> Polarity {
        Polarity::of_index(self.branch_index())
    }

    /// The polarity of the pair.
    #[must_use]
    pub const fn polarity(self) -> Polarity {
        Polarity::of_index(self.index)
    }

    /// The next position in the cycle.
    #[must_use]
    pub const fn next(self) -> Self {
        Self::from_index(self.index as i64 + 1)
    }

    /// The previous position in the cycle.
    #[must_use]
    pub const fn previous(self) -> Self {
        Self::from_index(self.index as i64 - 1)
    }

    /// The position `steps` further on, backwards for a negative `steps`.
    #[must_use]
    pub const fn advance(self, steps: i64) -> Self {
        Self::from_index(self.index as i64 + steps)
    }
}

impl fmt::Display for Sexagenary {
    /// Renders as the two romanised halves joined by a hyphen, `jia-zi`.
    /// Scripted output needs `hc-i18n`, which knows which characters a locale
    /// wants and whether to separate them.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.stem_name(), self.branch_name())
    }
}

/// One of the twelve double-hours (十二時辰) that divide a traditional day.
///
/// Index 0 is 子, which runs from 23:00 to 01:00 — *not* from midnight. The
/// traditional day is centred on midnight rather than started by it, so the
/// first double-hour straddles the civil date boundary, and that one hour is
/// the source of most of the disagreement in four-pillar software. See
/// [`ZiHourConvention`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DoubleHour {
    /// The zero-based index, 0 being 子.
    index: u8,
}

impl DoubleHour {
    /// Build from a zero-based index; values outside `0..12` wrap.
    #[must_use]
    pub const fn from_index(index: i64) -> Self {
        Self {
            index: index.rem_euclid(12) as u8,
        }
    }

    /// The double-hour containing a civil hour of the day.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::DayOutOfRange`] when `hour` is not in
    /// `0..=23`.
    pub const fn from_hour_of_day(hour: u8) -> CalendarResult<Self> {
        if hour > 23 {
            return Err(CalendarError::DayOutOfRange);
        }
        // 子 opens at 23:00, so shifting the clock forward by one hour puts
        // each double-hour on an even boundary.
        Ok(Self {
            index: ((hour + 1) % 24) / 2,
        })
    }

    /// The double-hour containing a time of day.
    ///
    /// Minutes and seconds are irrelevant: the boundaries fall on the odd
    /// hours exactly. A leap second at 23:59:60 still belongs to 子, which is
    /// the only reason this cannot be written as a bare division.
    #[must_use]
    pub const fn containing(time: CivilTime) -> Self {
        Self {
            index: ((time.hour() + 1) % 24) / 2,
        }
    }

    /// The zero-based index, 0 being 子.
    #[must_use]
    pub const fn index(self) -> u8 {
        self.index
    }

    /// The Earthly Branch index of this double-hour, which is the index
    /// itself.
    #[must_use]
    pub const fn branch_index(self) -> u8 {
        self.index
    }

    /// The branch in toneless pinyin, [`readings::PINYIN`]. Every other
    /// spelling is a [`readings::Reading`], indexed by [`DoubleHour::branch_index`].
    #[must_use]
    pub const fn branch_name(self) -> &'static str {
        readings::PINYIN.branch_at(self.index)
    }

    /// The zodiac animal of this double-hour.
    #[must_use]
    pub const fn zodiac_animal(self) -> &'static str {
        ZODIAC_ANIMALS[self.index as usize]
    }

    /// The classical descriptive name, e.g. 夜半 for 子.
    #[must_use]
    pub const fn classical_name_cjk(self) -> &'static str {
        DOUBLE_HOUR_CLASSICAL_NAMES_CJK[self.index as usize]
    }

    /// The civil hour this double-hour begins at: 23 for 子, then 1, 3, 5 …
    #[must_use]
    pub const fn start_hour(self) -> u8 {
        (23 + 2 * self.index) % 24
    }

    /// The civil hour this double-hour ends at, exclusive.
    #[must_use]
    pub const fn end_hour(self) -> u8 {
        (1 + 2 * self.index) % 24
    }

    /// Whether a civil hour falls within this double-hour.
    #[must_use]
    pub const fn contains_hour(self, hour: u8) -> bool {
        hour <= 23 && ((hour + 1) % 24) / 2 == self.index
    }

    /// Whether this double-hour begins on the previous civil day.
    ///
    /// True only for 子, whose first hour is 23:00 of the day before.
    #[must_use]
    pub const fn starts_on_the_previous_civil_day(self) -> bool {
        self.index == 0
    }

    /// The next double-hour.
    #[must_use]
    pub const fn next(self) -> Self {
        Self::from_index(self.index as i64 + 1)
    }
}

impl fmt::Display for DoubleHour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-hour", self.branch_name())
    }
}

/// Which civil day a time between 23:00 and midnight belongs to, for the
/// purpose of the day pillar.
///
/// The two schools are both live and neither is a mistake; quoting a chart
/// without saying which one produced it is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ZiHourConvention {
    /// 早子時: the day pillar advances with the double-hour, at 23:00.
    ///
    /// The majority school in 四柱推命 and BaZi. A birth at 23:30 on the 1st
    /// takes the *2nd*'s day pillar, and its hour pillar is derived from that
    /// same day stem.
    DayStartsAtZiHour,
    /// 夜子時: the branch of the hour is 子, but the day pillar waits for
    /// midnight.
    ///
    /// The minority school, which keeps the day pillar aligned with the civil
    /// date and treats 23:00–24:00 as a "late 子" belonging to the old day.
    DayStartsAtMidnight,
}

/// The fixed day whose sexagenary term supplies the day pillar of a moment.
///
/// Under [`ZiHourConvention::DayStartsAtZiHour`] a time from 23:00 onwards
/// belongs to the next day; otherwise the civil day stands. The two agree for
/// twenty-three of the twenty-four hours, which is exactly why the
/// twenty-fourth goes unnoticed.
#[must_use]
pub const fn pillar_day(when: CivilDateTime, convention: ZiHourConvention) -> Rd {
    let advances = matches!(convention, ZiHourConvention::DayStartsAtZiHour);
    if advances && when.time.hour() == 23 {
        Rd(when.day.0 + 1)
    } else {
        when.day
    }
}

/// The sexagenary year of a Chinese-style calendar year number.
///
/// This is the **lunar new year** convention: the year named here changes at
/// 正月初一, the first day of the first lunisolar month, which falls between
/// 21 January and 21 February. It is the civil convention — the animal
/// celebrated at 春節, 설날 or Tết — and it is the one the lunisolar
/// calendars in `hc-calendars-lunar` use, passing the elapsed-year count of
/// the calendar in question.
///
/// Year 1 of the traditional reckoning is *jia-zi*, index 0. The commonly
/// used anchor is that the year which began on 1984-02-02 was *jia-zi*; that
/// year is numbered 4681 under the 2697 BCE (Huangdi) reckoning and 4621
/// under the 2637 BCE epoch of *Calendrical Calculations*, and since the two
/// differ by exactly sixty they name the same term.
///
/// For the four-pillar year, which changes at 立春 instead, use
/// [`sexagenary_year_from_solar_term_year`]. For the 1 January
/// approximation, [`sexagenary_year_from_gregorian_year`].
#[must_use]
pub const fn sexagenary_year(chinese_year: i64) -> Sexagenary {
    Sexagenary::from_index(chinese_year - 1)
}

/// The sexagenary year pillar of a **solar-term year**, the one 四柱推命 and
/// BaZi mean.
///
/// `solar_term_year` is the proleptic Gregorian year in which that year's
/// 立春 fell: the term year labelled 2024 runs from 立春 2024 (about
/// 4 February) to the instant before 立春 2025. Get the boundary itself from
/// `hc-seasons` and the label from [`solar_term_year`], which applies it.
///
/// # The three conventions, and which function is which
///
/// The year pillar has three rival boundaries in circulation. They agree for
/// roughly ten and a half months of every year and disagree over late January
/// and early February, which is where the bug always is:
///
/// | Boundary | Used by | Function |
/// |---|---|---|
/// | 立春, the solar term, ≈4 February | 四柱推命, BaZi, 節月 reckoning, almanacs | this one |
/// | 正月初一, the lunisolar new year, 21 Jan – 21 Feb | civil and festival use; the animal of 春節 | [`sexagenary_year`] |
/// | 1 January | newspapers, greeting cards, most software that never thought about it | [`sexagenary_year_from_gregorian_year`] |
///
/// All three run the same sixty names in the same order — the arithmetic is
/// identical and the labels coincide — so a function cannot tell you which
/// one a caller meant. Only the boundary differs, and only the boundary can
/// be got wrong. Someone born on 1985-02-10 was born in the 甲子 year by the
/// lunisolar reckoning (the new year came on 20 February 1985) and in the
/// 乙丑 year by the four-pillar one (立春 came on 4 February).
#[must_use]
pub const fn sexagenary_year_from_solar_term_year(solar_term_year: i64) -> Sexagenary {
    // Gregorian 4 CE is the jia-zi year of this reckoning, hence the −4; that
    // it agrees with the modern anchor is the check 1984 − 4 = 1980 = 33 × 60.
    Sexagenary::from_index(solar_term_year - 4)
}

/// The sexagenary year pillar under the 1 January convention.
///
/// This is the approximation printed on greeting cards and in newspaper
/// horoscopes, where the animal has to be known before the new moon or the
/// solar term arrives. It is the same arithmetic as
/// [`sexagenary_year_from_solar_term_year`] applied to a different boundary,
/// so it is right for about eleven months of the year and wrong for the weeks
/// between 1 January and 立春.
#[must_use]
pub const fn sexagenary_year_from_gregorian_year(gregorian_year: i64) -> Sexagenary {
    Sexagenary::from_index(gregorian_year - 4)
}

/// Which solar-term year a fixed day belongs to.
///
/// `start_of_spring` is the fixed day of 立春 of `gregorian_year`, which
/// `hc-seasons` computes: this crate has no ephemeris and will not gain one,
/// so the astronomy is the caller's to supply. Days before it belong to the
/// previous term year, which is why a birthday in late January carries the
/// *previous* year's pillar.
#[must_use]
pub const fn solar_term_year(rd: Rd, gregorian_year: i64, start_of_spring: Rd) -> i64 {
    if rd.0 < start_of_spring.0 {
        gregorian_year - 1
    } else {
        gregorian_year
    }
}

/// The four-pillar year term of a fixed day, given that year's 立春.
///
/// The composition of [`solar_term_year`] and
/// [`sexagenary_year_from_solar_term_year`], which is what a caller wanting a
/// birth chart actually needs.
#[must_use]
pub const fn sexagenary_year_at(rd: Rd, gregorian_year: i64, start_of_spring: Rd) -> Sexagenary {
    sexagenary_year_from_solar_term_year(solar_term_year(rd, gregorian_year, start_of_spring))
}

/// The sexagenary day of a fixed day.
///
/// The day cycle has run without interruption for longer than any surviving
/// calendar; the anchor used here is that RD 1 (`0001-01-01` proleptic
/// Gregorian) was *jia-zi* day index 14.
///
/// The same cycle is published as a Julian Day Number rule — stem
/// `(JDN + 9) mod 10`, branch `(JDN + 1) mod 12` — and the two agree
/// identically, since `JDN = RD + 1_721_425` and both constants reduce to
/// `+14`. It is anchored in the tests against 1900-01-01, a 甲戌 day, and
/// `hc-seasons` checks it a third way against published almanac dates for
/// 社日 and the 三伏, which are defined by the day stems.
///
/// This is the one pillar that needs no calendar, no meridian and no
/// astronomy — but see [`pillar_day`] for which fixed day a *time* belongs
/// to.
#[must_use]
pub const fn sexagenary_day(rd: Rd) -> Sexagenary {
    Sexagenary::from_index(rd.0 + 14)
}

/// The stem of the 子 hour of a day with the given day stem: the 五鼠遁 rule.
///
/// The mnemonic 五鼠遁日起時訣 runs 甲己還加甲, 乙庚丙作初, 丙辛從戊起,
/// 丁壬庚子居, 戊癸壬子是真途 — the five "rat" starting points, one per stem
/// pair, stepping by two stems each time. `day_stem` is reduced modulo ten,
/// so any stem index is accepted.
#[must_use]
pub const fn first_hour_stem(day_stem: u8) -> u8 {
    (2 * (day_stem % 5)) % 10
}

/// The stem of the 寅 month of a year with the given year stem: the 五虎遁
/// rule.
///
/// The mnemonic 五虎遁年起月訣 runs 甲己之年丙作首, 乙庚之歲戊為頭,
/// 丙辛必定尋庚起, 丁壬壬位順行流, 戊癸甲寅好追求. It is the 五鼠遁
/// pattern shifted by one stem pair, because the year's first month is 寅 and
/// not 子.
#[must_use]
pub const fn first_month_stem(year_stem: u8) -> u8 {
    (2 * (year_stem % 5) + 2) % 10
}

/// The hour pillar (時柱) of a double-hour on a day with the given day
/// pillar.
///
/// The branch is the double-hour itself; the stem comes from the day's stem
/// by [`first_hour_stem`] and then advances one place per double-hour, so the
/// twelve hour pillars of a day are twelve consecutive terms of the sixty
/// cycle and five days of hours are one full cycle.
///
/// `day` must be the pillar of the day the *hour* belongs to, which between
/// 23:00 and midnight is not the civil day — see [`pillar_day`].
#[must_use]
pub const fn hour_pillar(day: Sexagenary, hour: DoubleHour) -> Sexagenary {
    let stem = (first_hour_stem(day.stem_index()) + hour.index()) % 10;
    match Sexagenary::from_stem_branch_index(stem, hour.branch_index()) {
        Some(pillar) => pillar,
        // Unreachable: the 五鼠遁 offset is even, so the stem and the branch
        // keep the same parity for every hour of every day.
        None => Sexagenary::from_index(0),
    }
}

/// The month pillar (月柱) of a solar-term month in a year with the given
/// year pillar.
///
/// `solar_term_month` is one-based and counts the twelve 節月, the months of
/// the *solar-term* year: month 1 is 寅月, which opens at 立春, month 2 is
/// 卯月 at 驚蟄, and so on through month 11, 子月 at 大雪 — the month holding
/// the winter solstice — and month 12, 丑月 at 小寒. It is **not** the
/// lunisolar month number, and it does not change at the new moon.
///
/// `hc-seasons` produces it: take the index of the preceding 節気 in its
/// 立春-first ordering, where the sectional terms are the even indices, and
/// the month is `index / 2 + 1`.
///
/// The branch follows directly (`month + 1` modulo twelve) and the stem from
/// the year's stem by [`first_month_stem`], so the twelve month pillars of a
/// year are consecutive and sixty months — five years — close the cycle.
///
/// # Errors
///
/// Returns [`CalendarError::MonthOutOfRange`] unless `solar_term_month` is in
/// `1..=12`.
pub const fn month_pillar(year: Sexagenary, solar_term_month: u8) -> CalendarResult<Sexagenary> {
    if solar_term_month == 0 || solar_term_month > 12 {
        return Err(CalendarError::MonthOutOfRange);
    }
    let stem = (first_month_stem(year.stem_index()) + solar_term_month - 1) % 10;
    let branch = (solar_term_month + 1) % 12;
    match Sexagenary::from_stem_branch_index(stem, branch) {
        Some(pillar) => Ok(pillar),
        // Unreachable: the 五虎遁 offset is even, so stem and branch keep the
        // same parity for every month of every year.
        None => Err(CalendarError::MonthOutOfRange),
    }
}

/// The one-based solar-term month a month branch belongs to.
///
/// The inverse of the branch half of [`month_pillar`]: 寅 is month 1 and 子
/// is month 11.
#[must_use]
pub const fn solar_term_month_of_branch(branch_index: u8) -> u8 {
    match branch_index % 12 {
        0 => 11,
        1 => 12,
        other => other - 1,
    }
}

/// The four pillars (四柱) of a moment: year, month, day and hour, eight
/// characters in all (八字).
///
/// Assembling these by hand from four separate calls is where the mistakes
/// happen — a month pillar taken from the lunisolar month, a year pillar
/// taken from 1 January, an hour pillar derived from the wrong day because
/// the birth was at 23:30. [`FourPillars::new`] takes the pieces a caller can
/// actually obtain and does the two stem derivations itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FourPillars {
    /// 年柱, the year pillar. Which boundary it used is the caller's choice;
    /// see [`sexagenary_year_from_solar_term_year`].
    pub year: Sexagenary,
    /// 月柱, the month pillar, fixed by the solar term.
    pub month: Sexagenary,
    /// 日柱, the day pillar.
    pub day: Sexagenary,
    /// 時柱, the hour pillar.
    pub hour: Sexagenary,
}

impl FourPillars {
    /// Assemble the four pillars from the pieces a caller has.
    ///
    /// The year pillar and the day pillar come from the caller, because both
    /// depend on boundaries this crate cannot compute: the year on 立春 (from
    /// `hc-seasons`, via [`sexagenary_year_at`]) and the day on the civil
    /// date, on the meridian and on the 子-hour convention (via
    /// [`pillar_day`] and [`sexagenary_day`]). The month and hour *stems* are
    /// not independent data at all — they follow from the year and day stems
    /// by the 五虎遁 and 五鼠遁 rules — so they are derived here rather than
    /// asked for.
    ///
    /// ```
    /// use hc_calendar::Rd;
    /// use hc_calendar::cycle::{DoubleHour, FourPillars, sexagenary_day, sexagenary_year_at};
    ///
    /// // 2024-02-10 at noon. 立春 fell on 4 February, so the solar-term year
    /// // is 2024 and the solar-term month is the first, 寅月.
    /// let day = Rd(738_926);
    /// let start_of_spring = Rd(738_920);
    /// let chart = FourPillars::new(
    ///     sexagenary_year_at(day, 2024, start_of_spring),
    ///     1,
    ///     sexagenary_day(day),
    ///     DoubleHour::from_index(6),
    /// )?;
    /// assert_eq!(chart.to_string(), "jia-chen bing-yin jia-chen geng-wu");
    /// # Ok::<(), hc_calendar::CalendarError>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::MonthOutOfRange`] unless `solar_term_month`
    /// is in `1..=12`.
    pub const fn new(
        year: Sexagenary,
        solar_term_month: u8,
        day: Sexagenary,
        hour: DoubleHour,
    ) -> CalendarResult<Self> {
        match month_pillar(year, solar_term_month) {
            Ok(month) => Ok(Self {
                year,
                month,
                day,
                hour: hour_pillar(day, hour),
            }),
            Err(error) => Err(error),
        }
    }

    /// Assemble from four pillars that are already known.
    ///
    /// For reading a chart back out of a printed source, where the stems are
    /// given rather than derived. Nothing checks that the four are mutually
    /// possible; [`FourPillars::is_consistent`] does that on request.
    #[must_use]
    pub const fn from_pillars(
        year: Sexagenary,
        month: Sexagenary,
        day: Sexagenary,
        hour: Sexagenary,
    ) -> Self {
        Self {
            year,
            month,
            day,
            hour,
        }
    }

    /// The double-hour of the hour pillar.
    #[must_use]
    pub const fn double_hour(self) -> DoubleHour {
        DoubleHour::from_index(self.hour.branch_index() as i64)
    }

    /// The one-based solar-term month of the month pillar.
    #[must_use]
    pub const fn solar_term_month(self) -> u8 {
        solar_term_month_of_branch(self.month.branch_index())
    }

    /// 日主 / 日元, the day stem, which is what a four-pillar reading is
    /// centred on.
    #[must_use]
    pub const fn day_master(self) -> u8 {
        self.day.stem_index()
    }

    /// Whether the month and hour stems follow from the year and day stems.
    ///
    /// A chart that fails this was mis-transcribed or mis-derived: the 五虎遁
    /// and 五鼠遁 rules leave no freedom once the year and day pillars and
    /// the month and hour branches are fixed.
    #[must_use]
    pub const fn is_consistent(self) -> bool {
        let month_ok = match month_pillar(self.year, self.solar_term_month()) {
            Ok(expected) => expected.index() == self.month.index(),
            Err(_) => false,
        };
        let hour_ok = hour_pillar(self.day, self.double_hour()).index() == self.hour.index();
        month_ok && hour_ok
    }
}

impl fmt::Display for FourPillars {
    /// Renders the eight characters romanised, in year-month-day-hour order.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {} {}", self.year, self.month, self.day, self.hour)
    }
}

/// The number of the sixty-**year** cycle a traditional year count falls in.
///
/// The cycles are conventionally numbered, and the numbering depends on the
/// epoch the year count uses. Under the reckoning in common circulation, which
/// counts the first cycle from 2697 BCE — the traditional accession of the
/// Yellow Emperor, so that Gregorian 1984 is year 4681 — the year that began
/// on 1984-02-02 opened **cycle 79**, and cycle 79 runs to 2043. Under the
/// 2637 BCE epoch of *Calendrical Calculations*, where the same year is 4621,
/// this returns 78.
///
/// The position within the cycle is the same either way, because the two
/// epochs differ by exactly sixty years; only the ordinal moves. That is why
/// a cycle number quoted without its epoch means nothing, and why the
/// lunisolar calendars publish `cycle` and `year_of_cycle` together with the
/// year count they were derived from.
#[must_use]
pub const fn sexagenary_year_cycle(chinese_year: i64) -> i64 {
    (chinese_year - 1).div_euclid(60) + 1
}

/// The one-based position of a year within its sixty-year cycle.
#[must_use]
pub const fn year_of_sexagenary_cycle(chinese_year: i64) -> u8 {
    ((chinese_year - 1).rem_euclid(60) + 1) as u8
}

/// The number of the sixty-**day** cycle containing a fixed day.
///
/// Unlike the year cycles, the day cycles carry no traditional numbering —
/// almanacs name the day and never count the run it belongs to. So this is
/// the library's own count, anchored so that cycle 1 is the sixty-day run
/// containing RD 1, which begins on the *jia-zi* day RD −14. It is for
/// grouping and for diagnostics, not for citation.
#[must_use]
pub const fn sexagenary_cycle_containing(rd: Rd) -> i64 {
    (rd.0 + 14).div_euclid(60) + 1
}

/// A generic named cycle: `n` positions repeating from an anchor day.
#[derive(Debug, Clone, Copy)]
pub struct NamedDayCycle<'a> {
    /// The names, in cycle order.
    pub names: &'a [&'a str],
    /// A fixed day that occupies position zero.
    pub anchor: Rd,
}

impl<'a> NamedDayCycle<'a> {
    /// Build a cycle from its names and anchor.
    #[must_use]
    pub const fn new(names: &'a [&'a str], anchor: Rd) -> Self {
        Self { names, anchor }
    }

    /// The zero-based position of a day within the cycle.
    #[must_use]
    pub fn position(&self, rd: Rd) -> Option<usize> {
        if self.names.is_empty() {
            return None;
        }
        Some((rd.0 - self.anchor.0).rem_euclid(self.names.len() as i64) as usize)
    }

    /// The name of a day's position.
    #[must_use]
    pub fn name(&self, rd: Rd) -> Option<&'a str> {
        self.position(rd).map(|index| self.names[index])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// RD of 1900-01-01, a 甲戌 day in the published tables.
    const RD_1900_01_01: Rd = Rd(693_596);
    /// RD of 1970-01-01.
    const RD_1970_01_01: Rd = Rd(719_163);
    /// RD of 2024-02-10, the lunisolar new year of the 甲辰 year.
    const RD_2024_02_10: Rd = Rd(738_926);
    /// RD of 1984-01-01.
    const RD_1984_01_01: Rd = Rd(724_276);
    /// RD of 1984-01-15, three weeks before the 甲子 year began.
    const RD_1984_01_15: Rd = Rd(724_290);
    /// RD of 1984-02-02, the first day of the 甲子 lunisolar year.
    const RD_1984_02_02: Rd = Rd(724_308);
    /// RD of 1984-02-04, 立春 of 1984.
    const RD_1984_02_04: Rd = Rd(724_310);
    /// RD of 1985-02-04, 立春 of 1985.
    const RD_1985_02_04: Rd = Rd(724_676);
    /// RD of 1985-02-10, after 立春 but before the lunisolar new year.
    const RD_1985_02_10: Rd = Rd(724_682);
    /// RD of 1985-02-20, the lunisolar new year of the 乙丑 year.
    const RD_1985_02_20: Rd = Rd(724_692);

    /// The Julian Day Number of a fixed day, used to check the day pillar
    /// against the published `(JDN + 9) mod 10` / `(JDN + 1) mod 12` rule.
    const fn jdn(rd: Rd) -> i64 {
        rd.0 + 1_721_425
    }

    #[test]
    fn the_reference_days_agree_with_the_calendar_and_with_the_week() {
        // Every fixed day above is checked twice: once against the day counts
        // between them, and once against the weekday, which is independent of
        // anything in this module.
        assert_eq!(RD_1984_01_01.days_since(RD_1970_01_01), 5_113);
        assert_eq!(RD_1984_01_15.days_since(RD_1984_01_01), 14);
        assert_eq!(RD_1984_02_02.days_since(RD_1984_01_01), 32);
        assert_eq!(RD_1984_02_04.days_since(RD_1984_01_01), 34);
        assert_eq!(RD_1985_02_04.days_since(RD_1984_01_01), 400);
        assert_eq!(RD_1985_02_10.days_since(RD_1985_02_04), 6);
        assert_eq!(RD_1985_02_20.days_since(RD_1985_02_04), 16);
        assert_eq!(RD_2024_02_10.days_since(RD_1984_01_01), 14_650);
        assert_eq!(RD_1970_01_01.days_since(RD_1900_01_01), 25_567);

        use crate::weekday::Weekday;
        assert_eq!(Weekday::from_rd(RD_1900_01_01), Weekday::Monday);
        assert_eq!(Weekday::from_rd(RD_1970_01_01), Weekday::Thursday);
        assert_eq!(Weekday::from_rd(RD_1984_01_01), Weekday::Sunday);
        assert_eq!(Weekday::from_rd(RD_1985_02_04), Weekday::Monday);
        assert_eq!(Weekday::from_rd(RD_2024_02_10), Weekday::Saturday);
    }

    #[test]
    fn the_cycle_starts_at_jia_zi() {
        let first = Sexagenary::from_index(0);
        assert_eq!(first.stem_name(), "jia");
        assert_eq!(first.branch_name(), "zi");
        assert_eq!(first.zodiac_animal(), "rat");
        assert_eq!(first.five_phase(), "wood");
        assert_eq!(first.ordinal(), 1);
    }

    #[test]
    fn the_cycle_closes_after_sixty_steps() {
        let mut position = Sexagenary::from_index(0);
        for _ in 0..60 {
            position = position.next();
        }
        assert_eq!(position, Sexagenary::from_index(0));
    }

    #[test]
    fn stems_and_branches_advance_together() {
        for index in 0..60 {
            let position = Sexagenary::from_index(index);
            assert_eq!(position.stem_index() % 2, position.branch_index() % 2);
        }
    }

    #[test]
    fn impossible_stem_branch_pairs_are_rejected() {
        assert_eq!(
            Sexagenary::from_stem_branch(1, 1),
            Some(Sexagenary::from_index(0))
        );
        // jia (odd) with chou (even) never occurs.
        assert_eq!(Sexagenary::from_stem_branch(1, 2), None);
        assert_eq!(Sexagenary::from_stem_branch(0, 1), None);
        assert_eq!(Sexagenary::from_stem_branch(11, 1), None);
    }

    #[test]
    fn nineteen_eighty_four_is_a_jia_zi_year() {
        // Chinese year 4681 corresponds to Gregorian 1984.
        assert_eq!(sexagenary_year(4_681).index(), 0);
    }

    #[test]
    fn the_day_cycle_advances_by_one_per_day() {
        let today = sexagenary_day(Rd(1));
        assert_eq!(sexagenary_day(Rd(2)), today.next());
        assert_eq!(sexagenary_day(Rd(61)), today);
    }

    #[test]
    fn negative_indices_wrap_into_the_cycle() {
        assert_eq!(Sexagenary::from_index(-1).index(), 59);
        assert_eq!(Sexagenary::from_index(-61).index(), 59);
    }

    #[test]
    fn named_cycles_wrap_in_both_directions() {
        let cycle = NamedDayCycle::new(&["a", "b", "c"], Rd(0));
        assert_eq!(cycle.name(Rd(0)), Some("a"));
        assert_eq!(cycle.name(Rd(4)), Some("b"));
        assert_eq!(cycle.name(Rd(-1)), Some("c"));
        assert_eq!(NamedDayCycle::new(&[], Rd(0)).name(Rd(0)), None);
    }

    // --- indices, polarity and the name tables ---------------------------

    #[test]
    fn stem_and_branch_indices_round_trip_through_the_whole_cycle() {
        for index in 0..60 {
            let position = Sexagenary::from_index(index);
            assert_eq!(
                Sexagenary::from_stem_branch_index(position.stem_index(), position.branch_index()),
                Some(position),
                "index {index}"
            );
            assert_eq!(
                Sexagenary::from_stem_branch(
                    position.stem_index() + 1,
                    position.branch_index() + 1
                ),
                Some(position)
            );
        }
    }

    #[test]
    fn mismatched_parities_have_no_index() {
        assert_eq!(Sexagenary::from_stem_branch_index(0, 1), None);
        assert_eq!(Sexagenary::from_stem_branch_index(1, 0), None);
        assert_eq!(Sexagenary::from_stem_branch_index(10, 0), None);
        assert_eq!(Sexagenary::from_stem_branch_index(0, 12), None);
    }

    #[test]
    fn the_stems_alternate_yang_and_yin() {
        for index in 0..10u8 {
            let expected = if index % 2 == 0 {
                Polarity::Yang
            } else {
                Polarity::Yin
            };
            assert_eq!(Polarity::of_index(index), expected);
        }
        assert!(Polarity::Yang.is_yang());
        assert!(!Polarity::Yin.is_yang());
    }

    #[test]
    fn a_pillars_two_halves_always_share_a_polarity() {
        for index in 0..60 {
            let position = Sexagenary::from_index(index);
            assert_eq!(position.stem_polarity(), position.branch_polarity());
            assert_eq!(position.stem_polarity(), position.polarity());
        }
    }

    #[test]
    fn jia_is_yang_wood_and_gui_is_yin_water() {
        let jia_zi = Sexagenary::from_index(0);
        assert_eq!(jia_zi.polarity(), Polarity::Yang);
        assert_eq!(jia_zi.five_phase(), "wood");
        let gui_hai = Sexagenary::from_index(59);
        assert_eq!(gui_hai.polarity(), Polarity::Yin);
        assert_eq!(gui_hai.five_phase(), "water");
        assert_eq!(readings::HAN.stem(gui_hai), "癸");
        assert_eq!(readings::HAN.branch(gui_hai), "亥");
    }

    #[test]
    fn polarity_names_are_the_standard_characters() {
        assert_eq!(Polarity::Yang.cjk_name(), "陽");
        assert_eq!(Polarity::Yin.cjk_name(), "陰");
        assert_eq!(Polarity::Yang.japanese_kun(), "e");
        assert_eq!(Polarity::Yin.japanese_kun(), "to");
        assert_eq!(Polarity::Yang.to_string(), "yang");
    }

    #[test]
    fn every_name_table_has_the_length_its_cycle_needs() {
        assert_eq!(ZODIAC_ANIMALS.len(), 12);
        assert_eq!(FIVE_PHASES.len(), 5);
        assert_eq!(FIVE_PHASES_CJK.len(), 5);
        assert_eq!(FIVE_PHASES_PINYIN.len(), 5);
        assert_eq!(FIVE_PHASES_JAPANESE_ON.len(), 5);
        assert_eq!(FIVE_PHASES_JAPANESE_KUN.len(), 5);
        assert_eq!(DOUBLE_HOUR_CLASSICAL_NAMES_CJK.len(), 12);
    }

    #[test]
    fn no_name_table_has_an_empty_entry() {
        for table in [
            &ZODIAC_ANIMALS[..],
            &FIVE_PHASES[..],
            &FIVE_PHASES_CJK[..],
            &FIVE_PHASES_PINYIN[..],
            &FIVE_PHASES_JAPANESE_ON[..],
            &FIVE_PHASES_JAPANESE_KUN[..],
            &DOUBLE_HOUR_CLASSICAL_NAMES_CJK[..],
        ] {
            for name in table {
                assert!(!name.is_empty());
            }
        }
    }

    #[test]
    fn the_first_pair_is_the_wood_rat() {
        let jia_zi = Sexagenary::from_index(0);
        assert_eq!(jia_zi.zodiac_animal(), "rat");
        assert_eq!(jia_zi.five_phase_cjk(), "木");
        assert_eq!(jia_zi.five_phase_index(), 0);
        assert_eq!(jia_zi.to_string(), "jia-zi");
    }

    #[test]
    fn stepping_backwards_undoes_stepping_forwards() {
        for index in 0..60 {
            let position = Sexagenary::from_index(index);
            assert_eq!(position.next().previous(), position);
            assert_eq!(position.advance(60), position);
            assert_eq!(position.advance(-1), position.previous());
        }
    }

    // --- the twelve double-hours -----------------------------------------

    #[test]
    fn the_day_of_double_hours_begins_at_eleven_at_night() {
        let zi = DoubleHour::from_hour_of_day(23).unwrap();
        assert_eq!(zi.index(), 0);
        assert_eq!(zi.branch_name(), "zi");
        assert_eq!(readings::HAN.branch_at(zi.branch_index()), "子");
        assert_eq!(zi.start_hour(), 23);
        assert_eq!(zi.end_hour(), 1);
        assert!(zi.starts_on_the_previous_civil_day());
    }

    #[test]
    fn midnight_is_still_within_the_rat_hour() {
        assert_eq!(
            DoubleHour::from_hour_of_day(0).unwrap(),
            DoubleHour::from_hour_of_day(23).unwrap()
        );
        assert_eq!(DoubleHour::from_hour_of_day(0).unwrap().index(), 0);
        assert_eq!(DoubleHour::from_hour_of_day(1).unwrap().index(), 1);
    }

    #[test]
    fn each_double_hour_covers_exactly_two_civil_hours() {
        let mut counts = [0u8; 12];
        for hour in 0..24u8 {
            counts[DoubleHour::from_hour_of_day(hour).unwrap().index() as usize] += 1;
        }
        assert_eq!(counts, [2u8; 12]);
    }

    #[test]
    fn noon_falls_in_the_horse_hour() {
        // 午時 runs 11:00–13:00, which is why 正午 means noon.
        for hour in [11u8, 12] {
            let position = DoubleHour::from_hour_of_day(hour).unwrap();
            assert_eq!(position.branch_name(), "wu");
            assert_eq!(position.zodiac_animal(), "horse");
            assert_eq!(position.index(), 6);
        }
        assert_eq!(DoubleHour::from_index(6).start_hour(), 11);
    }

    #[test]
    fn double_hours_report_the_hours_they_contain() {
        for index in 0..12i64 {
            let position = DoubleHour::from_index(index);
            assert!(position.contains_hour(position.start_hour()));
            assert!(position.contains_hour((position.start_hour() + 1) % 24));
            assert!(!position.contains_hour(position.end_hour()));
            assert!(!position.contains_hour(24));
        }
    }

    #[test]
    fn double_hour_indices_wrap_and_step() {
        assert_eq!(DoubleHour::from_index(12).index(), 0);
        assert_eq!(DoubleHour::from_index(-1).index(), 11);
        assert_eq!(DoubleHour::from_index(11).next().index(), 0);
        assert_eq!(DoubleHour::from_index(0).to_string(), "zi-hour");
    }

    #[test]
    fn an_hour_outside_the_civil_day_is_rejected() {
        assert_eq!(
            DoubleHour::from_hour_of_day(24),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            DoubleHour::from_hour_of_day(255),
            Err(CalendarError::DayOutOfRange)
        );
    }

    #[test]
    fn a_leap_second_still_belongs_to_the_rat_hour() {
        let leap = CivilTime::hms(23, 59, 60).unwrap();
        assert_eq!(DoubleHour::containing(leap).index(), 0);
        assert_eq!(
            DoubleHour::containing(CivilTime::hms(23, 0, 0).unwrap()).index(),
            0
        );
        assert_eq!(
            DoubleHour::containing(CivilTime::hms(22, 59, 59).unwrap()).index(),
            11
        );
    }

    #[test]
    fn the_classical_double_hour_names_run_from_midnight() {
        assert_eq!(DoubleHour::from_index(0).classical_name_cjk(), "夜半");
        assert_eq!(DoubleHour::from_index(6).classical_name_cjk(), "日中");
        assert_eq!(DoubleHour::from_index(11).classical_name_cjk(), "人定");
    }

    // --- the day boundary -------------------------------------------------

    #[test]
    fn the_day_pillar_advances_at_eleven_at_night() {
        let late = CivilDateTime::new(RD_1970_01_01 - 1, CivilTime::hms(23, 30, 0).unwrap());
        assert_eq!(
            pillar_day(late, ZiHourConvention::DayStartsAtZiHour),
            RD_1970_01_01
        );
        assert_eq!(
            pillar_day(late, ZiHourConvention::DayStartsAtMidnight),
            RD_1970_01_01 - 1
        );
    }

    #[test]
    fn the_two_zi_conventions_agree_for_twenty_three_hours_out_of_twenty_four() {
        for hour in 0..23u8 {
            let when = CivilDateTime::new(RD_1970_01_01, CivilTime::hms(hour, 0, 0).unwrap());
            assert_eq!(
                pillar_day(when, ZiHourConvention::DayStartsAtZiHour),
                pillar_day(when, ZiHourConvention::DayStartsAtMidnight),
                "hour {hour}"
            );
        }
        let last = CivilDateTime::new(RD_1970_01_01, CivilTime::hms(23, 0, 0).unwrap());
        assert_ne!(
            pillar_day(last, ZiHourConvention::DayStartsAtZiHour),
            pillar_day(last, ZiHourConvention::DayStartsAtMidnight)
        );
    }

    #[test]
    fn the_two_zi_schools_give_different_hour_pillars_for_the_same_instant() {
        // 1969-12-31 was a geng-chen day and 1970-01-01 a xin-si one.
        let when = CivilDateTime::new(RD_1970_01_01 - 1, CivilTime::hms(23, 30, 0).unwrap());
        let early = sexagenary_day(pillar_day(when, ZiHourConvention::DayStartsAtZiHour));
        let late = sexagenary_day(pillar_day(when, ZiHourConvention::DayStartsAtMidnight));
        assert_eq!(early.stem_name(), "xin");
        assert_eq!(late.stem_name(), "geng");
        let hour = DoubleHour::containing(when.time);
        // 辛 days open with 戊子, 庚 days with 丙子.
        assert_eq!(hour_pillar(early, hour).stem_name(), "wu");
        assert_eq!(hour_pillar(late, hour).stem_name(), "bing");
    }

    // --- the hour pillar, 五鼠遁 ------------------------------------------

    #[test]
    fn the_rat_hour_stem_follows_the_five_rats_rule() {
        // 甲己→甲子, 乙庚→丙子, 丙辛→戊子, 丁壬→庚子, 戊癸→壬子.
        let expected = [0u8, 2, 4, 6, 8, 0, 2, 4, 6, 8];
        for (day_stem, want) in expected.iter().enumerate() {
            assert_eq!(first_hour_stem(day_stem as u8), *want, "stem {day_stem}");
        }
    }

    #[test]
    fn the_twelve_hour_pillars_of_a_day_are_consecutive() {
        for day_index in 0..60 {
            let day = Sexagenary::from_index(day_index);
            let first = hour_pillar(day, DoubleHour::from_index(0));
            for hour in 0..12i64 {
                assert_eq!(
                    hour_pillar(day, DoubleHour::from_index(hour)),
                    first.advance(hour),
                    "day {day_index} hour {hour}"
                );
            }
        }
    }

    #[test]
    fn five_days_of_hours_are_one_full_cycle() {
        let mut seen = [false; 60];
        for day_offset in 0..5i64 {
            let day = Sexagenary::from_index(day_offset);
            for hour in 0..12i64 {
                let pillar = hour_pillar(day, DoubleHour::from_index(hour));
                assert!(!seen[pillar.index() as usize], "repeat at {day_offset}");
                seen[pillar.index() as usize] = true;
            }
        }
        assert!(seen.iter().all(|hit| *hit));
    }

    #[test]
    fn every_hour_pillar_is_a_possible_stem_branch_pair() {
        for day_index in 0..60 {
            let day = Sexagenary::from_index(day_index);
            for hour in 0..12i64 {
                let position = DoubleHour::from_index(hour);
                let pillar = hour_pillar(day, position);
                assert_eq!(pillar.branch_index(), position.branch_index());
                assert_eq!(pillar.stem_index() % 2, pillar.branch_index() % 2);
            }
        }
    }

    #[test]
    fn the_hour_pillar_of_noon_on_the_unix_epoch() {
        // 1970-01-01 was a xin-si day; 辛 days open with 戊子, so 午 — the
        // seventh double-hour — carries the stem six places on from 戊.
        let day = sexagenary_day(RD_1970_01_01);
        let noon = DoubleHour::containing(CivilTime::NOON);
        let pillar = hour_pillar(day, noon);
        assert_eq!(pillar.stem_name(), "jia");
        assert_eq!(pillar.branch_name(), "wu");
    }

    // --- the month pillar, 五虎遁 -----------------------------------------

    #[test]
    fn the_tiger_month_stem_follows_the_five_tigers_rule() {
        // 甲己→丙寅, 乙庚→戊寅, 丙辛→庚寅, 丁壬→壬寅, 戊癸→甲寅.
        let expected = [2u8, 4, 6, 8, 0, 2, 4, 6, 8, 0];
        for (year_stem, want) in expected.iter().enumerate() {
            assert_eq!(first_month_stem(year_stem as u8), *want, "stem {year_stem}");
        }
    }

    #[test]
    fn the_first_solar_term_month_is_always_the_tiger_month() {
        for year_index in 0..60 {
            let year = Sexagenary::from_index(year_index);
            let first = month_pillar(year, 1).unwrap();
            assert_eq!(first.branch_name(), "yin");
            assert_eq!(readings::HAN.branch(first), "寅");
        }
    }

    #[test]
    fn the_eleventh_solar_term_month_holds_the_winter_solstice() {
        // 子月 is the month of 大雪 and the solstice, hence "month 11".
        let year = Sexagenary::from_index(0);
        assert_eq!(month_pillar(year, 11).unwrap().branch_name(), "zi");
        assert_eq!(month_pillar(year, 12).unwrap().branch_name(), "chou");
    }

    #[test]
    fn the_twelve_month_pillars_of_a_year_are_consecutive() {
        for year_index in 0..60 {
            let year = Sexagenary::from_index(year_index);
            let first = month_pillar(year, 1).unwrap();
            for month in 1..=12u8 {
                assert_eq!(
                    month_pillar(year, month).unwrap(),
                    first.advance(i64::from(month) - 1),
                    "year {year_index} month {month}"
                );
            }
        }
    }

    #[test]
    fn sixty_months_of_pillars_are_five_years() {
        let mut seen = [false; 60];
        for year_offset in 0..5i64 {
            let year = Sexagenary::from_index(year_offset);
            for month in 1..=12u8 {
                let pillar = month_pillar(year, month).unwrap();
                assert!(!seen[pillar.index() as usize]);
                seen[pillar.index() as usize] = true;
            }
        }
        assert!(seen.iter().all(|hit| *hit));
        // And the cycle really closes: year 0 and year 5 share their months.
        assert_eq!(
            month_pillar(Sexagenary::from_index(0), 1),
            month_pillar(Sexagenary::from_index(5), 1)
        );
    }

    #[test]
    fn a_month_outside_one_to_twelve_is_rejected() {
        let year = Sexagenary::from_index(0);
        assert_eq!(month_pillar(year, 0), Err(CalendarError::MonthOutOfRange));
        assert_eq!(month_pillar(year, 13), Err(CalendarError::MonthOutOfRange));
    }

    #[test]
    fn the_month_branch_and_the_month_number_are_inverses() {
        for month in 1..=12u8 {
            let pillar = month_pillar(Sexagenary::from_index(0), month).unwrap();
            assert_eq!(solar_term_month_of_branch(pillar.branch_index()), month);
        }
    }

    #[test]
    fn the_month_pillar_agrees_with_the_lunisolar_month_anchor() {
        // `hc-calendars-lunar` numbers month pillars
        // `12 * (elapsed - 1) + ordinal - 1 + 2` from the same epoch as
        // `sexagenary_year`, where month 1 of the lunisolar year is the 寅
        // month. The two derivations must agree term for term.
        for elapsed in 4_600i64..4_700 {
            let year = sexagenary_year(elapsed);
            for month in 1..=12u8 {
                let from_rule = month_pillar(year, month).unwrap();
                let from_anchor =
                    Sexagenary::from_index(12 * (elapsed - 1) + i64::from(month) - 1 + 2);
                assert_eq!(from_rule, from_anchor, "year {elapsed} month {month}");
            }
        }
    }

    #[test]
    fn the_first_month_of_the_jia_chen_year_is_bing_yin() {
        // 2024 is a 甲 year, so 五虎遁 gives 丙寅 for the month that opens at
        // 立春 2024.
        let year = sexagenary_year_from_gregorian_year(2_024);
        assert_eq!(readings::HAN.stem(year), "甲");
        assert_eq!(readings::HAN.branch(year), "辰");
        let first = month_pillar(year, 1).unwrap();
        assert_eq!(readings::HAN.stem(first), "丙");
        assert_eq!(readings::HAN.branch(first), "寅");
    }

    // --- the year pillar and its three boundaries ------------------------

    #[test]
    fn two_thousand_and_twenty_four_is_jia_chen() {
        // The Wood Dragon, by two independent derivations: the Gregorian-year
        // rule and the Chinese elapsed-year count of Calendrical Calculations.
        let from_gregorian = sexagenary_year_from_gregorian_year(2_024);
        assert_eq!(from_gregorian.index(), 40);
        assert_eq!(from_gregorian.stem_name(), "jia");
        assert_eq!(from_gregorian.branch_name(), "chen");
        assert_eq!(from_gregorian.zodiac_animal(), "dragon");
        assert_eq!(sexagenary_year(4_661), from_gregorian);
    }

    #[test]
    fn nineteen_eighty_four_is_jia_zi_by_every_route() {
        assert_eq!(sexagenary_year_from_gregorian_year(1_984).index(), 0);
        assert_eq!(sexagenary_year_from_solar_term_year(1_984).index(), 0);
        assert_eq!(sexagenary_year(4_681).index(), 0);
        assert_eq!(sexagenary_year(4_621).index(), 0);
    }

    #[test]
    fn the_three_year_conventions_share_their_arithmetic() {
        for gregorian in 1_800i64..2_200 {
            assert_eq!(
                sexagenary_year_from_gregorian_year(gregorian),
                sexagenary_year_from_solar_term_year(gregorian)
            );
            // The Chinese elapsed count of Calendrical Calculations is
            // Gregorian + 2637.
            assert_eq!(
                sexagenary_year(gregorian + 2_637),
                sexagenary_year_from_gregorian_year(gregorian)
            );
        }
    }

    #[test]
    fn a_january_birthday_carries_the_previous_years_pillar() {
        // 1984-01-15: 立春 1984 fell on 4 February, so the four-pillar year
        // is still 1983 — gui-hai, the last term of the cycle — even though
        // the Gregorian year number says jia-zi.
        assert_eq!(solar_term_year(RD_1984_01_15, 1_984, RD_1984_02_04), 1_983);
        let pillar = sexagenary_year_at(RD_1984_01_15, 1_984, RD_1984_02_04);
        assert_eq!(pillar.index(), 59);
        assert_eq!(pillar.stem_name(), "gui");
        assert_eq!(pillar.branch_name(), "hai");
        assert_ne!(pillar, sexagenary_year_from_gregorian_year(1_984));
    }

    #[test]
    fn the_year_pillar_turns_on_the_day_of_the_start_of_spring() {
        assert_eq!(
            solar_term_year(RD_1984_02_04 - 1, 1_984, RD_1984_02_04),
            1_983
        );
        assert_eq!(solar_term_year(RD_1984_02_04, 1_984, RD_1984_02_04), 1_984);
        assert_eq!(
            sexagenary_year_at(RD_1984_02_04, 1_984, RD_1984_02_04).index(),
            0
        );
    }

    #[test]
    fn the_lunisolar_new_year_and_the_start_of_spring_disagree_for_days_at_a_time() {
        // 1985: 立春 on 4 February, lunisolar new year on 20 February. A
        // birth on 10 February is in the yi-chou year by the four-pillar
        // reckoning and still in the jia-zi year by the civil one.
        let four_pillar = sexagenary_year_at(RD_1985_02_10, 1_985, RD_1985_02_04);
        assert_eq!(four_pillar.stem_name(), "yi");
        assert_eq!(four_pillar.branch_name(), "chou");
        assert!(RD_1985_02_10 < RD_1985_02_20);
        // The civil year that began in 1984 is jia-zi and runs to 1985-02-19.
        assert_eq!(sexagenary_year(4_681).index(), 0);
        assert_eq!(sexagenary_year(4_682).index(), 1);
    }

    #[test]
    fn the_lunisolar_new_year_of_nineteen_eighty_four_opened_the_jia_zi_year() {
        assert_eq!(RD_1984_02_02.days_since(RD_1984_01_15), 18);
        assert!(RD_1984_02_02 < RD_1984_02_04);
        // Before 立春 but after the lunisolar new year the two conventions
        // disagree the other way round: civil jia-zi, four-pillar gui-hai.
        assert_eq!(
            sexagenary_year_at(RD_1984_02_02, 1_984, RD_1984_02_04).index(),
            59
        );
    }

    #[test]
    fn the_solar_term_year_label_is_stable_within_a_year() {
        for offset in 0..360i64 {
            let day = Rd(RD_1984_02_04.0 + offset);
            assert_eq!(solar_term_year(day, 1_984, RD_1984_02_04), 1_984);
        }
    }

    // --- the day pillar ---------------------------------------------------

    #[test]
    fn the_first_day_of_nineteen_hundred_was_a_jia_xu_day() {
        // The anchor published in almanac tables and used by the classic
        // (JDN + 9) mod 10 / (JDN + 1) mod 12 rule.
        let pillar = sexagenary_day(RD_1900_01_01);
        assert_eq!(readings::HAN.stem(pillar), "甲");
        assert_eq!(readings::HAN.branch(pillar), "戌");
        assert_eq!(pillar.index(), 10);
    }

    #[test]
    fn the_unix_epoch_was_a_xin_si_day() {
        // 25 567 days after 1900-01-01, which is 7 places on in the cycle.
        assert_eq!(RD_1970_01_01.days_since(RD_1900_01_01), 25_567);
        let pillar = sexagenary_day(RD_1970_01_01);
        assert_eq!(pillar.index(), 17);
        assert_eq!(pillar.stem_name(), "xin");
        assert_eq!(pillar.branch_name(), "si");
        assert_eq!(pillar, sexagenary_day(RD_1900_01_01).advance(25_567 % 60));
    }

    #[test]
    fn the_day_pillar_agrees_with_the_julian_day_number_rule() {
        // Two centuries of days, checked against the published JDN rule
        // rather than against this module's own anchor.
        for offset in 0..73_000i64 {
            let rd = Rd(RD_1900_01_01.0 + offset);
            let pillar = sexagenary_day(rd);
            assert_eq!(
                i64::from(pillar.stem_index()),
                (jdn(rd) + 9).rem_euclid(10),
                "stem at RD {}",
                rd.0
            );
            assert_eq!(
                i64::from(pillar.branch_index()),
                (jdn(rd) + 1).rem_euclid(12),
                "branch at RD {}",
                rd.0
            );
        }
    }

    #[test]
    fn the_day_pillar_runs_backwards_through_the_proleptic_past() {
        for offset in 1..2_000i64 {
            let rd = Rd(1 - offset);
            assert_eq!(sexagenary_day(rd), sexagenary_day(Rd(1)).advance(-offset));
        }
    }

    #[test]
    fn the_lunisolar_new_year_of_two_thousand_and_twenty_four_was_a_jia_chen_day() {
        // 2024-02-10, reached from the 1900-01-01 anchor: 45 330 days on,
        // which is 30 places in the cycle.
        assert_eq!(RD_2024_02_10.days_since(RD_1900_01_01), 45_330);
        let pillar = sexagenary_day(RD_2024_02_10);
        assert_eq!(pillar.index(), 40);
        assert_eq!(readings::HAN.stem(pillar), "甲");
        assert_eq!(readings::HAN.branch(pillar), "辰");
    }

    // --- cycle numbering ---------------------------------------------------

    #[test]
    fn nineteen_eighty_four_opened_the_seventy_ninth_cycle() {
        // Under the 2697 BCE reckoning, where 1984 is year 4681.
        assert_eq!(sexagenary_year_cycle(4_681), 79);
        assert_eq!(year_of_sexagenary_cycle(4_681), 1);
        assert_eq!(sexagenary_year_cycle(4_740), 79);
        assert_eq!(year_of_sexagenary_cycle(4_740), 60);
        assert_eq!(sexagenary_year_cycle(4_741), 80);
        // 4681 + 59 is Gregorian 2043, the last year of the cycle.
        assert_eq!(4_681 + 59 - 2_697, 2_043);
    }

    #[test]
    fn the_two_epochs_disagree_by_a_cycle_but_not_by_a_position() {
        // Calendrical Calculations numbers the same year 4621.
        assert_eq!(sexagenary_year_cycle(4_621), 78);
        assert_eq!(year_of_sexagenary_cycle(4_621), 1);
        assert_eq!(sexagenary_year(4_621), sexagenary_year(4_681));
    }

    #[test]
    fn the_year_of_cycle_is_one_based_and_wraps_at_sixty() {
        for year in 4_681i64..4_741 {
            let position = year_of_sexagenary_cycle(year);
            assert!((1..=60).contains(&position));
            assert_eq!(position, sexagenary_year(year).ordinal(), "year {year}");
        }
        assert_eq!(year_of_sexagenary_cycle(1), 1);
        assert_eq!(year_of_sexagenary_cycle(0), 60);
        assert_eq!(sexagenary_year_cycle(1), 1);
        assert_eq!(sexagenary_year_cycle(0), 0);
    }

    #[test]
    fn the_day_cycle_number_advances_every_sixtieth_day() {
        let first = sexagenary_cycle_containing(Rd(1));
        assert_eq!(first, 1);
        // RD −14 is the jia-zi day that opens the run containing RD 1.
        assert_eq!(sexagenary_day(Rd(-14)).index(), 0);
        assert_eq!(sexagenary_cycle_containing(Rd(-14)), 1);
        assert_eq!(sexagenary_cycle_containing(Rd(-15)), 0);
        assert_eq!(sexagenary_cycle_containing(Rd(45)), 1);
        assert_eq!(sexagenary_cycle_containing(Rd(46)), 2);
    }

    #[test]
    fn a_day_cycle_number_changes_exactly_when_the_cycle_restarts() {
        let mut previous = sexagenary_cycle_containing(Rd(0));
        for rd in 1..600i64 {
            let day = Rd(rd);
            let number = sexagenary_cycle_containing(day);
            if sexagenary_day(day).index() == 0 {
                assert_eq!(number, previous + 1, "RD {rd}");
            } else {
                assert_eq!(number, previous, "RD {rd}");
            }
            previous = number;
        }
    }

    // --- the four pillars --------------------------------------------------

    #[test]
    fn the_four_pillars_assemble_from_the_pieces_a_caller_has() {
        // 2024-02-10 at noon, the lunisolar new year of the jia-chen year:
        // 立春 had passed on 4 February, so the solar-term month is 1.
        let year = sexagenary_year_at(RD_2024_02_10, 2_024, Rd(RD_2024_02_10.0 - 6));
        let day = sexagenary_day(RD_2024_02_10);
        let hour = DoubleHour::containing(CivilTime::NOON);
        let chart = FourPillars::new(year, 1, day, hour).unwrap();
        assert_eq!(readings::HAN.stem(chart.year), "甲");
        assert_eq!(readings::HAN.branch(chart.year), "辰");
        assert_eq!(readings::HAN.stem(chart.month), "丙");
        assert_eq!(readings::HAN.branch(chart.month), "寅");
        assert_eq!(readings::HAN.stem(chart.day), "甲");
        assert_eq!(readings::HAN.branch(chart.day), "辰");
        assert_eq!(readings::HAN.stem(chart.hour), "庚");
        assert_eq!(readings::HAN.branch(chart.hour), "午");
        assert!(chart.is_consistent());
    }

    #[test]
    fn the_four_pillars_round_trip_through_their_parts() {
        for year_index in (0..60).step_by(7) {
            let year = Sexagenary::from_index(year_index);
            for month in 1..=12u8 {
                for day_index in (0..60).step_by(11) {
                    let day = Sexagenary::from_index(day_index);
                    for hour in 0..12i64 {
                        let position = DoubleHour::from_index(hour);
                        let chart = FourPillars::new(year, month, day, position).unwrap();
                        assert_eq!(chart.solar_term_month(), month);
                        assert_eq!(chart.double_hour(), position);
                        assert_eq!(chart.year, year);
                        assert_eq!(chart.day, day);
                        assert_eq!(chart.day_master(), day.stem_index());
                        assert!(chart.is_consistent());
                        assert_eq!(
                            FourPillars::new(
                                chart.year,
                                chart.solar_term_month(),
                                chart.day,
                                chart.double_hour()
                            )
                            .unwrap(),
                            chart
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn the_four_pillars_reject_an_impossible_month() {
        let position = Sexagenary::from_index(0);
        let hour = DoubleHour::from_index(0);
        assert_eq!(
            FourPillars::new(position, 0, position, hour),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            FourPillars::new(position, 13, position, hour),
            Err(CalendarError::MonthOutOfRange)
        );
    }

    #[test]
    fn a_mis_transcribed_chart_fails_the_consistency_check() {
        let year = Sexagenary::from_index(0);
        let day = Sexagenary::from_index(17);
        let good = FourPillars::new(year, 1, day, DoubleHour::from_index(6)).unwrap();
        assert!(good.is_consistent());
        // Twelve places on is the same branch with the stem moved by two, a
        // pairing no year stem and no day stem can produce.
        let bad_month =
            FourPillars::from_pillars(good.year, good.month.advance(12), good.day, good.hour);
        assert_eq!(bad_month.month.branch_index(), good.month.branch_index());
        assert_ne!(bad_month.month.stem_index(), good.month.stem_index());
        assert!(!bad_month.is_consistent());
        let bad_hour =
            FourPillars::from_pillars(good.year, good.month, good.day, good.hour.advance(12));
        assert_eq!(bad_hour.double_hour(), good.double_hour());
        assert!(!bad_hour.is_consistent());
    }

    #[test]
    fn the_four_pillars_render_as_eight_characters() {
        let chart = FourPillars::new(
            Sexagenary::from_index(0),
            1,
            Sexagenary::from_index(0),
            DoubleHour::from_index(0),
        )
        .unwrap();
        assert_eq!(chart.to_string(), "jia-zi bing-yin jia-zi jia-zi");
    }

    #[test]
    fn a_late_evening_birth_gets_the_next_days_pillars() {
        // 1969-12-31 at 23:30 under the majority school: the day pillar is
        // 1970-01-01's, and the whole chart follows from it.
        let when = CivilDateTime::new(RD_1970_01_01 - 1, CivilTime::hms(23, 30, 0).unwrap());
        let day = sexagenary_day(pillar_day(when, ZiHourConvention::DayStartsAtZiHour));
        let chart = FourPillars::new(
            sexagenary_year_from_solar_term_year(1_969),
            11,
            day,
            DoubleHour::containing(when.time),
        )
        .unwrap();
        assert_eq!(chart.day.stem_name(), "xin");
        assert_eq!(readings::HAN.stem(chart.hour), "戊");
        assert_eq!(readings::HAN.branch(chart.hour), "子");
        assert_eq!(readings::HAN.branch(chart.month), "子");
        assert!(chart.is_consistent());
    }
}
