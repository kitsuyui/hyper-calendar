//! The almanac, behind the `seasons` feature: the 24 solar terms and the 72
//! pentads of `hc-seasons`, judged at a named meridian, and 寒食 under each
//! of its reckonings. The lines are `hyper_calendar::season_lines`', shared
//! with the WebAssembly module.

hc::exports!("seasons", c_exports);
