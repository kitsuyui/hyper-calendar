//! Seasonal subdivisions: what a year is made of once you stop counting
//! months.
//!
//! A calendar names days. This crate names the *parts of the year* that sit
//! underneath the naming — the 24 solar terms, the 72 pentads, the Japanese
//! 雑節, the 六曜 cycle, the Moon's phases, and the four seasons under each
//! of the three definitions that disagree about them. None of it is a
//! calendar; all of it is what a calendar prints in the margins.
//!
//! * [`solar_terms`] — 二十四節気, the 24 solar terms, in both orderings,
//!   with the 節気 / 中気 split the lunisolar leap rule depends on.
//! * [`pentads`] — 七十二候, the 72 pentads, with both the classical Chinese
//!   and the 1874 Japanese name sets.
//! * [`zassetsu`] — 雑節: 節分, 彼岸, 社日, 八十八夜, 入梅, 半夏生, 土用 with
//!   its 丑の日, 二百十日 and 二百二十日, each with its rule as data.
//! * [`rokuyo`] — 六曜, the six-day cycle Japanese calendars print.
//! * [`san_fu`] — 三伏 and 數九, the Chinese dog days and the nine nines,
//!   counted in 庚 days and in nines from the solstices.
//! * [`cold_food`] — 寒食, the Cold Food Day, under each reckoning: 105
//!   days after the winter solstice, the eve of 清明, and Korea's 한식.
//! * [`quarter_days`] — the quarter days and term days of England and
//!   Wales, Ireland and Scotland, each tradition's four fixed dates.
//! * [`dog_days`] — the European dog days, under each convention that
//!   dates them.
//! * [`moon_calendar`] — phase names, 月齢, illuminated fraction, the four
//!   principal phases of a month, 十五夜 and 十三夜, and the National
//!   Astronomical Observatory's 伝統的七夕.
//! * [`seasons`] — the four seasons, astronomical, meteorological and East
//!   Asian.
//! * [`zodiac`] — 黄道十二宮: the ecliptic cut into twelve, tropically
//!   (the Western signs), siderally (the Indian rāśi, and the solar months
//!   the Tamil, Bengali and Malayalam calendars take from them) and as the
//!   Chinese 十二次.
//! * [`lunisolar`] — a minimal month-and-day derivation, kept rather than
//!   routed through `hc-calendars-lunar` for measured reasons the module
//!   itself records.
//!
//! The solar terms, the pentads, the meridians and the zodiac are written up
//! in `docs/systems/solar-terms-and-pentads.md` in the repository — the
//! rules, a worked example, what is carried, how the instants compare with
//! the published almanacs, and the sources, keyed in `docs/references.bib`.
//! The module pages summarise it and state the code's own facts.
//!
//! # A day is not an instant, and a meridian is not optional
//!
//! Everything here begins as an astronomical instant in Universal Time and
//! ends as a calendar day. The step between the two is a choice of meridian,
//! and it is a choice that changes answers: Beijing is an hour behind Tokyo,
//! so the same solar term lands on different dates in the Chinese and the
//! Japanese almanac about once a year, and the same is true of a lunar month
//! boundary.
//!
//! So no function in this crate that turns an instant into an [`Rd`]
//! guesses. They all take a [`Meridian`], and the named ones — [`Meridian::JAPAN`],
//! [`Meridian::CHINA`] — are the meridians the respective national almanacs
//! are computed at.
//!
//! ```
//! use hc_seasons::{Meridian, SolarTerm};
//! use hc_seasons::solar_terms::term_day;
//!
//! // 立春 2024 fell on 4 February in Japan.
//! assert_eq!(
//!     term_day(2024, SolarTerm::BEGINNING_OF_SPRING, Meridian::JAPAN),
//!     hc_calendar::Rd(738_920)
//! );
//! ```
//!
//! # Accuracy
//!
//! The underlying solar longitude is `hc-astro`'s VSOP87 series, good to
//! about 1″, and an event within about a minute of local midnight can still
//! be given the wrong *day*. That is measured rather than asserted: the
//! integration tests compare against the 240 equinox days Japan published
//! for 1980–2099, and the document states the rate and the minute-by-minute
//! comparison with the 暦要項.
//!
//! The Moon is better — conjunctions land within about a minute — so the
//! lunar month boundaries and the phase dates are firmer than the solar-term
//! dates.

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod cold_food;
pub mod dog_days;
pub mod lunisolar;
pub mod meridian;
pub mod moon_calendar;
pub mod pentads;
pub mod quarter_days;
pub mod rokuyo;
pub mod san_fu;
pub mod seasons;
pub mod solar_terms;
pub mod zassetsu;
pub mod zodiac;

mod gregorian;

pub use cold_food::ColdFoodConvention;
pub use dog_days::DogDaysConvention;
pub use lunisolar::LunisolarDay;
pub use meridian::Meridian;
pub use moon_calendar::PhaseName;
pub use pentads::{Pentad, PentadPosition, PentadTradition};
pub use quarter_days::{QuarterDay, QuarterDayTradition};
pub use rokuyo::Rokuyo;
pub use seasons::{Hemisphere, Season, SeasonDefinition};
pub use solar_terms::{SolarTerm, TermKind, TermOrder};
pub use zassetsu::{Zassetsu, ZassetsuRule};
pub use zodiac::{
    Ayanamsa, ChineseStation, Element, Modality, Rashi, RulingPlanet, SiderealSign, SignPeriod,
    SolarMonthTradition, TropicalSign,
};

pub use hc_astro;
pub use hc_calendar;
pub use hc_calendar::Rd;
pub use hc_calendar::fixed::Moment;
