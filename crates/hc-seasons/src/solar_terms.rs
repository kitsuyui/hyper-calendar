//! 二十四節気 — the 24 solar terms.
//!
//! The system is written up in `docs/systems/solar-terms-and-pentads.md` in
//! the repository: the 15° division, 定気 against 平気, who publishes the
//! terms and where, the meridian at which a term becomes a date, a worked
//! example against the 暦要項, the comparison with the published times, and
//! the sources, keyed in `docs/references.bib`. This page summarises it and
//! states the code's own facts.
//!
//! A solar term is the instant the Sun's apparent longitude reaches a
//! multiple of 15°. Twenty-four multiples, twenty-four terms, one tropical
//! year. That is the whole definition; everything else here is naming,
//! ordering and the business of turning an instant into a date.
//!
//! # Two orderings, and why both exist
//!
//! Almanacs in China, Japan and Korea print the terms starting from 立春 at
//! 315°, because 立春 is where the agricultural year begins. The longitudes,
//! though, start from 春分 at 0°. Both orderings are in common use and
//! neither is wrong, so [`SolarTerm`] carries no preferred index: you ask for
//! one with a [`TermOrder`].
//!
//! Internally the index is the longitude one, and that is worth stating
//! because it makes the 中気 rule fall out: in the 春分-first ordering the
//! **even** indices are the twelve 中気 (*zhongqi*, principal terms, the
//! multiples of 30°) and the **odd** indices are the twelve 節気 (sectional
//! terms). The lunisolar leap-month rule is "a month containing no 中気 is a
//! leap month", so this distinction is not decoration — see
//! [`crate::lunisolar`], which depends on it, and [`crate::rokuyo`], which is
//! built on that.
//!
//! # The day is not the instant
//!
//! A term is an instant in Universal Time; a term *date* is that instant read
//! at some meridian. Beijing is an hour behind Tokyo, so the Chinese and
//! Japanese almanacs put about one term a year on different dates (98 of the
//! 2,400 terms of 1950–2049). Every function here that returns an [`Rd`]
//! therefore takes a [`Meridian`], and none of them guesses. What the
//! instants are good to, and how that was measured against the published
//! times, is in the document.

use hc_astro::solar::{seasonal_event, solar_longitude, solar_longitude_after};
use hc_calendar::Rd;
use hc_calendar::fixed::Moment;
use hc_core::math::floor;

use crate::meridian::Meridian;

/// How many degrees of apparent solar longitude separate two terms.
pub const DEGREES_PER_TERM: f64 = 15.0;

/// How many terms make a year.
pub const TERMS_PER_YEAR: usize = 24;

/// Which term an index counts from.
///
/// The two orderings differ by 21 places; a [`SolarTerm`] is the same term
/// either way.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TermOrder {
    /// Index 0 is 春分 (Chunfen, the spring equinox) at 0°, and the index is
    /// the longitude divided by 15.
    ///
    /// In this ordering the even indices are the 中気 and the odd ones the
    /// 節気, which is the form the lunisolar leap rule wants.
    SpringEquinoxFirst,
    /// Index 0 is 立春 (Lichun, the beginning of spring) at 315°, the
    /// traditional almanac ordering.
    BeginningOfSpringFirst,
}

/// The two classes of solar term.
///
/// The classes alternate strictly around the year, so between any two 中気
/// there is exactly one 節気.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TermKind {
    /// 節気 (*jieqi*, sectional term): the odd multiples of 15°.
    ///
    /// These begin the twelve "months" of the solar-term year, which is what
    /// the four-pillar and 雑節 rules count from.
    Sectional,
    /// 中気 (*zhongqi*, principal term): the multiples of 30°.
    ///
    /// A lunisolar month is named after the 中気 it contains, and a month
    /// containing none is a leap month.
    Principal,
}

/// One of the 24 solar terms.
///
/// Ordering is by apparent solar longitude from 0°, i.e. the
/// [`TermOrder::SpringEquinoxFirst`] ordering, so `SolarTerm` values compare
/// the way their longitudes do and not the way an almanac prints them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SolarTerm(u8);

