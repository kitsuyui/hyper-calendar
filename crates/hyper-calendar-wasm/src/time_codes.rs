//! The CCSDS and radio time codes, behind the `time-codes` feature: the
//! binary CCSDS codes and the ASCII codes A and B, and the long-wave radio
//! codes of JJY, DCF77 and WWVB, read and written. A layer of its own, so
//! that `timestamps` stays small for a page that needs only POSIX, ISO and
//! TAI conversions. The lines are `hyper_calendar::time_code_lines`',
//! shared with the C library.

hc::exports!("time-codes", w_exports);
