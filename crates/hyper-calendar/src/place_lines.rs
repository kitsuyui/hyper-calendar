//! The tab-separated lines the WebAssembly module and the C library write
//! about what places are called, written once.
//!
//! A line is one territory or one ISO 3166-2 subdivision of
//! [`hc_i18n::place_names`], CLDR 48's names in every locale `hc-i18n`
//! carries: the code as ISO writes it; the name in the locale; the English
//! name; the tag of the data that named it; that value's CLDR draft level;
//! and the code's CLDR validity status. `docs/systems/place-names.md`
//! explains the data, the lookup and how the lines stand beside
//! `hc_holiday_tables`' column 9 and the countries of `hc_zones`.

use alloc::string::String;

use hc_i18n::Locale;
use hc_i18n::place_names::{self, NamedPlace, Places};

use crate::boundary::{Answer, Line, Refusal};

/// How many columns every line of [`territories`], [`subdivisions`] and
/// [`place_name`] has.
pub const PLACE_COLUMNS: usize = 6;

/// The locale a tag asks for: `None` for `native`, which names no one
/// locale and asks for English, and the root locale, which reaches
/// English too, for a tag that does not parse.
fn requested_locale(tag: &str) -> Option<Locale> {
    if tag == "native" {
        None
    } else {
        Some(Locale::parse(tag).unwrap_or(Locale::ROOT))
    }
}

/// One line per place: the code; the name in the locale; the English
/// name; the tag that answered; the draft level; and the status.
fn push_lines(out: &mut String, places: Places, locale: &str) {
    let requested = requested_locale(locale);
    for NamedPlace {
        place,
        name,
        english,
    } in place_names::named(places, requested.as_ref())
    {
        let mut line = Line::new(out);
        line.cell(place.code())
            .cell_or_empty(name.map(|name| name.name))
            .cell_or_empty(english)
            .cell_or_empty(name.map(|name| name.tag))
            .cell_or_empty(name.map(|name| name.draft.name()))
            .cell(place.status().name());
        line.end();
    }
}

/// The lines of `hc_territories`: one per territory CLDR 48 names — the
/// ISO 3166-1 countries, the UN M.49 areas such as `001` and `419`, and
/// CLDR's `EU`, `EZ`, `UN`, `QO`, `XA`, `XB` and `ZZ` — in code order.
///
/// Each line is the code; the name in the locale — 日本 for `JP` under
/// `ja` — where the locale's chain names it, else CLDR's English name;
/// the English name, `en.xml`'s; the tag of the data that answered, `ja`
/// for a request for `ja-JP`, or `en`; the draft level of that value,
/// `approved`, `contributed` or `provisional`; and the code's status in
/// CLDR's validity data, `regular`, `macroregion`, `special` or `unknown`.
/// A tag that does not parse, one whose chain reaches no table, and
/// `native` answer in English. Every territory has a name, so no cell but
/// none is empty.
#[must_use]
pub fn territories(locale: &str) -> String {
    let mut out = String::new();
    push_lines(&mut out, place_names::territories(), locale);
    out
}

/// The lines of `hc_subdivisions`: one per ISO 3166-2 subdivision of
/// `country` that a carried locale's CLDR 48 file names, in code order,
/// in [`territories`]' columns; with no `country`, every one of the
/// 5 503, country by country.
///
/// Column 1 is the code as ISO writes it, `JP-13`, where CLDR's id is
/// `jp13`. Column 2 is the name in the locale: 東京都 under `ja`, where
/// the value is CLDR's provisional one, as nearly every subdivision name
/// outside English is. Column 6 is `regular` for the 5 027 codes in use
/// and `deprecated` for 476 CLDR keeps from earlier ISO lists; 104 of
/// those English does not name, whose columns 2 to 5 are empty where the
/// locale does not name them either.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a `country` that is not a territory's code. A
/// territory with no subdivisions, `AQ` or `001`, writes nothing.
pub fn subdivisions(country: Option<&str>, locale: &str) -> Answer<String> {
    let places = match country {
        Some(country) => place_names::subdivisions_of(country).ok_or(Refusal::Unknown)?,
        None => place_names::subdivisions(),
    };
    let mut out = String::new();
    push_lines(&mut out, places, locale);
    Ok(out)
}

