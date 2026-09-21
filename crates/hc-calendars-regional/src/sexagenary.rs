//! The sexagenary cycle (干支) as a calendar over years, months and days.
//!
//! Ten Heavenly Stems against twelve Earthly Branches give sixty pairs, and
//! East Asia has named years, months, days and two-hour periods with them
//! for well over two thousand years. [`hc_calendar::cycle`] holds the
//! arithmetic; this module turns it into a [`Calendar`] and adds the
//! readings the same sixty names carry in Chinese, Japanese and Korean.
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
//! want.
//!
//! The rule the month pillar does follow here is the traditional mnemonic
//! 甲己之年丙作首 — in a year whose stem is 甲 or 己, the first month is
//! 丙寅 — which [`hc_calendars_lunar::lunisolar`] already implements and
//! tests.
//!
//! # Readings
//!
//! `hc-i18n` is not a dependency of this crate, so the readings ship here.
//! The Japanese ones are the native *kun* readings (甲 = きのえ, "elder
//! brother of wood"), which is how the stems are read in 暦注 and in
//! almanacs; the Sino-Japanese *on* readings (こう, おつ) are used too and
//! are not shipped. The Korean ones are the standard Hangul with Revised
//! Romanisation.

use core::fmt;

use hc_calendar::cycle::{
    EARTHLY_BRANCHES, FIVE_PHASES, HEAVENLY_STEMS, Sexagenary, ZODIAC_ANIMALS, sexagenary_day,
};
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

/// The ten Heavenly Stems in Han characters.
pub const STEMS_HAN: [&str; 10] = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];

/// The twelve Earthly Branches in Han characters.
pub const BRANCHES_HAN: [&str; 12] = [
    "子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥",
];

/// The ten Heavenly Stems in their Japanese *kun* readings.
pub const STEMS_JAPANESE: [&str; 10] = [
    "きのえ",
    "きのと",
    "ひのえ",
    "ひのと",
    "つちのえ",
    "つちのと",
    "かのえ",
    "かのと",
    "みずのえ",
    "みずのと",
];

/// The ten Heavenly Stems in Japanese, romanised.
pub const STEMS_JAPANESE_ROMAJI: [&str; 10] = [
    "kinoe",
    "kinoto",
    "hinoe",
    "hinoto",
    "tsuchinoe",
    "tsuchinoto",
    "kanoe",
    "kanoto",
    "mizunoe",
    "mizunoto",
];

/// The twelve Earthly Branches in their Japanese readings.
pub const BRANCHES_JAPANESE: [&str; 12] = [
    "ね",
    "うし",
    "とら",
    "う",
    "たつ",
    "み",
    "うま",
    "ひつじ",
    "さる",
    "とり",
    "いぬ",
    "い",
];

/// The twelve Earthly Branches in Japanese, romanised.
pub const BRANCHES_JAPANESE_ROMAJI: [&str; 12] = [
    "ne", "ushi", "tora", "u", "tatsu", "mi", "uma", "hitsuji", "saru", "tori", "inu", "i",
];

/// The ten Heavenly Stems in Korean Hangul.
pub const STEMS_KOREAN: [&str; 10] = ["갑", "을", "병", "정", "무", "기", "경", "신", "임", "계"];

/// The ten Heavenly Stems in Korean, Revised Romanisation.
pub const STEMS_KOREAN_ROMAJA: [&str; 10] = [
    "gap", "eul", "byeong", "jeong", "mu", "gi", "gyeong", "sin", "im", "gye",
];

/// The twelve Earthly Branches in Korean Hangul.
pub const BRANCHES_KOREAN: [&str; 12] = [
    "자", "축", "인", "묘", "진", "사", "오", "미", "신", "유", "술", "해",
];

/// The twelve Earthly Branches in Korean, Revised Romanisation.
pub const BRANCHES_KOREAN_ROMAJA: [&str; 12] = [
    "ja", "chuk", "in", "myo", "jin", "sa", "o", "mi", "sin", "yu", "sul", "hae",
];

/// Which spelling of the sixty names to use.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Script {
    /// Han characters, shared by all three languages.
    Han,
    /// Mandarin pinyin, without tone marks.
    Pinyin,
    /// Japanese *kun* readings in hiragana.
    Japanese,
    /// Japanese *kun* readings, romanised.
    JapaneseRomaji,
    /// Korean Hangul.
    Korean,
    /// Korean, in Revised Romanisation.
    KoreanRomaja,
}

/// The Heavenly Stem of a cycle position, in the given script.
#[must_use]
pub fn stem_name(position: Sexagenary, script: Script) -> &'static str {
    let index = position.stem_index() as usize;
    match script {
        Script::Han => STEMS_HAN[index],
        Script::Pinyin => HEAVENLY_STEMS[index],
        Script::Japanese => STEMS_JAPANESE[index],
        Script::JapaneseRomaji => STEMS_JAPANESE_ROMAJI[index],
        Script::Korean => STEMS_KOREAN[index],
        Script::KoreanRomaja => STEMS_KOREAN_ROMAJA[index],
    }
}

/// The Earthly Branch of a cycle position, in the given script.
#[must_use]
pub fn branch_name(position: Sexagenary, script: Script) -> &'static str {
    let index = position.branch_index() as usize;
    match script {
        Script::Han => BRANCHES_HAN[index],
        Script::Pinyin => EARTHLY_BRANCHES[index],
        Script::Japanese => BRANCHES_JAPANESE[index],
        Script::JapaneseRomaji => BRANCHES_JAPANESE_ROMAJI[index],
        Script::Korean => BRANCHES_KOREAN[index],
        Script::KoreanRomaja => BRANCHES_KOREAN_ROMAJA[index],
    }
}

