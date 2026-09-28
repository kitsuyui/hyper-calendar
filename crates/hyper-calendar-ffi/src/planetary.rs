//! Time on other bodies, behind the `planetary` feature: Mars time, the
//! surface missions' sol counts, and the solar day and local mean solar
//! time of every body in `hc-planetary`'s table.
//!
//! The calendars of other bodies' days are [`hc_circad_date`], one entry
//! point beside [`hc_mars_time`] with its own line, not new columns of an
//! existing one.

hc::exports!("planetary", c_exports);
