//! Holidays and observances, as data.
//!
//! **A holiday is data, never code.** This crate provides one evaluator and
//! one rule vocabulary; every country and every religious tradition in it is
//! a table of rule values, and adding a country adds no branch to the
//! engine. Where a country's law says something the vocabulary has no
//! shape for — Oman's compensation days, Russia's transfers — the table
//! carries it as a [`Rule::Computed`] or [`Rule::Tabulated`] function beside
//! the table, named after the country it serves, and the engine calls it
//! like any other rule without knowing whose it is. The days a decree, a
//! gazette or an exchange announces year by year — Vietnam's notices,
//! China's arrangements — are not code at all but a [`Listing`], read by a
//! [`Rule::Listed`]. A caller who wants a company calendar, a school
//! year or a fictional setting supplies their own [`RuleSet`] and gets the
//! same machinery.
//!
//! | Module | What it holds |
//! | --- | --- |
//! | [`rule`] | the rule vocabulary and the observance modifiers |
//! | [`group`] | the groups of people a day can be given to alone |
//! | [`computus`] | Easter, Gregorian and Julian, the astronomical reckoning at Jerusalem, and the offsets keyed to it |
//! | [`engine`] | evaluation, and business-day arithmetic |
//! | [`hindu`] | the Hindu festival rules the traditions and the national tables share |
//! | [`traditions`] | the cross-cutting religious cycles |
//! | [`roman_calendar`] | the General Roman Calendar, every celebration with its rank |
//! | [`roman_calendar_1960`] | the General Roman Calendar of 1960, the 1962 Missal's, every day with its class |
//! | [`common_worship`] | the Church of England's *Common Worship* calendar, its ranks and its required transfers |
//! | [`holy_years`] | the Catholic Holy Years, each from its bull of indiction |
//! | [`lectionary`] | the lectionary cycles: the Sunday and weekday years and the RCL's Propers |
//! | [`orthodox_fasts`] | the Eastern Orthodox fasting seasons, weekly fasts and fast-free weeks, on the Julian and the Revised Julian reckoning |
//! | [`international`] | the United Nations international days, each citing its resolution |
//! | [`countries`] | the national tables |
//! | [`exchanges`] | the trading calendars of stock exchanges |
//!
//! # What the engine will not do
//!
//! * **It will not claim a Hijri holiday is exact.** Eid depends on a
//!   crescent sighting decided per country, sometimes on the night before.
//!   Every Hijri-dated entry here is flagged [`Confidence::Approximate`]: it
//!   is a good prediction, not an announcement.
//! * **It will not invent substitution rules it cannot cite.** A country
//!   whose weekend-substitution law is not in front of the author carries no
//!   policy, and its holidays fall on the weekend and stay there.
//! * **It will not pretend a holiday list is current.** Every table carries
//!   a [`SourceDate`] and names its sources: the statute, gazette or
//!   official list where one was read, and, where the only source read was
//!   secondary — an encyclopaedia, a newspaper, an aggregator — a `sources`
//!   string that says so.
//! * **It will not guess outside the span it evaluated.** Business-day
//!   arithmetic that walks off the end of a [`HolidayCalendar`] returns
//!   `None`.
//!
//! # Example
//!
//! ```
//! use hc_calendar::Rd;
//! use hc_calendars_solar::gregorian;
//! use hc_holiday::countries::JAPAN;
//! use hc_holiday::engine::HolidayCalendar;
//!
//! let calendar = HolidayCalendar::for_year(&JAPAN, None, 2020);
//!
//! // 海の日 was moved to 23 July for the Tokyo Olympics.
//! let marine_day = gregorian::to_fixed(2020, 7, 23)?;
//! assert_eq!(calendar.name_on(marine_day), Some("Marine Day"));
//!
//! // And the third Monday of July, where it would otherwise have been,
//! // was an ordinary working day.
//! let third_monday = gregorian::to_fixed(2020, 7, 20)?;
//! assert!(calendar.is_business_day(third_monday));
//! # Ok::<(), hc_calendar::CalendarError>(())
//! ```
//!
//! # Provenance and accuracy
//!
//! Japan is complete and exact from the 1948 祝日法 to today, amendment by
//! amendment. 春分の日 and 秋分の日 are computed from `hc-seasons` at the
//! Japan meridian rather than tabulated, which is right in principle and,
//! over 1980–2099, agrees with every date the National Astronomical
//! Observatory has published. Everything else is stated in `README.md`,
//! including which entries are predictions.

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod common_worship;
pub mod computus;
pub mod group;
pub mod hindu;
pub mod holy_years;
pub mod international;
pub mod lectionary;
pub mod orthodox_fasts;
pub mod rule;

#[cfg(feature = "alloc")]
pub mod countries;
#[cfg(feature = "alloc")]
pub mod engine;
#[cfg(feature = "alloc")]
pub mod exchanges;
#[cfg(feature = "alloc")]
pub mod roman_calendar;
#[cfg(feature = "alloc")]
pub mod roman_calendar_1960;
#[cfg(feature = "alloc")]
pub mod traditions;

pub use computus::{Computus, easter, gregorian_easter, orthodox_easter};
pub use group::Group;
#[cfg(feature = "alloc")]
pub use rule::EvaluationContext;
pub use rule::{
    BridgePolicy, CalendarSystem, Confidence, Days, HolidayRule, Kind, ListedEntry, Listing,
    ListingKey, Rule, RuleSet, Scope, SourceDate, SubstituteDirection, SubstitutionPolicy,
    TibetanMonth, WeekendPolicy,
};

#[cfg(feature = "alloc")]
pub use engine::{Gap, Holiday, HolidayCalendar, holidays_in_year, holidays_on, is_holiday};

pub use hc_calendar::Rd;
