//! 暦注 and 選日 — the divinatory annotations of the Japanese and Chinese
//! almanac.
//!
//! `hc-seasons` owns the *astronomical* subdivisions of the year: the 24
//! solar terms, the 72 pentads, the 雑節. This crate owns what a printed
//! almanac lays on top of them. A page of the 神宮館 or 高島 almanac gives
//! the date, then the 干支, then 十二直, then 二十八宿, then a paragraph of
//! 暦注下段 and 選日 — and every one of those is a *rule over a cycle*, not
//! an astronomical event.
//!
//! That shape is the crate's architecture. There is one [`DayContext`]
//! holding the facts a rule can ask about, one [`AlmanacRule`] enumerating
//! the shapes a rule can take, and one [`rule_applies`] evaluating them.
//! Thirty-six annotations, one evaluator, and the annotations themselves are
//! data — the same split `hc_seasons::ZassetsuRule` makes for the 雑節.
//!
//! | Module | 暦注 | Cycle it is a rule over |
//! |---|---|---|
//! | [`mansions`] | 二十八宿, 二十七宿 | a free-running 28-day cycle; a lunisolar reset |
//! | [`mansion_undertakings`] | each mansion's favoured and forbidden undertakings, list by list | the 28-day cycle |
//! | [`nayin`] | 納音 | the sixty in pairs |
//! | [`mod@nine_stars`] | 九星 | nine, per year, per 節月 and per day |
//! | [`twelve_directs`] | 十二直 | the day branch, re-anchored at every 節気 |
//! | [`mod@lower_register`] | 暦注下段 | the sexagenary day, the 節月, the mansion |
//! | [`mod@selected_days`] | 選日 | the sexagenary day, the 節月, the Moon |
//! | [`seven_luminaries`] | 七曜 | the seven-day week |
//! | [`lucky_direction`] | 恵方 | the year's heavenly stem |
//! | [`direction_deities`] | 八将神, 金神, 大金神, 姫金神; the 遊行 of 大将軍 and 金神, one rule per reading, and 金神's 間日 | the year's branch or stem; the day's 干支, the season and the 土用 |
//! | [`rounichi`] | 臘日, one rule per reckoning | 小寒, 大寒 or 冬至 and the day's branch; the lunar date |
//! | [`nine_periods`] | 三元九運 | twenty-year periods from 1864, turning at 立春 |
//! | [`days_without_son`] | 손 없는 날 | the Korean lunar day, `dangi` |
//! | [`vietnamese_days`] | Tam Nương, Nguyệt Kỵ | the Vietnamese lunar day, `vietnamese` |
//! | [`mod@first_month_counts`] | 几龙治水, 几牛耕田, 几日得辛, 几人分饼 | the day signs of the Chinese 正月 |
//! | [`rokuyo`] | 六曜 | the Japanese lunisolar date |
//! | [`moon_viewing`] | 十五夜, 十三夜 | the Japanese lunisolar date |
//! | [`mod@day_notes`] | the whole page | all of the above at once |
//!
//! # These are traditional rules, and traditions disagree
//!
//! Almost everything here is folk practice transmitted through commercial
//! almanacs. The National Astronomical Observatory of Japan publishes the
//! solar terms, the 雑節 and the holidays in the 暦要項 and **nothing in this
//! crate**; the 中段 and 下段 were struck from the official calendar at the
//! Meiji reform of 1873 as superstition. Publishers differ from one another,
//! the Edo and Meiji forms differ, and the web is full of confidently wrong
//! tables.
//!
//! So: every rule names its source in a comment; every place two authorities
//! genuinely disagree names each reading in the doc comment, registers a
//! reading of its own where a published date can test it — the 旧暦-month
//! 凶会日 and the two readings of a 九星 switch on 癸巳 — and says which one
//! the plain functions use; every table is checked against *published date
//! lists* in the tests, because a citation cannot catch a transcription
//! error and a printed calendar can. Where no rule could be established the
//! annotation is left out rather than invented, and the README lists the
//! gaps; [`AlmanacRule::Undetermined`] is the value an entry without an
//! established rule would carry.
//!
//! The system document `docs/systems/japanese-almanac-notes.md` explains
//! the annotations, where publishers differ, and how the tables were
//! checked.
//!
//! # Accuracy
//!
//! ## Which lunisolar calendar the Moon-keyed rules use
//!
//! 六曜, 不成就日, the 旧暦 reading of 凶会日, 二十七宿, 十五夜 and 十三夜
//! need a 旧暦 date. All of them take it from [`lunisolar`], inside a
//! [`hc_core::memo::scope`], in the [`lunisolar::Reckoning`] the meridian
//! names: `hc-calendars-lunar`'s `chinese` calendar at [`Meridian::CHINA`]
//! and [`Meridian::CHINA_BEFORE_1929`], and its Japanese Tenpō calendar
//! with the 1872 bound removed at every other meridian, so they agree with
//! one another and with the calendar.
//!
//! | Class | Annotations | Exactness |
//! |---|---|---|
//! | Pure day count | 干支 rules, 七曜, 二十八宿, 恵方 | exact |
//! | 節月-keyed | 十二直, 九星, most of 下段 and 選日 | `hc-astro`'s VSOP87 solar series, good to about 1″ |
//! | Lunisolar | 六曜, 不成就日, 二十七宿, 十五夜, 十三夜 | the 天保暦's rules continued, as `hc-calendars-lunar` computes them |
//!
//! A solar-term instant within about a minute of local midnight can be
//! assigned the wrong day, which moves a 節月 boundary and with it every
//! annotation keyed to one. See the crate README.
//!
//! # Example
//!
//! ```
//! use hc_almanac::{LowerRegister, Meridian, Rd, day_notes::day_notes};
//!
//! // 1 January 2024 was 甲子, the head of the sexagenary cycle; the
//! // almanacs print 建 and 畢宿 against it, and a winter 甲子 is a 天赦日.
//! let notes = day_notes(Rd(738_886), Meridian::JAPAN);
//! assert_eq!(notes.sexagenary().index(), 0);
//! assert_eq!(notes.twelve_direct().japanese_name(), "建");
//! assert_eq!(notes.mansion().japanese_name(), "畢");
//! assert!(notes.lower_register().contains(LowerRegister::TENSHANICHI));
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod context;
pub mod day_notes;
pub mod days_without_son;
pub mod direction_deities;
pub mod first_month_counts;
pub mod lower_register;
pub mod lucky_direction;
pub mod lunisolar;
pub mod mansion_undertakings;
pub mod mansions;
pub mod moon_viewing;
pub mod nayin;
pub mod nine_periods;
pub mod nine_stars;
pub mod rokuyo;
pub mod rounichi;
pub mod rules;
pub mod selected_days;
pub mod seven_luminaries;
pub mod twelve_directs;
pub mod vietnamese_days;

