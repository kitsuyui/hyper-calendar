//! ISO 8601 beyond a calendar date, behind the `datetime` feature:
//! date-times with a zone, durations, intervals and repeating intervals, week
//! and ordinal dates, RFC 3339, RFC 2822 and HTTP dates, Python's `isoformat`,
//! and `strptime` and CLDR patterns read in the C locale; from `hc-format`.
//! The lines are `hyper_calendar::datetime_lines`', shared with the C library.

hc::exports!("datetime", w_exports);