/// The ways the 24 positions are named, one entry per language or
/// convention. See [`hc_calendar::shape::Naming`].
pub mod namings {
    use hc_calendar::shape::Naming;

    hc_core::catalogue! {
        type: Naming<24>,
        id: |naming| naming.id,
        provenance: |naming| naming.authority,
        tests: term_naming_tests,

        /// Every naming this crate ships.
        pub const ALL;
        /// The naming with this identifier.
        pub fn by_id;

        entries: {
            /// The `chinese` column.
            pub const TRADITIONAL_CHINESE = Naming {
                id: "zh-hant",
                english_name: "Traditional Chinese",
                names: &[
                "春分",
                "清明",
                "穀雨",
                "立夏",
                "小滿",
                "芒種",
                "夏至",
                "小暑",
                "大暑",
                "立秋",
                "處暑",
                "白露",
                "秋分",
                "寒露",
                "霜降",
                "立冬",
                "小雪",
                "大雪",
                "冬至",
                "小寒",
                "大寒",
                "立春",
                "雨水",
                "驚蟄",
                ],
                authority: "Traditional characters, as the almanacs of Taiwan and Hong Kong print the 二十四節氣",
            };
            /// The `japanese` column.
            pub const JAPANESE = Naming {
                id: "ja",
                english_name: "Japanese",
                names: &[
                "春分",
                "清明",
                "穀雨",
                "立夏",
                "小満",
                "芒種",
                "夏至",
                "小暑",
                "大暑",
                "立秋",
                "処暑",
                "白露",
                "秋分",
                "寒露",
                "霜降",
                "立冬",
                "小雪",
                "大雪",
                "冬至",
                "小寒",
                "大寒",
                "立春",
                "雨水",
                "啓蟄",
                ],
                authority: "Post-1946 shinjitai, as the 暦要項 of the National Astronomical Observatory of Japan prints them",
            };
            /// The `pinyin` column.
            pub const PINYIN = Naming {
                id: "zh-pinyin-toned",
                english_name: "Hanyu Pinyin",
                names: &[
                "Chūnfēn",
                "Qīngmíng",
                "Gǔyǔ",
                "Lìxià",
                "Xiǎomǎn",
                "Mángzhòng",
                "Xiàzhì",
                "Xiǎoshǔ",
                "Dàshǔ",
                "Lìqiū",
                "Chǔshǔ",
                "Báilù",
                "Qiūfēn",
                "Hánlù",
                "Shuāngjiàng",
                "Lìdōng",
                "Xiǎoxuě",
                "Dàxuě",
                "Dōngzhì",
                "Xiǎohán",
                "Dàhán",
                "Lìchūn",
                "Yǔshuǐ",
                "Jīngzhé",
                ],
                authority: "汉语拼音方案 (1958), with tone marks",
            };
            /// The `romaji` column.
            pub const ROMAJI = Naming {
                id: "ja-latn",
                english_name: "Japanese, romanised",
                names: &[
                "Shunbun",
                "Seimei",
                "Kokuu",
                "Rikka",
                "Shōman",
                "Bōshu",
                "Geshi",
                "Shōsho",
                "Taisho",
                "Risshū",
                "Shosho",
                "Hakuro",
                "Shūbun",
                "Kanro",
                "Sōkō",
                "Rittō",
                "Shōsetsu",
                "Taisetsu",
                "Tōji",
                "Shōkan",
                "Daikan",
                "Risshun",
                "Usui",
                "Keichitsu",
                ],
                authority: "Hepburn romanisation of the Japanese readings",
            };
            /// The `english` column.
            pub const ENGLISH = Naming {
                id: "en",
                english_name: "English glosses",
                names: &[
                "spring equinox",
                "clear and bright",
                "grain rain",
                "beginning of summer",
                "grain fills",
                "grain in ear",
                "summer solstice",
                "minor heat",
                "major heat",
                "beginning of autumn",
                "limit of heat",
                "white dew",
                "autumn equinox",
                "cold dew",
                "frost descends",
                "beginning of winter",
                "minor snow",
                "major snow",
                "winter solstice",
                "minor cold",
                "major cold",
                "beginning of spring",
                "rain water",
                "insects awaken",
                ],
                authority: "This crate's glosses of the characters; nothing depends on them",
            };
        }
    }
}