pub use context::{DayContext, SolarMonth, solar_month_of};
pub use day_notes::{Combination, CombinationSet, DayNotes, day_notes};
pub use days_without_son::is_day_without_son;
pub use direction_deities::{BranchDirection, General, WanderingRule};
pub use first_month_counts::{FirstMonthCounts, first_month_counts};
pub use lower_register::{GraveDays, LowerRegister, LowerRegisterSet, lower_register};
pub use lucky_direction::{LuckyDirection, lucky_direction_of_year};
pub use mansions::{Mansion, Mansion27, Quadrant, mansion_of, mansion27_of, mansion27_of_date};
pub use nayin::Nayin;
pub use nine_periods::{Period, period, period_of_year};
pub use nine_stars::{Dun, NineStar, NineStars, day_star, month_star, nine_stars, year_star};
pub use rokuyo::Rokuyo;
pub use rounichi::RounichiRule;
pub use rules::{AlmanacRule, rule_applies};
pub use selected_days::{SelectedDay, SelectedDaySet, selected_days};
pub use seven_luminaries::{Luminary, luminary_of};
pub use twelve_directs::{TwelveDirect, direct_of};

pub use hc_calendar::Rd;
pub use hc_seasons::Meridian;
pub use vietnamese_days::{is_nguyet_ky, is_tam_nuong};
