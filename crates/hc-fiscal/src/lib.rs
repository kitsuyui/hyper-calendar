//! Years that do not begin on 1 January.
//!
//! Fiscal years, tax years, academic years and the 52/53-week reporting
//! calendars retailers and broadcasters run on. The subject of this crate is
//! not a new calendar: it is the observation that a great many institutions
//! agree about what day it is and disagree about what *year* it is, and that
//! the disagreement is systematic enough to be data.
//!
//! # The two traps
//!
//! **The label.** Japan's 2024年度 runs 1 April 2024 to 31 March 2025. The
//! United States' FY 2024 ran 1 October **2023** to 30 September 2024,
//! because the United States names a fiscal year after the calendar year it
//! *ends* in. Both are written "FY2024". On 1 November 2023 Tokyo and
//! Washington disagree by a whole year about which fiscal year it is. This
//! crate makes the convention an explicit
//! [`LabelConvention`] with **no default**,
//! because a default here would take a position on the caller's behalf
//! silently — the failure mode `docs/policy.md` §5 exists to prevent.
//!
//! **The calendar.** "The year starts on 1 April" is not a complete rule
//! until the calendar is named. Iran's fiscal year begins at Nowruz, which is
//! 1 Farvardin in the Solar Hijri calendar and lands on 20 or 21 March
//! depending on where the equinox falls. Ethiopia's begins on Hamle 1 in the
//! Ethiopic calendar. So a [`YearStart`] carries a
//! [`StartCalendar`], and the non-Gregorian
//! entries go through the real calendars in `hc-calendars-solar` rather than
//! through a hard-coded "about 21 March".
//!
//! # The modules
//!
//! | Module | Subject |
//! | --- | --- |
//! | [`year_system`] | the core type: a start in a named calendar, a labelling convention, a validity range |
//! | [`quarters`] | quarters, halves and months *of the fiscal year* — Japan's Q1 is April, the United States' is October |
//! | [`countries`] | the national tables, each with a source and a check date |
//! | [`academic`] | school and university years, which differ again from fiscal ones and are mostly not statutory |
//! | [`retail`] | 4-4-5, 4-5-4, 5-4-4 and the 52/53-week year, as named conventions |
//!
//! # Example
//!
//! ```
//! use hc_calendars_solar::gregorian;
//! use hc_fiscal::FiscalError;
//! use hc_fiscal::countries::{JAPAN, UNITED_STATES};
//!
//! let day = gregorian::to_fixed(2023, 11, 1)?;
//! let japan = JAPAN.government(2023).ok_or(FiscalError::NoSystemInForce)?;
//! let us = UNITED_STATES.government(2024).ok_or(FiscalError::NoSystemInForce)?;
//!
//! // The same day, in two fiscal years a year apart. Both are written "FY".
//! assert_eq!(japan.label_at(day)?, 2023);
//! assert_eq!(us.label_at(day)?, 2024);
//! # Ok::<(), FiscalError>(())
//! ```
//!
//! # What this crate does not do
//!
//! * It does not know your company's fiscal year. It knows the named
//!   conventions; a filer's own 52/53-week year is a value the caller
//!   constructs, on the same terms as the ones shipped here.
//! * It does not compute business days, holidays or settlement dates. That
//!   is `hc-holiday`, and a fiscal year is deliberately independent of it:
//!   31 March is the end of Japan's 年度 whether or not it is a Sunday.
//! * It claims nothing about Iran's official calendar beyond what the
//!   arithmetic approximation in `hc-calendars-solar` supports. See
//!   [`countries::IRAN`].

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod academic;
pub mod countries;
pub mod error;
pub mod quarters;
pub mod retail;
pub mod year_system;

pub use error::{FiscalError, FiscalResult};
pub use quarters::{Half, PeriodSpan, Quarter};
pub use retail::{AnchorRule, PeriodShape, WeekYearSystem};
pub use year_system::{
    Authority, FiscalPosition, FiscalSpan, LabelConvention, SourceDate, StartCalendar, SystemKind,
    YearStart, YearSystem,
};

pub use hc_calendar;
pub use hc_calendars_solar;