/// How many places the 立春-first ordering is rotated from the internal one.
///
/// 立春 sits at 315°, which is internal index 21, so subtracting 21 modulo 24
/// converts one index into the other.
const BEGINNING_OF_SPRING_INDEX: u8 = 21;

/// The internal index of 小寒 at 285°, the first term of a Gregorian year.
///
/// Every Gregorian year over the era this crate supports opens with 小寒 in
/// the first week of January and closes with 冬至 just before Christmas, so
/// this is where a year's iteration starts.
const FIRST_TERM_OF_GREGORIAN_YEAR: u8 = 19;

impl SolarTerm {
    /// 立春, the beginning of spring, at 315°. The traditional start of the
    /// solar-term year and the anchor of most 雑節.
    pub const BEGINNING_OF_SPRING: Self = Self(21);
    /// 春分, the spring equinox, at 0°.
    pub const SPRING_EQUINOX: Self = Self(0);
    /// 立夏, the beginning of summer, at 45°.
    pub const BEGINNING_OF_SUMMER: Self = Self(3);
    /// 夏至, the summer solstice, at 90°.
    pub const SUMMER_SOLSTICE: Self = Self(6);
    /// 立秋, the beginning of autumn, at 135°.
    pub const BEGINNING_OF_AUTUMN: Self = Self(9);
    /// 秋分, the autumn equinox, at 180°.
    pub const AUTUMN_EQUINOX: Self = Self(12);
    /// 立冬, the beginning of winter, at 225°.
    pub const BEGINNING_OF_WINTER: Self = Self(15);
    /// 冬至, the winter solstice, at 270°.
    pub const WINTER_SOLSTICE: Self = Self(18);
    /// 芒種, grain in ear, at 75°. The anchor of the pre-modern 入梅 rule.
    pub const GRAIN_IN_EAR: Self = Self(5);

    /// The term at an internal index, i.e. at longitude `index * 15°`.
    ///
    /// The index is reduced modulo 24, so this cannot fail; it exists so that
    /// the internal call sites do not have to unwrap an `Option` they have
    /// already proved cannot be `None`.
    const fn at(index: u8) -> Self {
        Self(index % 24)
    }

    /// The term at a given index in a given ordering.
    ///
    /// Returns `None` for an index of 24 or more.
    #[must_use]
    pub const fn from_index(order: TermOrder, index: u8) -> Option<Self> {
        if index as usize >= TERMS_PER_YEAR {
            return None;
        }
        match order {
            TermOrder::SpringEquinoxFirst => Some(Self(index)),
            TermOrder::BeginningOfSpringFirst => {
                Some(Self((index + BEGINNING_OF_SPRING_INDEX) % 24))
            }
        }
    }

    /// This term's index in a given ordering, from 0 to 23.
    #[must_use]
    pub const fn index(self, order: TermOrder) -> u8 {
        match order {
            TermOrder::SpringEquinoxFirst => self.0,
            TermOrder::BeginningOfSpringFirst => (self.0 + 24 - BEGINNING_OF_SPRING_INDEX) % 24,
        }
    }

    /// The term defined by a whole number of degrees of apparent solar
    /// longitude.
    ///
    /// Returns `None` unless the angle is a multiple of 15°. Negative and
    /// over-full angles are reduced first, so −45° is 315°.
    #[must_use]
    pub const fn from_degrees(degrees: i32) -> Option<Self> {
        let reduced = degrees.rem_euclid(360);
        if reduced % 15 != 0 {
            return None;
        }
        Some(Self((reduced / 15) as u8))
    }

    /// The apparent solar longitude, in degrees, that defines this term.
    #[must_use]
    pub const fn solar_longitude_degrees(self) -> f64 {
        self.0 as f64 * DEGREES_PER_TERM
    }

    /// Whether this is a 節気 or a 中気.
    #[must_use]
    pub const fn kind(self) -> TermKind {
        if self.0.is_multiple_of(2) {
            TermKind::Principal
        } else {
            TermKind::Sectional
        }
    }

