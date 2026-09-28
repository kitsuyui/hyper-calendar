//! Time on other bodies, behind the `planetary` feature: Mars time, the
//! surface missions' sol counts, and the solar day and local mean solar
//! time of every body in `hc-planetary`'s table.
//!
//! Titan's and the Galilean moons' circad calendars and the Martiana
//! calendar are [`hc_circad_date`], one export with its own line beside
//! [`hc_mars_time`], not new columns of an existing one.

hc::exports!("planetary", w_exports);
