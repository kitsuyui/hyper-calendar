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
//! | [`mod@nine_stars`] | 九星 | nine, per year, per 節月 and per day |
//! | [`twelve_directs`] | 十二直 | the day branch, re-anchored at every 節気 |
//! | [`mod@lower_register`] | 暦注下段 | the sexagenary day, the 節月, the mansion |
//! | [`mod@selected_days`] | 選日 | the sexagenary day, the 節月, the Moon |
//! | [`seven_luminaries`] | 七曜 | the seven-day week |
//! | [`rokuyo`] | 六曜 | the lunisolar date — re-exported from `hc-seasons` |
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
//! 六曜, 不成就日 and 二十七宿 need a 旧暦 date. This crate takes it from
//! `hc_seasons::lunisolar`, the minimal 定気 derivation `hc-seasons` keeps
//! for 六曜 — not from `hc-calendars-lunar`, whose 天保暦 and Chinese
//! calendars are the fuller implementations. That is deliberate: the three
//! annotations must agree with each other and with the 六曜 a caller gets
//! from `hc-seasons`, and one derivation shared is worth more here than a
//! better one used inconsistently. `hc-calendars-lunar` is re-exported as
//! [`hc_calendars_lunar`] for callers who want the fuller article, and
//! [`crate::context`] carries a test measuring how far the two diverge.
//!
//! | Class | Annotations | Exactness |
//! |---|---|---|
//! | Pure day count | 干支 rules, 七曜, 二十八宿 | exact |
//! | 節月-keyed | 十二直, 九星, most of 下段 and 選日 | `hc-astro`'s VSOP87 solar series, good to about 1″ |
//! | Lunisolar | 六曜, 不成就日, 二十七宿 | `hc-seasons`' minimal 定気 derivation |
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
pub mod lower_register;
pub mod mansions;
pub mod nine_stars;
pub mod rokuyo;
pub mod rules;
pub mod selected_days;
pub mod seven_luminaries;
pub mod twelve_directs;

pub use context::{DayContext, SolarMonth, solar_month_of};
pub use day_notes::{Combination, CombinationSet, DayNotes, day_notes};
pub use lower_register::{LowerRegister, LowerRegisterSet, lower_register};
pub use mansions::{Mansion, Mansion27, Quadrant, mansion_of, mansion27_of};
pub use nine_stars::{Dun, NineStar, NineStars, day_star, month_star, nine_stars, year_star};
pub use rokuyo::Rokuyo;
pub use rules::{AlmanacRule, rule_applies};
pub use selected_days::{SelectedDay, SelectedDaySet, selected_days};
pub use seven_luminaries::{Luminary, luminary_of};
pub use twelve_directs::{TwelveDirect, direct_of};

pub use hc_astro;
pub use hc_calendar;
pub use hc_calendar::Rd;
pub use hc_calendars_lunar;
pub use hc_seasons;
pub use hc_seasons::Meridian;