    /// Whether this is a 中気, the class the lunisolar leap-month rule counts.
    #[must_use]
    pub const fn is_principal(self) -> bool {
        matches!(self.kind(), TermKind::Principal)
    }

    /// Whether this is a 節気.
    #[must_use]
    pub const fn is_sectional(self) -> bool {
        matches!(self.kind(), TermKind::Sectional)
    }

    /// The name in traditional Chinese characters, e.g. `"驚蟄"`.
    #[must_use]
    pub const fn chinese_name(self) -> &'static str {
        namings::TRADITIONAL_CHINESE.names[self.0 as usize]
    }

    /// The name in Japanese shinjitai, e.g. `"啓蟄"`.
    ///
    /// Identical to [`Self::chinese_name`] for 21 of the 24 terms.
    #[must_use]
    pub const fn japanese_name(self) -> &'static str {
        namings::JAPANESE.names[self.0 as usize]
    }

    /// The Mandarin reading in pinyin with tone marks, e.g. `"Jīngzhé"`.
    #[must_use]
    pub const fn pinyin(self) -> &'static str {
        namings::PINYIN.names[self.0 as usize]
    }

    /// The Japanese reading in Hepburn romaji, e.g. `"Keichitsu"`.
    #[must_use]
    pub const fn romaji(self) -> &'static str {
        namings::ROMAJI.names[self.0 as usize]
    }

    /// A short English gloss, e.g. `"insects awaken"`.
    ///
    /// These are glosses, not translations: 清明 is "clear and bright" the way
    /// a dictionary would have it, and nothing in this crate depends on them.
    #[must_use]
    pub const fn english_name(self) -> &'static str {
        namings::ENGLISH.names[self.0 as usize]
    }

    /// The next term, 15° further along the ecliptic, wrapping at 360°.
    #[must_use]
    pub const fn next(self) -> Self {
        Self((self.0 + 1) % 24)
    }

    /// The previous term, wrapping at 0°.
    #[must_use]
    pub const fn previous(self) -> Self {
        Self((self.0 + 23) % 24)
    }

    /// All 24 terms in a given ordering.
    #[must_use]
    pub const fn all(order: TermOrder) -> [Self; TERMS_PER_YEAR] {
        let mut terms = [Self(0); TERMS_PER_YEAR];
        let mut index = 0;
        while index < TERMS_PER_YEAR {
            terms[index] = match Self::from_index(order, index as u8) {
                Some(term) => term,
                // Unreachable: the loop bound is the table length.
                None => Self(0),
            };
            index += 1;
        }
        terms
    }
}

/// A solar term together with when it happened.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SolarTermEvent {
    /// Which term.
    pub term: SolarTerm,
    /// The instant the Sun reached the term's longitude, in Universal Time.
    pub moment: Moment,
    /// The day that instant falls on at the meridian it was asked for.
    pub day: Rd,
}

/// The Universal Time instant at which a term occurs in a Gregorian year.
///
/// Each of the 24 terms occurs exactly once in each Gregorian year: the year
/// opens with 小寒 in the first week of January and closes with 冬至 three
/// weeks before it ends, so no term is ever pushed out of its year by the
/// mismatch between the tropical and calendar years.
#[must_use]
pub fn term_moment(year: i64, term: SolarTerm) -> Moment {
    seasonal_event(year, term.solar_longitude_degrees())
}

/// The day a term falls on in a Gregorian year, at a given meridian.
///
/// This is the function whose answer the Chinese and Japanese almanacs can
/// disagree about: the instant is the same, the meridian is not.
///
/// ```
/// use hc_seasons::{Meridian, SolarTerm, solar_terms::term_day};
///
/// // 立春 2024 fell on 4 February in Japan.
/// let day = term_day(2024, SolarTerm::BEGINNING_OF_SPRING, Meridian::JAPAN);
/// assert_eq!(day, hc_calendar::Rd(738_920));
/// ```
#[must_use]
pub fn term_day(year: i64, term: SolarTerm, meridian: Meridian) -> Rd {
    meridian.day_of(term_moment(year, term))
}

