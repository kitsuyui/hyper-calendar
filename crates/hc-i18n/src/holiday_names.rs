//! What a locale calls a day of a holiday table, beside the table's own
//! English and local names, where a source in the language names it.
//!
//! The tables are `hc-holiday`'s, each named by its code and its days by
//! their identifiers there (`hc_holiday::id`), which a line of
//! `hc_holidays_on` carries beside the English name. A name is carried only where a source prints
//! it; nothing here is translated.
//!
//! # What is carried
//!
//! * **`cop`**, Coptic in its Bohairic form, for `coptic-orthodox`, whose
//!   local names are the Arabic the church's publications use: seven feasts
//!   whose Coptic names a source prints. Nayrouz, the new year, is
//!   ⲡⲓⲭⲗⲟⲙ ⲛ̀ⲧⲉ ϯⲣⲟⲙⲡⲓ, "the crown of the year", as Wikipedia's "Nayrouz"
//!   gives "The recorded Bohairic name for the new year"
//!   [wikipedia-nayrouz]; the Feast of the Cross, the Circumcision,
//!   Theophany, the Transfiguration and the Assumption are the names the
//!   Coptic Wikipedia's test project prints for 17 Thout, 6 and 11 Tobi,
//!   13 and 16 Mesori in its pages of the months, and Easter is its page
//!   "Ⲡⲁⲥⲭⲁ", "ϯⲀⲛⲁⲥⲧⲁⲥⲓⲥ ⲉ̀ⲑⲟⲩⲁⲃ" [incubator-coptic-wikipedia]. Both are
//!   secondary and edited by their readers; no church publication read
//!   prints the names in Coptic script. The other feasts of the table —
//!   the Nativity, the Wedding at Cana, the Entry into the Temple, the
//!   second Cross, the Annunciation, the Entry into Egypt, the Apostles,
//!   the Dormition, the paschal cycle but Easter — have no name here: no
//!   source read prints one.
//!
//! # Locales
//!
//! [`holiday_name`] walks [`Locale::fallback`] and takes the first table in
//! the chain that names the day, as [`crate::holiday_groups`] does.

use crate::locale::Locale;

/// One locale's names for the days of one holiday table.
#[derive(Debug, Clone, Copy)]
pub struct HolidayNames {
    /// The BCP 47 tag, spelled as [`crate::data::LOCALES`] spells it.
    pub tag: &'static str,
    /// The code of the `hc-holiday` table the days are of.
    pub table: &'static str,
    /// Each named day: its identifier in the table, and its name.
    pub names: &'static [(&'static str, &'static str)],
}

/// A day's name and the tag of the table that gave it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HolidayName {
    /// What the locale calls the day.
    pub name: &'static str,
    /// The tag of the table that answered.
    pub tag: &'static str,
}

/// What `locale` calls the day of the holiday table `table` whose
/// identifier is `id`, from the first table in its fallback chain that
/// names it; `None` when none does. The table's code and the identifier
/// match without regard to ASCII case, white space around them ignored.
#[must_use]
pub fn holiday_name(locale: &Locale, table: &str, id: &str) -> Option<HolidayName> {
    locale.fallback().find_map(|candidate| {
        let rendered = candidate.rendered()?;
        TABLES
            .iter()
            .filter(|names| {
                rendered.as_str() == names.tag && names.table.eq_ignore_ascii_case(table.trim())
            })
            .find_map(|names| {
                names
                    .names
                    .iter()
                    .find(|(day, _)| day.eq_ignore_ascii_case(id.trim()))
                    .map(|(_, name)| HolidayName {
                        name,
                        tag: names.tag,
                    })
            })
    })
}

/// Every table, in tag order.
pub static TABLES: &[HolidayNames] = &[HolidayNames {
    tag: "cop",
    table: "coptic-orthodox",
    names: &[
        ("nayrouz-new-year", "ⲡⲓⲭⲗⲟⲙ ⲛ̀ⲧⲉ ϯⲣⲟⲙⲡⲓ"),
        ("feast-of-the-cross", "Ⲡⲓϫⲓⲛⲟⲩⲱⲛϩ ⲛ̀ⲧⲉⲡⲓⲥⲧⲁⲩⲣⲟⲥ ⲉ̀ⲑⲟⲩⲁⲃ"),
        ("circumcision-of-the-lord", "Ⲡⲓⲉⲣⲫⲙⲉⲩⲓ̀ ⲛ̀ⲧⲉⲡⲥⲉⲃⲓ ⲙ̀Ⲡϭⲱⲓⲥ"),
        ("theophany-epiphany", "Ⲑⲉⲟ̀ⲫⲁⲛⲓⲁ̀ ⲉ̀ⲑⲟⲩⲁⲃ"),
        (
            "transfiguration",
            "Ⲡϣⲁⲓ ⲙ̀ⲡⲓϣⲓⲃϯⲭⲉⲣⲉⲃ ⲛ̀ⲧⲉⲡⲉⲛϭⲟⲓⲥ ϩⲓϫⲉⲛⲡⲓⲧⲱⲟⲩ ⲛ̀Ⲑⲁⲃⲱⲣ",
        ),
        (
            "assumption-of-st-mary",
            "Ⲡⲓϫⲓⲛⲉⲣⲁⲛⲁⲗⲩⲙⲯⲓⲥ ⲙ̀ⲡⲓⲥⲱⲙⲁ ⲛ̀ⲧⲉϯⲡⲁⲣⲑⲉⲛⲟⲥ ⲉ̀ⲑⲟⲩⲁⲃ Ⲙⲁⲣⲓⲁ̀",
        ),
        ("easter-resurrection", "ϯⲀⲛⲁⲥⲧⲁⲥⲓⲥ ⲉ̀ⲑⲟⲩⲁⲃ"),
    ],
}];

#[cfg(test)]
mod tests {
    use super::*;

    fn locale(tag: &str) -> Locale {
        Locale::parse(tag).expect("a tag")
    }

    #[test]
    fn every_table_is_a_carried_locale_and_names_each_day_once() {
        for table in TABLES {
            assert!(
                crate::data::LOCALES
                    .iter()
                    .any(|data| data.tag == table.tag)
            );
            for (index, (day, name)) in table.names.iter().enumerate() {
                assert!(!name.is_empty());
                assert!(
                    !table.names[index + 1..]
                        .iter()
                        .any(|(other, _)| other == day)
                );
            }
        }
    }

    #[test]
    fn coptic_names_the_new_year_and_nothing_else_answers() {
        let nayrouz = holiday_name(&locale("cop"), "coptic-orthodox", "nayrouz-new-year");
        assert_eq!(
            nayrouz,
            Some(HolidayName {
                name: "ⲡⲓⲭⲗⲟⲙ ⲛ̀ⲧⲉ ϯⲣⲟⲙⲡⲓ",
                tag: "cop"
            })
        );
        assert_eq!(
            holiday_name(&locale("cop-EG"), "Coptic-Orthodox", " Nayrouz-New-Year "),
            nayrouz
        );
        assert_eq!(
            holiday_name(&locale("cop"), "coptic-orthodox", "nativity-christmas"),
            None
        );
        assert_eq!(
            holiday_name(&locale("ar"), "coptic-orthodox", "nayrouz-new-year"),
            None
        );
    }
}
