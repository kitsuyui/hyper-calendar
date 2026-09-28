//! Every calendar, behind the `calendars` feature: the registry the facade
//! populates, described for one day, walked as eras, years, months and
//! days, and listed, in the vocabulary of a locale; the locales
//! themselves; when each country adopted the Gregorian calendar; and the
//! month and weekday names a government decreed for a period. The lines
//! are `hyper_calendar::lines`', shared with the WebAssembly module.
//!
//! Days in the calendars, behind the `calendars` feature: the pañcāṅga's
//! yoga and karaṇa, the Hindu lunisolar date at a place, the *Sūrya
//! Siddhānta*'s sky and sunrise, the young crescent by a named criterion,
//! the modern Olympiad of a year, the Hebrew anniversaries and sabbatical
//! cycle, and the days of the Asian calendar as it writes them, from
//! `hyper_calendar`'s `panchanga_lines`, `hindu_lines`, `crescent_lines`
//! and `calendar_values`, shared with the WebAssembly module.
//!
//! The parts of a day the calendars mark, behind the `calendars` feature:
//! Rāhu kālam, Yamaganda and Gulika kālam, and the almanac's cycles of a
//! day. The lines are `hyper_calendar`'s `panchanga_lines` and
//! `almanac_lines`, shared with the WebAssembly module.

hc::exports!("calendars", c_exports);