/// A term in a Gregorian year as a full event.
#[must_use]
pub fn term_event(year: i64, term: SolarTerm, meridian: Meridian) -> SolarTermEvent {
    let moment = term_moment(year, term);
    SolarTermEvent {
        term,
        moment,
        day: meridian.day_of(moment),
    }
}

/// The term in effect on a day: the most recent term to have begun on or
/// before it.
///
/// A term "begins" on the day its instant falls on, even if that instant is
/// at eleven at night — the almanac prints the whole day as 立春. So the term
/// in effect on the day a term begins is that term.
#[must_use]
pub fn term_in_effect(day: Rd, meridian: Meridian) -> SolarTermEvent {
    // Work from the instant local midnight ends the day. The last term
    // strictly before that instant is the last one whose own day is <= day.
    let end_of_day = meridian.midnight(Rd(day.0 + 1));
    let longitude = solar_longitude(end_of_day);
    let index = floor(longitude / DEGREES_PER_TERM) as u8;
    let term = SolarTerm::at(index);
    // The Sun covers 15° in at most 15.8 days, so searching from 20 days
    // earlier brackets exactly one crossing of the term's longitude: the
    // most recent one, which is the one wanted.
    let moment = solar_longitude_after(term.solar_longitude_degrees(), Moment(end_of_day.0 - 20.0));
    SolarTermEvent {
        term,
        moment,
        day: meridian.day_of(moment),
    }
}

/// The term in effect on a day.
#[must_use]
pub fn term_on_day(day: Rd, meridian: Meridian) -> SolarTerm {
    term_in_effect(day, meridian).term
}

/// The term that begins on a day, if one does.
///
/// Most days answer `None`; 24 days a year answer `Some`.
#[must_use]
pub fn term_beginning_on(day: Rd, meridian: Meridian) -> Option<SolarTermEvent> {
    let event = term_in_effect(day, meridian);
    if event.day == day { Some(event) } else { None }
}

/// How many days a day is into the term in effect, counting the term's own
/// day as 0.
///
/// The answer is 0 to 15; a 15 means the term ran long, which happens around
/// the June solstice, when the Sun is moving slowest and a term lasts nearly
/// 15.7 days.
#[must_use]
pub fn days_into_term(day: Rd, meridian: Meridian) -> i64 {
    day.0 - term_in_effect(day, meridian).day.0
}

/// The 24 terms of a Gregorian year, in date order.
///
/// Date order starts at 小寒 in early January, not at 立春, because that is
/// the order they appear in a Gregorian year. Use
/// [`SolarTerm::all`] if you want almanac order instead.
#[must_use]
pub fn terms_in_year(year: i64, meridian: Meridian) -> TermsInYear {
    TermsInYear {
        year,
        meridian,
        position: 0,
    }
}

/// The iterator returned by [`terms_in_year`].
#[derive(Debug, Clone, Copy)]
pub struct TermsInYear {
    year: i64,
    meridian: Meridian,
    position: usize,
}

