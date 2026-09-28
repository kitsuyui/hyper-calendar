//! Place names, behind the `places` feature: what each locale `hc-i18n`
//! carries calls every territory and every ISO 3166-2 subdivision CLDR 48
//! names, with the English name, the locale that answered, the value's
//! draft level and the code's validity status. The lines are
//! `hyper_calendar::place_lines`', shared with the WebAssembly module.

hc::exports!("places", c_exports);
