//! The holiday tables, behind the `holiday` feature: every country,
//! exchange, tradition and international set of `hc-holiday`, looked up by
//! identifier and rendered as tab-separated lines.
//!
//! The tables and the liturgical year, behind the `holiday` feature: every
//! holiday table described, the lectionary cycles of a day, the
//! astronomical Easter, the Holy Year a day falls in and the ranks of the
//! *Common Worship* celebrations of a day. The lines are
//! `hyper_calendar::holiday_lines`', shared with the C library.
//!
//! The Eastern Orthodox fasts, behind the `holiday` feature: the fast a day
//! falls in and the fasting seasons of a year, under each reckoning of
//! `hc-holiday`'s `orthodox_fasts`. The lines are
//! `hyper_calendar::holiday_lines`', shared with the C library.

hc::exports!("holiday", w_exports);
