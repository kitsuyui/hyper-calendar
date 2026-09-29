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
//! * [`plural`] — CLDR cardinal plural categories for 53 languages and
//!   European Portuguese.
//! * [`names`] — month, weekday, day-period, era, quarter and sexagenary
//!   vocabulary, keyed by locale, calendar, width and context, and the
//!   templates a locale writes a date with.
//! * [`notation`] — the notations a calendar writes its dates in whatever
//!   the language: the Long Count's dotted places, the ISO week date.
//! * [`fields`] — what a calendar's extra fields are called, and which of
//!   its cycles names each one's value.
//! * [`almanac`] — what a locale calls the Japanese almanac's annotations,
//!   六曜 to the 選日.
//! * [`dated`] — month and weekday names a government gave for a period,
//!   with the days they were in force: Turkmenistan's of 2002–2008.
//! * [`reckonings`] — what a locale calls the terms of the other
//!   reckonings of a day and a year: the choghadiya, the Panchak kinds, the
//!   Kumbh sites and Pushkaram rivers, the planets of the planetary hours,
//!   the night watches, and the Vietnamese, Chinese and Turkish folk days.
//! * [`horizons`] — what a locale calls a horizon a rising is measured
//!   against, where an observatory or almanac office names it.
//! * [`holiday_groups`] — what a locale calls a group of people a holiday
//!   is given to alone, where an instrument in the language names it.
//! * [`holiday_names`] — what a locale calls a day of a holiday table
//!   beside the table's own names, where a source in the language prints
//!   it: the Coptic names of the Coptic Orthodox feasts.
//! * [`day_periods`] — the day periods beyond am and pm, midnight, noon and
//!   the flexible *in the morning* and *at night*, with each language's
//!   rules for when they begin.
//! * [`direction`] — script direction and the bidi isolation a formatter
//!   needs when it embeds a date in text running the other way.
//! * [`casing`] — the locale-dependent parts of upper/lower/title casing.
//! * `exemplar_cities` — with the `exemplar-cities` feature, CLDR's
//!   English exemplar cities for the zones `hc-tz` locates, and with
//!   `localized-exemplar-cities` those of every other carried locale.
//! * `place_names` — with the `territories` feature, CLDR's names of every
//!   territory in every carried locale, with their `alt` forms, which the
//!   holiday tables and the zone names name countries by; with
//!   `place-names`, those of every ISO 3166-2 subdivision too, and which
//!   lies in which.
//! * [`zone_names`] — each locale's time zone formats, and with the
//!   `zone-names` feature CLDR's metazones and English's zone names, with
//!   `localized-zone-names` every other carried locale's.
//!
//! # Scope
//!
//! This is a *calendar* internationalisation crate, not a general one. It
//! carries no collation, no message formatting, no number grouping or
//! currency, no compact-notation plural operands (`c`/`e`), and no
//! transliteration. Its data is a subset of CLDR, not a copy of it: the
//! entries carried before the most-spoken languages are hand-checked, and
//! those of the twelve locales added for them are generated from their CLDR
//! 48 files by CLDR's inheritance; see the crate README for the exact
//! provenance and for what the subset leaves out.

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod almanac;
pub mod casing;
pub mod data;
pub mod dated;
pub mod day_periods;
pub mod direction;
pub mod error;
#[cfg(feature = "exemplar-cities")]
pub mod exemplar_cities;
pub mod fields;
pub mod holiday_groups;
pub mod holiday_names;
pub mod horizons;
pub mod locale;
pub mod names;
pub mod notation;
pub mod numbering;
#[cfg(feature = "territories")]
pub mod place_names;
pub mod plural;
pub mod reckonings;
pub mod zone_names;

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
