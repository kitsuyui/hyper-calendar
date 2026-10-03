//! Python's `humanize`, behind the `natural` feature: the number, size,
//! list and time functions, in the language of the catalogue that serves the
//! locale, from `hc-humanize`'s `natural`. The lines are
//! `hyper_calendar::humanize_lines`', shared with the C library.

hc::exports!("natural", c_exports);
