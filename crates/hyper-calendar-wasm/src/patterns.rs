//! Patterns read against a text, behind the `patterns` feature: Python's
//! `strptime`, POSIX's and CLDR's, in the C locale's names or a locale's,
//! from `hc-format`. The lines are `hyper_calendar::datetime_lines`' and
//! `i18n_lines`', shared with the C library.

hc::exports!("patterns", w_exports);