impl Iterator for TermsInYear {
    type Item = SolarTermEvent;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position >= TERMS_PER_YEAR {
            return None;
        }
        let index = FIRST_TERM_OF_GREGORIAN_YEAR + self.position as u8;
        self.position += 1;
        Some(term_event(self.year, SolarTerm::at(index), self.meridian))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = TERMS_PER_YEAR - self.position;
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for TermsInYear {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gregorian::from_year_month_day;

    #[test]
    fn the_two_orderings_are_rotations_of_one_another() {
        for index in 0..24u8 {
            let from_equinox = SolarTerm::from_index(TermOrder::SpringEquinoxFirst, index).unwrap();
            assert_eq!(from_equinox.index(TermOrder::SpringEquinoxFirst), index);
            let from_spring =
                SolarTerm::from_index(TermOrder::BeginningOfSpringFirst, index).unwrap();
            assert_eq!(from_spring.index(TermOrder::BeginningOfSpringFirst), index);
        }
        assert_eq!(
            SolarTerm::from_index(TermOrder::BeginningOfSpringFirst, 0),
            Some(SolarTerm::BEGINNING_OF_SPRING)
        );
        assert_eq!(
            SolarTerm::from_index(TermOrder::SpringEquinoxFirst, 0),
            Some(SolarTerm::SPRING_EQUINOX)
        );
        assert_eq!(
            SolarTerm::from_index(TermOrder::SpringEquinoxFirst, 24),
            None
        );
        assert_eq!(
            SolarTerm::from_index(TermOrder::BeginningOfSpringFirst, 200),
            None
        );
    }

    #[test]
    fn the_named_terms_sit_at_the_longitudes_that_define_them() {
        let expected = [
            (SolarTerm::BEGINNING_OF_SPRING, 315.0),
            (SolarTerm::SPRING_EQUINOX, 0.0),
            (SolarTerm::BEGINNING_OF_SUMMER, 45.0),
            (SolarTerm::SUMMER_SOLSTICE, 90.0),
            (SolarTerm::BEGINNING_OF_AUTUMN, 135.0),
            (SolarTerm::AUTUMN_EQUINOX, 180.0),
            (SolarTerm::BEGINNING_OF_WINTER, 225.0),
            (SolarTerm::WINTER_SOLSTICE, 270.0),
            (SolarTerm::GRAIN_IN_EAR, 75.0),
        ];
        for (term, degrees) in expected {
            assert!(
                (term.solar_longitude_degrees() - degrees).abs() < 1e-12,
                "{} was at {}",
                term.japanese_name(),
                term.solar_longitude_degrees()
            );
            assert_eq!(SolarTerm::from_degrees(degrees as i32), Some(term));
        }
    }

    #[test]
    fn only_multiples_of_fifteen_degrees_name_a_term() {
        assert_eq!(SolarTerm::from_degrees(7), None);
        assert_eq!(SolarTerm::from_degrees(14), None);
        assert_eq!(SolarTerm::from_degrees(-45), Some(SolarTerm(21)));
        assert_eq!(
            SolarTerm::from_degrees(720),
            Some(SolarTerm::SPRING_EQUINOX)
        );
    }

    #[test]
    fn the_principal_terms_are_the_multiples_of_thirty_degrees() {
        let mut principal = 0;
        let mut sectional = 0;
        for term in SolarTerm::all(TermOrder::SpringEquinoxFirst) {
            let degrees = term.solar_longitude_degrees() as i32;
            if term.is_principal() {
                principal += 1;
                assert_eq!(
                    degrees % 30,
                    0,
                    "{} at {degrees} is not a multiple of 30",
                    term.japanese_name()
                );
            } else {
                sectional += 1;
                assert_eq!(degrees % 30, 15);
            }
        }
        assert_eq!(principal, 12);
        assert_eq!(sectional, 12);
        assert!(SolarTerm::SPRING_EQUINOX.is_principal());
        assert!(SolarTerm::WINTER_SOLSTICE.is_principal());
        assert!(SolarTerm::BEGINNING_OF_SPRING.is_sectional());
    }

    #[test]
    fn the_two_classes_alternate_all_the_way_round() {
        let mut term = SolarTerm::SPRING_EQUINOX;
        for _ in 0..24 {
            assert_ne!(term.kind(), term.next().kind());
            term = term.next();
        }
        assert_eq!(term, SolarTerm::SPRING_EQUINOX);
    }

    #[test]
    fn stepping_forward_and_back_returns_to_the_same_term() {
        for term in SolarTerm::all(TermOrder::BeginningOfSpringFirst) {
            assert_eq!(term.next().previous(), term);
            assert_eq!(term.previous().next(), term);
        }
    }

    #[test]
    fn every_term_has_a_full_set_of_names() {
        for term in SolarTerm::all(TermOrder::SpringEquinoxFirst) {
            assert!(!term.chinese_name().is_empty());
            assert!(!term.japanese_name().is_empty());
            assert!(!term.pinyin().is_empty());
            assert!(!term.romaji().is_empty());
            assert!(!term.english_name().is_empty());
            assert!(term.english_name().is_ascii());
        }
    }

    /// The Japanese and traditional Chinese spellings differ for exactly
    /// three terms; conflating the two columns would hide that.
    #[test]
    fn three_terms_are_written_differently_in_the_two_scripts() {
        let differing: usize = SolarTerm::all(TermOrder::SpringEquinoxFirst)
            .iter()
            .filter(|term| term.chinese_name() != term.japanese_name())
            .count();
        assert_eq!(differing, 3);
        assert_eq!(SolarTerm(23).chinese_name(), "驚蟄");
        assert_eq!(SolarTerm(23).japanese_name(), "啓蟄");
    }

    #[test]
    fn all_twenty_four_terms_are_distinct() {
        let terms = SolarTerm::all(TermOrder::BeginningOfSpringFirst);
        for (position, term) in terms.iter().enumerate() {
            for other in &terms[position + 1..] {
                assert_ne!(term, other, "{} appeared twice", term.japanese_name());
            }
        }
    }

    /// The National Astronomical Observatory of Japan's 暦要項 for 2024 gives
    /// 立春 on 4 February, 春分 on 20 March, 夏至 on 21 June, 秋分 on 22
    /// September and 冬至 on 21 December, all in JST.
    #[test]
    fn the_terms_of_2024_fall_where_the_japanese_almanac_puts_them() {
        let japan = Meridian::JAPAN;
        let expected = [
            (SolarTerm::BEGINNING_OF_SPRING, (2024, 2, 4)),
            (SolarTerm::SPRING_EQUINOX, (2024, 3, 20)),
            (SolarTerm::SUMMER_SOLSTICE, (2024, 6, 21)),
            (SolarTerm::AUTUMN_EQUINOX, (2024, 9, 22)),
            (SolarTerm::WINTER_SOLSTICE, (2024, 12, 21)),
        ];
        for (term, (year, month, day)) in expected {
            assert_eq!(
                term_day(2024, term, japan),
                from_year_month_day(year, month, day),
                "{} landed wrong",
                term.japanese_name()
            );
        }
    }

    /// The same almanac for 2000: 立春 4 February, 春分 20 March, 冬至 21
    /// December.
    #[test]
    fn the_terms_of_2000_fall_where_the_japanese_almanac_puts_them() {
        let japan = Meridian::JAPAN;
        assert_eq!(
            term_day(2000, SolarTerm::BEGINNING_OF_SPRING, japan),
            from_year_month_day(2000, 2, 4)
        );
        assert_eq!(
            term_day(2000, SolarTerm::SPRING_EQUINOX, japan),
            from_year_month_day(2000, 3, 20)
        );
        assert_eq!(
            term_day(2000, SolarTerm::WINTER_SOLSTICE, japan),
            from_year_month_day(2000, 12, 21)
        );
    }

    #[test]
    fn a_year_has_twenty_four_terms_in_strictly_increasing_order() {
        for year in [1900i64, 1950, 2000, 2024, 2050, 2099] {
            let mut previous: Option<SolarTermEvent> = None;
            let mut count = 0;
            for event in terms_in_year(year, Meridian::JAPAN) {
                if let Some(earlier) = previous {
                    assert!(
                        event.moment.0 > earlier.moment.0,
                        "{} did not precede {} in {year}",
                        earlier.term.japanese_name(),
                        event.term.japanese_name()
                    );
                    assert!(
                        event.day.0 > earlier.day.0,
                        "two terms shared a day in {year}"
                    );
                }
                previous = Some(event);
                count += 1;
            }
            assert_eq!(count, TERMS_PER_YEAR);
        }
    }

    #[test]
    fn consecutive_terms_are_fourteen_to_sixteen_days_apart() {
        for year in [1800i64, 1900, 2000, 2024, 2100, 2200] {
            let mut previous: Option<SolarTermEvent> = None;
            for event in terms_in_year(year, Meridian::JAPAN) {
                if let Some(earlier) = previous {
                    let gap = event.moment.0 - earlier.moment.0;
                    assert!(
                        (14.0..=16.5).contains(&gap),
                        "{} to {} was {gap} days in {year}",
                        earlier.term.japanese_name(),
                        event.term.japanese_name()
                    );
                    let day_gap = event.day.0 - earlier.day.0;
                    assert!(
                        (14..=16).contains(&day_gap),
                        "{} to {} was {day_gap} whole days in {year}",
                        earlier.term.japanese_name(),
                        event.term.japanese_name()
                    );
                }
                previous = Some(event);
            }
        }
    }

    #[test]
    fn a_years_terms_are_the_twenty_four_terms_exactly_once_each() {
        let mut seen = [false; TERMS_PER_YEAR];
        let mut count = 0;
        for event in terms_in_year(2024, Meridian::JAPAN) {
            let index = event.term.index(TermOrder::SpringEquinoxFirst) as usize;
            assert!(
                !seen[index],
                "{} appeared twice",
                event.term.japanese_name()
            );
            seen[index] = true;
            count += 1;
        }
        assert_eq!(count, TERMS_PER_YEAR);
        assert!(seen.iter().all(|&flag| flag));
    }

    #[test]
    fn a_years_iteration_opens_with_minor_cold_and_closes_with_the_solstice() {
        let mut iterator = terms_in_year(2024, Meridian::JAPAN);
        assert_eq!(iterator.len(), 24);
        let first = iterator.next().unwrap();
        assert_eq!(first.term.japanese_name(), "小寒");
        let last = iterator.last().unwrap();
        assert_eq!(last.term, SolarTerm::WINTER_SOLSTICE);
    }

    #[test]
    fn the_term_in_effect_never_lies_about_which_day_it_started() {
        let start = from_year_month_day(2024, 1, 1);
        for offset in 0..366 {
            let day = Rd(start.0 + offset);
            let event = term_in_effect(day, Meridian::JAPAN);
            assert!(event.day <= day, "term dated after the day it covers");
            assert!(
                day.0 - event.day.0 <= 15,
                "a term ran {} days at {day}",
                day.0 - event.day.0
            );
            assert!(
                (0..=15).contains(&days_into_term(day, Meridian::JAPAN)),
                "days into term out of range at {day}"
            );
        }
    }

    #[test]
    fn exactly_twenty_four_days_of_a_year_begin_a_term() {
        let start = crate::gregorian::from_year_month_day(2024, 1, 1);
        let end = crate::gregorian::from_year_month_day(2025, 1, 1);
        let mut beginnings = 0;
        for offset in 0..(end.0 - start.0) {
            if term_beginning_on(Rd(start.0 + offset), Meridian::JAPAN).is_some() {
                beginnings += 1;
            }
        }
        assert_eq!(beginnings, 24);
    }

    #[test]
    fn the_term_in_effect_agrees_with_the_years_own_table() {
        for event in terms_in_year(2024, Meridian::JAPAN) {
            assert_eq!(term_on_day(event.day, Meridian::JAPAN), event.term);
            // The day before belongs to the previous term.
            assert_eq!(
                term_on_day(Rd(event.day.0 - 1), Meridian::JAPAN),
                event.term.previous()
            );
        }
    }

    /// The meridian is not decoration: over a century of 立春 instants, Tokyo
    /// and Beijing must disagree about the date at least once, because a
    /// term instant lands in the 15:00–16:00 UT hour roughly one year in
    /// twenty-four.
    #[test]
    fn tokyo_and_beijing_do_not_always_agree_on_a_terms_date() {
        let mut disagreements = 0;
        for year in 1950..2050 {
            for term in SolarTerm::all(TermOrder::BeginningOfSpringFirst) {
                if term_day(year, term, Meridian::JAPAN) != term_day(year, term, Meridian::CHINA) {
                    disagreements += 1;
                }
            }
        }
        assert!(
            disagreements > 0,
            "the two meridians never disagreed, which cannot be right"
        );
        // One hour in twenty-four, over 2400 term-years, is about 100.
        assert!(
            (40..250).contains(&disagreements),
            "{disagreements} disagreements is not the expected rate"
        );
    }

    #[test]
    fn the_terms_of_a_year_stay_inside_that_year() {
        for year in [1700i64, 1900, 2000, 2024, 2200] {
            let start = crate::gregorian::from_year_month_day(year, 1, 1);
            let end = crate::gregorian::from_year_month_day(year + 1, 1, 1);
            for event in terms_in_year(year, Meridian::JAPAN) {
                assert!(
                    event.day >= start && event.day < end,
                    "{} of {year} escaped its year",
                    event.term.japanese_name()
                );
            }
        }
    }
}