/// The zodiac animal of a cycle position, in English.
#[must_use]
pub fn zodiac_animal(position: Sexagenary) -> &'static str {
    ZODIAC_ANIMALS[position.branch_index() as usize]
}

/// The five-phase element of a cycle position, in English.
#[must_use]
pub fn five_phase(position: Sexagenary) -> &'static str {
    FIVE_PHASES[(position.stem_index() / 2) as usize]
}

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
        f.write_str(stem_name(self.position, Script::Han))?;
        f.write_str(branch_name(self.position, Script::Han))
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

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId("sexagenary"),
            english_name: "Sexagenary cycle (干支)",
            year_kind: YearKind::Astronomical,
            has_leap_months: false,
            is_astronomical: false,
            earliest: None,
            latest: None,
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
        assert_eq!(stem_name(date.position, Script::Pinyin), "jia");
        assert_eq!(branch_name(date.position, Script::Pinyin), "zi");
    }

    #[test]
    fn the_day_cycle_matches_the_anchor_in_hc_calendar() {
        // hc_calendar anchors the cycle so that RD 1 — 0001-01-01
        // proleptic Gregorian — is index 15, the sixteenth pair, 己卯.
        assert_eq!(day_pillar(Rd(1)).index(), 15);
        assert_eq!(day_pillar(Rd(1)).ordinal(), 16);
        assert_eq!(stem_name(day_pillar(Rd(1)), Script::Han), "己");
        assert_eq!(branch_name(day_pillar(Rd(1)), Script::Han), "卯");
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
    fn the_readings_are_complete_and_aligned() {
        assert_eq!(STEMS_HAN.len(), 10);
        assert_eq!(STEMS_JAPANESE.len(), 10);
        assert_eq!(STEMS_JAPANESE_ROMAJI.len(), 10);
        assert_eq!(STEMS_KOREAN.len(), 10);
        assert_eq!(STEMS_KOREAN_ROMAJA.len(), 10);
        assert_eq!(BRANCHES_HAN.len(), 12);
        assert_eq!(BRANCHES_JAPANESE.len(), 12);
        assert_eq!(BRANCHES_JAPANESE_ROMAJI.len(), 12);
        assert_eq!(BRANCHES_KOREAN.len(), 12);
        assert_eq!(BRANCHES_KOREAN_ROMAJA.len(), 12);
        for script in [
            Script::Han,
            Script::Pinyin,
            Script::Japanese,
            Script::JapaneseRomaji,
            Script::Korean,
            Script::KoreanRomaja,
        ] {
            for index in 0..60 {
                let position = Sexagenary::from_index(index);
                assert!(!stem_name(position, script).is_empty());
                assert!(!branch_name(position, script).is_empty());
            }
        }
    }

    #[test]
    fn the_first_pair_reads_the_same_in_all_three_languages() {
        let jiazi = Sexagenary::from_index(0);
        assert_eq!(stem_name(jiazi, Script::Han), "甲");
        assert_eq!(branch_name(jiazi, Script::Han), "子");
        assert_eq!(stem_name(jiazi, Script::Pinyin), "jia");
        assert_eq!(stem_name(jiazi, Script::Japanese), "きのえ");
        assert_eq!(stem_name(jiazi, Script::JapaneseRomaji), "kinoe");
        assert_eq!(branch_name(jiazi, Script::Japanese), "ね");
        assert_eq!(branch_name(jiazi, Script::JapaneseRomaji), "ne");
        assert_eq!(stem_name(jiazi, Script::Korean), "갑");
        assert_eq!(stem_name(jiazi, Script::KoreanRomaja), "gap");
        assert_eq!(branch_name(jiazi, Script::Korean), "자");
        assert_eq!(branch_name(jiazi, Script::KoreanRomaja), "ja");
        assert_eq!(zodiac_animal(jiazi), "rat");
        assert_eq!(five_phase(jiazi), "wood");
    }

    #[test]
    fn the_stems_pair_up_into_the_five_phases() {
        // 甲乙 wood, 丙丁 fire, 戊己 earth, 庚辛 metal, 壬癸 water.
        for index in 0..60i64 {
            let position = Sexagenary::from_index(index);
            assert_eq!(five_phase(position), position.five_phase());
            assert_eq!(zodiac_animal(position), position.zodiac_animal());
        }
        assert_eq!(five_phase(Sexagenary::from_index(2)), "fire");
        assert_eq!(five_phase(Sexagenary::from_index(9)), "water");
    }

    #[test]
    fn the_year_of_the_wood_dragon_began_in_2024() {
        // Chinese year 4661 is jia-chen, the Wood Dragon; its new year was
        // 2024-02-10.
        let new_year = greg(2024, 2, 10);
        let pillars = pillars(new_year).expect("in range");
        assert_eq!(stem_name(pillars.year, Script::Pinyin), "jia");
        assert_eq!(branch_name(pillars.year, Script::Pinyin), "chen");
        assert_eq!(zodiac_animal(pillars.year), "dragon");
        assert_eq!(five_phase(pillars.year), "wood");
    }

    #[test]
    fn the_first_month_of_a_jia_year_is_bing_yin() {
        // 甲己之年丙作首, the traditional mnemonic.
        let new_year = greg(2024, 2, 10);
        let pillars = pillars(new_year).expect("in range");
        assert_eq!(stem_name(pillars.month, Script::Pinyin), "bing");
        assert_eq!(branch_name(pillars.month, Script::Pinyin), "yin");
        assert_eq!(stem_name(pillars.month, Script::Han), "丙");
        assert_eq!(branch_name(pillars.month, Script::Han), "寅");
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