/// The line of `hc_place_name`: [`territories`]' or [`subdivisions`]'
/// line for one code, `JP` or `JP-13`, in any case.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a code that is neither a territory nor a
/// subdivision the data names, CLDR's own form `jp13` included.
pub fn place_name(code: &str, locale: &str) -> Answer<String> {
    let place = place_names::place(code).ok_or(Refusal::Unknown)?;
    let mut out = String::new();
    push_lines(&mut out, Places::from(place), locale);
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// CLDR 48 `subdivisions/ja.xml` `jp13` 東京都 and `en.xml` Tokyo;
    /// `zh_Hant.xml`, whose parent is root, names three subdivisions, so
    /// Tokyo is English's under `zh-TW`; `native` asks for English.
    #[test]
    fn a_line_is_six_cells() {
        assert_eq!(
            place_name(" jp-13 ", "ja").as_deref(),
            Ok("JP-13\t東京都\tTokyo\tja\tprovisional\tregular\n")
        );
        assert_eq!(
            place_name("JP-13", "zh-TW").as_deref(),
            Ok("JP-13\tTokyo\tTokyo\ten\tapproved\tregular\n")
        );
        assert_eq!(
            place_name("JP-13", "native").as_deref(),
            Ok("JP-13\tTokyo\tTokyo\ten\tapproved\tregular\n")
        );
        assert_eq!(
            place_name("FR-75", "ja").as_deref(),
            Ok("FR-75\t\t\t\t\tdeprecated\n")
        );
        assert_eq!(place_name("jp13", "ja"), Err(Refusal::Unknown));
        assert_eq!(subdivisions(Some("JPN"), "ja"), Err(Refusal::Unknown));
        assert_eq!(subdivisions(Some("AQ"), "ja").as_deref(), Ok(""));
        for text in [
            territories("ja"),
            subdivisions(None, "ja").unwrap_or_default(),
        ] {
            assert!(
                text.lines()
                    .all(|line| line.split('\t').count() == PLACE_COLUMNS)
            );
        }
    }

    /// Every subdivision a holiday table's rules are scoped to, column 9
    /// of `hc_holiday_tables`, and every country with a table, is a place
    /// named in Japanese or in English and in use today, but one; a
    /// municipality's code is not a CLDR place, and is left out. CLDR 48's
    /// validity data holds `gbeaw`, England and Wales, as deprecated. The
    /// department of Guatemala is `gt01`, which `en.xml` names Guatemala,
    /// not the deprecated `gtgu`.
    #[cfg(feature = "holiday")]
    #[test]
    fn every_place_a_holiday_table_names_has_a_line() {
        let tables = crate::holiday_lines::holiday_tables("en");
        let mut codes = 0;
        let mut deprecated = alloc::vec::Vec::new();
        for line in tables.lines() {
            let cells: alloc::vec::Vec<&str> = line.split('\t').collect();
            // A municipality's code (ADR 0014) has no CLDR name; its
            // subdivision's is held to the rule.
            let regions = cells[8]
                .split(';')
                .filter(|code| !code.is_empty() && hc_holiday::rule::region_parent(code).is_none());
            let country = (cells[1] == "country").then_some(cells[0]);
            for code in regions.chain(country) {
                codes += 1;
                let answer = place_name(code, "ja").expect(code);
                let cells: alloc::vec::Vec<&str> = answer.trim_end().split('\t').collect();
                assert_eq!(cells[0], code);
                if cells[5] != "regular" || cells[1].is_empty() {
                    deprecated.push(alloc::format!("{code} {} {}", cells[5], cells[2]));
                }
            }
        }
        assert!(codes > 195, "{codes}");
        assert_eq!(deprecated, ["GB-EAW deprecated England and Wales"]);
    }
}
