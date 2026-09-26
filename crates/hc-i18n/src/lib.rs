//! Internationalisation (i18n), multilingualisation (m17n) and localisation
//! (L10n) for calendar data.
//!
//! # What this crate is for
//!
//! A calendar computes; a locale decides what the computation is *called*.
//! `hc-calendar` can tell you that a day is month 9 of year 2024 in the
//! Gregorian calendar; only a locale can tell you whether to print that as
//! "September", "сентября", "сентябрь", "eylül" or "９月". This crate owns
//! that half, and nothing else in the workspace is allowed to hard-code a
//! localised string.
//!
//! # Data is not code
//!
//! Every locale is one [`names::LocaleData`] value of `&'static` slices in
//! [`data::LOCALES`]. Lookup walks the [`locale::Locale::fallback`] chain and
//! takes the first entry that actually carries the field asked for, so a
//! language is added by appending one entry to that table — no function in
//! this crate learns a new branch. The same split holds for
//! [`numbering::NumberingSystem`] (digit tables plus four Han numeral styles)
//! and for [`plural::PluralRules`] (one CLDR rule function per language in a
//! table keyed by language subtag).
//!
//! # Modules
//!
//! * [`locale`] — BCP 47 tags with the `-u-ca`/`-nu`/`-fw`/`-hc` keys, and
//!   the CLDR inheritance chain as an iterator.
//! * [`numbering`] — digit shapes and Han numerals, rendered and parsed.
//! * [`plural`] — CLDR cardinal plural categories for 30 languages.
//! * [`names`] — month, weekday, day-period, era, quarter and sexagenary
//!   vocabulary, keyed by locale, calendar, width and context.
//! * [`direction`] — script direction and the bidi isolation a formatter
//!   needs when it embeds a date in text running the other way.
//! * [`casing`] — the locale-dependent parts of upper/lower/title casing.
//! * `territories` — with the `territories` feature, CLDR's names for the
//!   regions the workspace keeps holiday tables for.
//!
//! # Scope
//!
//! This is a *calendar* internationalisation crate, not a general one. It
//! carries no collation, no message formatting, no number grouping or
//! currency, no compact-notation plural operands (`c`/`e`), and no
//! transliteration. Its data is a hand-checked subset of CLDR, not a
//! generated copy of it; see the crate README for the exact provenance and
//! for what "a hand-checked subset" leaves out.

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod casing;
pub mod data;
pub mod direction;
pub mod error;
pub mod locale;
pub mod names;
pub mod numbering;
pub mod plural;
#[cfg(feature = "territories")]
pub mod territories;

mod util;

pub use direction::Direction;
pub use error::{I18nError, I18nResult};
pub use locale::{HourCycle, Locale};
pub use names::{
    CalendarNames, ContextualNames, DayPeriod, EraNames, LeapMonthNames, LocaleData, NameContext,
    NameWidth, WidthSet,
};
pub use numbering::NumberingSystem;
pub use plural::{PluralCategory, PluralOperands, PluralRules};

pub use hc_calendar;
