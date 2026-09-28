//! The CCSDS and radio time codes, behind the `time-codes` feature: the
//! binary CCSDS codes and the ASCII codes A and B, and the long-wave radio
//! codes of JJY, DCF77 and WWVB, read and written. The lines are
//! `hyper_calendar::time_code_lines`', shared with the WebAssembly module.

hc::exports!("time-codes", c_exports);
