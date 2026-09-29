//! Time zone names, behind the `zone-names` feature: a zone's name at an
//! instant in a locale, as a CLDR pattern's `z`, `O`, `v` and `V` fields
//! write it, from CLDR 48's metazones and zone names in every locale
//! `hc-i18n` carries. The line is `hyper_calendar::zone_lines`', shared with
//! the WebAssembly module.

hc::exports!("zone-names", c_exports);
