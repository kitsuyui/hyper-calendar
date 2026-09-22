//! Holidays and observances, as data.
//!
//! **A holiday is data, never code.** This crate provides one evaluator and
//! one rule vocabulary; every country and every religious tradition in it is
//! a table of rule values and nothing else. There is no function named after
//! a country anywhere in the source, and adding a country adds no branch to
//! the engine. A caller who wants a company calendar, a school year or a
//! fictional setting supplies their own [`RuleSet`] and gets the same
//! machinery.
//!
//! | Module | What it holds |
//! | --- | --- |
//! | [`rule`] | the rule vocabulary and the observance modifiers |
//! | [`computus`] | Easter, Gregorian and Julian, and the offsets keyed to it |
//! | [`engine`] | evaluation, and business-day arithmetic |
//! | [`traditions`] | the cross-cutting religious cycles |
//! | [`countries`] | the national tables |
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
//!   a [`SourceDate`] and names its statute or gazette.
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

pub mod computus;
pub mod hindu;
pub mod rule;

#[cfg(feature = "alloc")]
pub mod countries;
#[cfg(feature = "alloc")]
pub mod engine;
#[cfg(feature = "alloc")]
pub mod traditions;

pub use computus::{Computus, easter, gregorian_easter, orthodox_easter};
pub use rule::{
    BridgePolicy, CalendarSystem, Confidence, Days, HolidayRule, Kind, Phase, Rule, RuleSet,
    SourceDate, SubstituteDirection, SubstitutionPolicy, WeekendPolicy,
};

#[cfg(feature = "alloc")]
pub use engine::{Gap, Holiday, HolidayCalendar, holidays_in_year, is_holiday};

pub use hc_astro;
pub use hc_calendar;
pub use hc_calendar::Rd;
pub use hc_calendars_lunar;
pub use hc_calendars_solar;
pub use hc_seasons;
