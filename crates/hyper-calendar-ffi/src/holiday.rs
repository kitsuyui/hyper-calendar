//! The holiday tables, behind the `holiday` feature: every country,
//! exchange, tradition and international set of `hc-holiday`, looked up by
//! identifier and rendered as tab-separated lines.
//!
//! The tables and the liturgical year, behind the `holiday` feature: every
//! holiday table described, the lectionary cycles of a day, the
//! astronomical Easter, the Holy Year a day falls in and the ranks of the
//! *Common Worship* celebrations of a day, from
//! `hyper_calendar::holiday_lines`, shared with the WebAssembly module.
//!
//! The Eastern Orthodox fasts, behind the `holiday` feature. The lines are
//! `hyper_calendar::holiday_lines`', shared with the WebAssembly module.

hc::exports!("holiday", c_exports);
