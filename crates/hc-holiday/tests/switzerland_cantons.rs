//! Switzerland's cantons, law by law.
//!
//! Each anchor is a day a canton's law keeps in the whole canton beyond
//! the four every canton keeps, in the first year the table carries it
//! and in 2026, with the date its rule gives, worked out by a script apart
//! from this crate. `docs/systems/switzerland-holidays.md` lists the laws.

use hc_calendar::Rd;
use hc_calendars_solar::gregorian;
use hc_holiday::countries::SWITZERLAND;
use hc_holiday::engine::{Holiday, HolidayCalendar};
use hc_holiday::rule::Kind;

fn ymd(year: i64, month: u8, day: u8) -> Rd {
    match gregorian::to_fixed(year, month, day) {
        Ok(fixed) => fixed,
        Err(error) => panic!("{year}-{month:02}-{day:02} is not a date: {error:?}"),
    }
}

/// A canton's own entries in a year.
fn own_in_year(region: &str, year: i64) -> Vec<Holiday> {
    HolidayCalendar::for_year(&SWITZERLAND, Some(region), year)
        .in_year(year)
        .into_iter()
        .filter(|holiday| !holiday.regions.is_empty())
        .collect()
}

/// `(region, local name, year, month, day, kind)`.
const ANCHORS: &[(&str, &str, i64, u8, u8, Kind)] = &[
    ("CH-AG", "Karfreitag", 2013, 3, 29, Kind::Public),
    ("CH-AG", "Karfreitag", 2026, 4, 3, Kind::Public),
    ("CH-AI", "Karfreitag", 2011, 4, 22, Kind::Public),
    ("CH-AI", "Karfreitag", 2026, 4, 3, Kind::Public),
    ("CH-AI", "Ostermontag", 2011, 4, 25, Kind::Public),
    ("CH-AI", "Ostermontag", 2026, 4, 6, Kind::Public),
    ("CH-AI", "Pfingstmontag", 2011, 6, 13, Kind::Public),
    ("CH-AI", "Pfingstmontag", 2026, 5, 25, Kind::Public),
    ("CH-AI", "Fronleichnam", 2011, 6, 23, Kind::Public),
    ("CH-AI", "Fronleichnam", 2026, 6, 4, Kind::Public),
    ("CH-AI", "Maria Himmelfahrt", 1982, 8, 15, Kind::Observance),
    ("CH-AI", "Maria Himmelfahrt", 2026, 8, 15, Kind::Observance),
    ("CH-AI", "Allerheiligen", 1982, 11, 1, Kind::Observance),
    ("CH-AI", "Allerheiligen", 2026, 11, 1, Kind::Observance),
    ("CH-AI", "Maria Empfängnis", 1982, 12, 8, Kind::Observance),
    ("CH-AI", "Maria Empfängnis", 2026, 12, 8, Kind::Observance),
    ("CH-AR", "Karfreitag", 1966, 4, 8, Kind::Public),
    ("CH-AR", "Karfreitag", 2026, 4, 3, Kind::Public),
    ("CH-AR", "Ostermontag", 1966, 4, 11, Kind::Public),
    ("CH-AR", "Ostermontag", 2026, 4, 6, Kind::Public),
    ("CH-AR", "Pfingstmontag", 1966, 5, 30, Kind::Public),
    ("CH-AR", "Pfingstmontag", 2026, 5, 25, Kind::Public),
    ("CH-BE", "der 2. Januar", 1998, 1, 2, Kind::Public),
    ("CH-BE", "der 2. Januar", 2026, 1, 2, Kind::Public),
    ("CH-BE", "Karfreitag", 1998, 4, 10, Kind::Public),
    ("CH-BE", "Karfreitag", 2026, 4, 3, Kind::Public),
    ("CH-BE", "Ostermontag", 1998, 4, 13, Kind::Public),
    ("CH-BE", "Ostermontag", 2026, 4, 6, Kind::Public),
    ("CH-BE", "Pfingstmontag", 1997, 5, 19, Kind::Public),
    ("CH-BE", "Pfingstmontag", 2026, 5, 25, Kind::Public),
    ("CH-BE", "der 26. Dezember", 1997, 12, 26, Kind::Public),
    ("CH-BE", "der 26. Dezember", 2026, 12, 26, Kind::Public),
    ("CH-BL", "Karfreitag", 2011, 4, 22, Kind::Public),
    ("CH-BL", "Karfreitag", 2026, 4, 3, Kind::Public),
    ("CH-BL", "Ostermontag", 2011, 4, 25, Kind::Public),
    ("CH-BL", "Ostermontag", 2026, 4, 6, Kind::Public),
    ("CH-BL", "1. Mai", 2011, 5, 1, Kind::Public),
    ("CH-BL", "1. Mai", 2026, 5, 1, Kind::Public),
    ("CH-BL", "Pfingstmontag", 2011, 6, 13, Kind::Public),
    ("CH-BL", "Pfingstmontag", 2026, 5, 25, Kind::Public),
    ("CH-BL", "Stephanstag", 2011, 12, 26, Kind::Public),
    ("CH-BL", "Stephanstag", 2026, 12, 26, Kind::Public),
    ("CH-BS", "Karfreitag", 1994, 4, 1, Kind::Public),
    ("CH-BS", "Karfreitag", 2026, 4, 3, Kind::Public),
    ("CH-BS", "Ostermontag", 1994, 4, 4, Kind::Public),
    ("CH-BS", "Ostermontag", 2026, 4, 6, Kind::Public),
    ("CH-BS", "1. Mai", 1994, 5, 1, Kind::Public),
    ("CH-BS", "1. Mai", 2026, 5, 1, Kind::Public),
    ("CH-BS", "Pfingstmontag", 1994, 5, 23, Kind::Public),
    ("CH-BS", "Pfingstmontag", 2026, 5, 25, Kind::Public),
    ("CH-BS", "Stephanstag", 1994, 12, 26, Kind::Public),
    ("CH-BS", "Stephanstag", 2026, 12, 26, Kind::Public),
    ("CH-FR", "Vendredi-Saint", 2011, 4, 22, Kind::Public),
    ("CH-FR", "Vendredi-Saint", 2026, 4, 3, Kind::Public),
    ("CH-GE", "Vendredi saint", 1991, 3, 29, Kind::Public),
    ("CH-GE", "Vendredi saint", 2026, 4, 3, Kind::Public),
    ("CH-GE", "Lundi de Pâques", 1991, 4, 1, Kind::Public),
    ("CH-GE", "Lundi de Pâques", 2026, 4, 6, Kind::Public),
    ("CH-GE", "Lundi de Pentecôte", 1991, 5, 20, Kind::Public),
    ("CH-GE", "Lundi de Pentecôte", 2026, 5, 25, Kind::Public),
    ("CH-GE", "Jeûne genevois", 1991, 9, 5, Kind::Public),
    ("CH-GE", "Jeûne genevois", 2026, 9, 10, Kind::Public),
    (
        "CH-GE",
        "31 Décembre, anniversaire de la restauration de la République",
        1991,
        12,
        31,
        Kind::Public,
    ),
    (
        "CH-GE",
        "31 Décembre, anniversaire de la restauration de la République",
        2026,
        12,
        31,
        Kind::Public,
    ),
    ("CH-GL", "Fahrtsfest", 2013, 4, 4, Kind::Public),
    ("CH-GL", "Fahrtsfest", 2026, 4, 9, Kind::Public),
    ("CH-GL", "Karfreitag", 2013, 3, 29, Kind::Public),
    ("CH-GL", "Karfreitag", 2026, 4, 3, Kind::Public),
    ("CH-GL", "Ostermontag", 2013, 4, 1, Kind::Public),
    ("CH-GL", "Ostermontag", 2026, 4, 6, Kind::Public),
    ("CH-GL", "Pfingstmontag", 2012, 5, 28, Kind::Public),
    ("CH-GL", "Pfingstmontag", 2026, 5, 25, Kind::Public),
    ("CH-GL", "Allerheiligen", 2012, 11, 1, Kind::Public),
    ("CH-GL", "Allerheiligen", 2026, 11, 1, Kind::Public),
    ("CH-GL", "Stephanstag", 2012, 12, 26, Kind::Public),
    ("CH-GL", "Stephanstag", 2026, 12, 26, Kind::Public),
    ("CH-GR", "Karfreitag", 2006, 4, 14, Kind::Public),
    ("CH-GR", "Karfreitag", 2026, 4, 3, Kind::Public),
    ("CH-GR", "Ostermontag", 2006, 4, 17, Kind::Public),
    ("CH-GR", "Ostermontag", 2026, 4, 6, Kind::Public),
    ("CH-GR", "Pfingstmontag", 2006, 6, 5, Kind::Public),
    ("CH-GR", "Pfingstmontag", 2026, 5, 25, Kind::Public),
    ("CH-GR", "Stefanstag", 2006, 12, 26, Kind::Public),
    ("CH-GR", "Stefanstag", 2026, 12, 26, Kind::Public),
    ("CH-JU", "Vendredi-Saint", 2026, 4, 3, Kind::Public),
    ("CH-JU", "Lundi de Pâques", 2026, 4, 6, Kind::Public),
    ("CH-JU", "1er mai", 2026, 5, 1, Kind::Public),
    ("CH-JU", "Lundi de Pentecôte", 2026, 5, 25, Kind::Public),
    ("CH-JU", "Fête-Dieu", 2026, 6, 4, Kind::Public),
    ("CH-JU", "2 janvier", 2026, 1, 2, Kind::Observance),
    ("CH-JU", "Assomption", 2026, 8, 15, Kind::Observance),
    ("CH-JU", "Toussaint", 2026, 11, 1, Kind::Observance),
    // RSJU 555.1, in force 1 January 2023: art. 3 lit. b "le 23 juin", a
    // jour férié officiel that art. 4 does not equate with Sunday.
    ("CH-JU", "le 23 juin", 2026, 6, 23, Kind::Observance),
    ("CH-JU", "le 23 juin", 2023, 6, 23, Kind::Observance),
    ("CH-JU", "Vendredi-Saint", 2023, 4, 7, Kind::Public),
    ("CH-LU", "Karfreitag", 1998, 4, 10, Kind::Public),
    ("CH-LU", "Karfreitag", 2026, 4, 3, Kind::Public),
    ("CH-LU", "Fronleichnam", 1998, 6, 11, Kind::Public),
    ("CH-LU", "Fronleichnam", 2026, 6, 4, Kind::Public),
    ("CH-LU", "Mariä Himmelfahrt", 1997, 8, 15, Kind::Public),
    ("CH-LU", "Mariä Himmelfahrt", 2026, 8, 15, Kind::Public),
    ("CH-LU", "Allerheiligen", 1997, 11, 1, Kind::Public),
    ("CH-LU", "Allerheiligen", 2026, 11, 1, Kind::Public),
    ("CH-LU", "Stefanstag", 1997, 12, 26, Kind::Public),
    ("CH-LU", "Stefanstag", 2026, 12, 26, Kind::Public),
    ("CH-LU", "Mariä Empfängnis", 1997, 12, 8, Kind::Observance),
    ("CH-LU", "Mariä Empfängnis", 2026, 12, 8, Kind::Observance),
    ("CH-NE", "le 1er mars", 2010, 3, 1, Kind::Public),
    ("CH-NE", "le 1er mars", 2026, 3, 1, Kind::Public),
    ("CH-NE", "le 1er mai", 2010, 5, 1, Kind::Public),
    ("CH-NE", "le 1er mai", 2026, 5, 1, Kind::Public),
    ("CH-NE", "Vendredi Saint", 2010, 4, 2, Kind::Public),
    ("CH-NE", "Vendredi Saint", 2026, 4, 3, Kind::Public),
    ("CH-NW", "Josefstag", 2006, 3, 19, Kind::Observance),
    ("CH-NW", "Josefstag", 2026, 3, 19, Kind::Observance),
    ("CH-NW", "Karfreitag", 2006, 4, 14, Kind::Public),
    ("CH-NW", "Karfreitag", 2026, 4, 3, Kind::Public),
    ("CH-NW", "Fronleichnam", 2006, 6, 15, Kind::Public),
    ("CH-NW", "Fronleichnam", 2026, 6, 4, Kind::Public),
    ("CH-NW", "Maria Himmelfahrt", 2006, 8, 15, Kind::Public),
    ("CH-NW", "Maria Himmelfahrt", 2026, 8, 15, Kind::Public),
    ("CH-NW", "Allerheiligen", 2005, 11, 1, Kind::Public),
    ("CH-NW", "Allerheiligen", 2026, 11, 1, Kind::Public),
    ("CH-NW", "Maria Empfängnis", 2005, 12, 8, Kind::Public),
    ("CH-NW", "Maria Empfängnis", 2026, 12, 8, Kind::Public),
    ("CH-OW", "Karfreitag", 2008, 3, 21, Kind::Public),
    ("CH-OW", "Karfreitag", 2026, 4, 3, Kind::Public),
    ("CH-OW", "Fronleichnam", 2008, 5, 22, Kind::Public),
    ("CH-OW", "Fronleichnam", 2026, 6, 4, Kind::Public),
    ("CH-OW", "Mariä Himmelfahrt", 2007, 8, 15, Kind::Public),
    ("CH-OW", "Mariä Himmelfahrt", 2026, 8, 15, Kind::Public),
    ("CH-OW", "Allerheiligen", 2007, 11, 1, Kind::Public),
    ("CH-OW", "Allerheiligen", 2026, 11, 1, Kind::Public),
    ("CH-OW", "Mariä Empfängnis", 2007, 12, 8, Kind::Public),
    ("CH-OW", "Mariä Empfängnis", 2026, 12, 8, Kind::Public),
    ("CH-OW", "Bruderklausenfest", 2007, 9, 25, Kind::Observance),
    ("CH-OW", "Bruderklausenfest", 2026, 9, 25, Kind::Observance),
    ("CH-SG", "Karfreitag", 2005, 3, 25, Kind::Public),
    ("CH-SG", "Karfreitag", 2026, 4, 3, Kind::Public),
    ("CH-SG", "Ostermontag", 2005, 3, 28, Kind::Public),
    ("CH-SG", "Ostermontag", 2026, 4, 6, Kind::Public),
    ("CH-SG", "Pfingstmontag", 2005, 5, 16, Kind::Public),
    ("CH-SG", "Pfingstmontag", 2026, 5, 25, Kind::Public),
    ("CH-SG", "Allerheiligen", 2004, 11, 1, Kind::Public),
    ("CH-SG", "Allerheiligen", 2026, 11, 1, Kind::Public),
    ("CH-SG", "Stefanstag", 2004, 12, 26, Kind::Public),
    ("CH-SG", "Stefanstag", 2026, 12, 26, Kind::Public),
    ("CH-SH", "Karfreitag", 2011, 4, 22, Kind::Public),
    ("CH-SH", "Karfreitag", 2026, 4, 3, Kind::Public),
    ("CH-SH", "Ostermontag", 2011, 4, 25, Kind::Public),
    ("CH-SH", "Ostermontag", 2026, 4, 6, Kind::Public),
    ("CH-SH", "1. Mai", 2011, 5, 1, Kind::Public),
    ("CH-SH", "1. Mai", 2026, 5, 1, Kind::Public),
    ("CH-SH", "Pfingstmontag", 2011, 6, 13, Kind::Public),
    ("CH-SH", "Pfingstmontag", 2026, 5, 25, Kind::Public),
    ("CH-SH", "Stephanstag", 2011, 12, 26, Kind::Public),
    ("CH-SH", "Stephanstag", 2026, 12, 26, Kind::Public),
    ("CH-SO", "Karfreitag", 2016, 3, 25, Kind::Public),
    ("CH-SO", "Karfreitag", 2026, 4, 3, Kind::Public),
    ("CH-SZ", "Josefstag", 2026, 3, 19, Kind::Public),
    ("CH-SZ", "Karfreitag", 2026, 4, 3, Kind::Public),
    ("CH-SZ", "Fronleichnam", 2026, 6, 4, Kind::Public),
    ("CH-SZ", "Mariä Himmelfahrt", 2026, 8, 15, Kind::Public),
    ("CH-SZ", "Allerheiligen", 2026, 11, 1, Kind::Public),
    ("CH-SZ", "Heilige Drei Könige", 2026, 1, 6, Kind::Observance),
    ("CH-SZ", "Ostermontag", 2026, 4, 6, Kind::Observance),
    ("CH-SZ", "Pfingstmontag", 2026, 5, 25, Kind::Observance),
    ("CH-SZ", "Mariä Empfängnis", 2026, 12, 8, Kind::Observance),
    ("CH-SZ", "Stephanstag", 2026, 12, 26, Kind::Observance),
    // SRSZ 545.110, § 2, its lists unchanged since 1 January 2002.
    ("CH-SZ", "Josefstag", 2002, 3, 19, Kind::Public),
    ("CH-SZ", "Karfreitag", 2002, 3, 29, Kind::Public),
    ("CH-SZ", "Stephanstag", 2002, 12, 26, Kind::Observance),
    ("CH-TG", "2. Januar", 2003, 1, 2, Kind::Public),
    ("CH-TG", "2. Januar", 2026, 1, 2, Kind::Public),
    ("CH-TG", "Karfreitag", 2003, 4, 18, Kind::Public),
    ("CH-TG", "Karfreitag", 2026, 4, 3, Kind::Public),
    ("CH-TG", "Ostermontag", 2003, 4, 21, Kind::Public),
    ("CH-TG", "Ostermontag", 2026, 4, 6, Kind::Public),
    ("CH-TG", "1. Mai", 2003, 5, 1, Kind::Public),
    ("CH-TG", "1. Mai", 2026, 5, 1, Kind::Public),
    ("CH-TG", "Pfingstmontag", 2003, 6, 9, Kind::Public),
    ("CH-TG", "Pfingstmontag", 2026, 5, 25, Kind::Public),
    ("CH-TG", "26. Dezember", 2003, 12, 26, Kind::Public),
    ("CH-TG", "26. Dezember", 2026, 12, 26, Kind::Public),
    ("CH-TI", "Epifania", 2012, 1, 6, Kind::Public),
    ("CH-TI", "Epifania", 2026, 1, 6, Kind::Public),
    ("CH-TI", "Lunedì di Pasqua", 2012, 4, 9, Kind::Public),
    ("CH-TI", "Lunedì di Pasqua", 2026, 4, 6, Kind::Public),
    ("CH-TI", "Assunzione", 2011, 8, 15, Kind::Public),
    ("CH-TI", "Assunzione", 2026, 8, 15, Kind::Public),
    ("CH-TI", "Ognissanti", 2011, 11, 1, Kind::Public),
    ("CH-TI", "Ognissanti", 2026, 11, 1, Kind::Public),
    ("CH-TI", "Santo Stefano", 2011, 12, 26, Kind::Public),
    ("CH-TI", "Santo Stefano", 2026, 12, 26, Kind::Public),
    ("CH-TI", "San Giuseppe", 2010, 3, 19, Kind::Observance),
    ("CH-TI", "San Giuseppe", 2026, 3, 19, Kind::Observance),
    ("CH-TI", "1° Maggio", 2010, 5, 1, Kind::Observance),
    ("CH-TI", "1° Maggio", 2026, 5, 1, Kind::Observance),
    (
        "CH-TI",
        "Lunedì di Pentecoste",
        2010,
        5,
        24,
        Kind::Observance,
    ),
    (
        "CH-TI",
        "Lunedì di Pentecoste",
        2026,
        5,
        25,
        Kind::Observance,
    ),
    ("CH-TI", "Corpus Domini", 2010, 6, 3, Kind::Observance),
    ("CH-TI", "Corpus Domini", 2026, 6, 4, Kind::Observance),
    ("CH-TI", "SS. Pietro e Paolo", 2010, 6, 29, Kind::Observance),
    ("CH-TI", "SS. Pietro e Paolo", 2026, 6, 29, Kind::Observance),
    ("CH-TI", "Immacolata", 2010, 12, 8, Kind::Observance),
    ("CH-TI", "Immacolata", 2026, 12, 8, Kind::Observance),
    ("CH-UR", "Karfreitag", 2002, 3, 29, Kind::Public),
    ("CH-UR", "Karfreitag", 2026, 4, 3, Kind::Public),
    ("CH-UR", "Fronleichnam", 2002, 5, 30, Kind::Public),
    ("CH-UR", "Fronleichnam", 2026, 6, 4, Kind::Public),
    ("CH-UR", "Mariä Himmelfahrt", 2002, 8, 15, Kind::Public),
    ("CH-UR", "Mariä Himmelfahrt", 2026, 8, 15, Kind::Public),
    ("CH-UR", "Allerheiligen", 2002, 11, 1, Kind::Public),
    ("CH-UR", "Allerheiligen", 2026, 11, 1, Kind::Public),
    ("CH-UR", "Mariä Empfängnis", 2002, 12, 8, Kind::Public),
    ("CH-UR", "Mariä Empfängnis", 2026, 12, 8, Kind::Public),
    ("CH-UR", "Dreikönigen", 2003, 1, 6, Kind::Observance),
    ("CH-UR", "Dreikönigen", 2026, 1, 6, Kind::Observance),
    ("CH-UR", "Sankt-Josefs-Tag", 2003, 3, 19, Kind::Observance),
    ("CH-UR", "Sankt-Josefs-Tag", 2026, 3, 19, Kind::Observance),
    ("CH-UR", "Ostermontag", 2003, 4, 21, Kind::Observance),
    ("CH-UR", "Ostermontag", 2026, 4, 6, Kind::Observance),
    ("CH-UR", "Pfingstmontag", 2003, 6, 9, Kind::Observance),
    ("CH-UR", "Pfingstmontag", 2026, 5, 25, Kind::Observance),
    ("CH-UR", "Sankt-Stefans-Tag", 2003, 12, 26, Kind::Observance),
    ("CH-UR", "Sankt-Stefans-Tag", 2026, 12, 26, Kind::Observance),
    ("CH-VD", "le Vendredi-Saint", 2006, 4, 14, Kind::Public),
    ("CH-VD", "le Vendredi-Saint", 2026, 4, 3, Kind::Public),
    ("CH-VD", "le lundi de Pâques", 2006, 4, 17, Kind::Public),
    ("CH-VD", "le lundi de Pâques", 2026, 4, 6, Kind::Public),
    (
        "CH-VD",
        "le lundi du Jeûne fédéral",
        2006,
        9,
        18,
        Kind::Public,
    ),
    (
        "CH-VD",
        "le lundi du Jeûne fédéral",
        2026,
        9,
        21,
        Kind::Public,
    ),
    ("CH-VD", "le 2 janvier", 2008, 1, 2, Kind::Public),
    ("CH-VD", "le 2 janvier", 2026, 1, 2, Kind::Public),
    ("CH-VD", "le lundi de Pentecôte", 2008, 5, 12, Kind::Public),
    ("CH-VD", "le lundi de Pentecôte", 2026, 5, 25, Kind::Public),
    ("CH-VS", "Saint-Joseph", 2017, 3, 19, Kind::Public),
    ("CH-VS", "Saint-Joseph", 2026, 3, 19, Kind::Public),
    ("CH-VS", "Fête-Dieu", 2017, 6, 15, Kind::Public),
    ("CH-VS", "Fête-Dieu", 2026, 6, 4, Kind::Public),
    ("CH-VS", "Assomption", 2017, 8, 15, Kind::Public),
    ("CH-VS", "Assomption", 2026, 8, 15, Kind::Public),
    ("CH-VS", "Toussaint", 2016, 11, 1, Kind::Public),
    ("CH-VS", "Toussaint", 2026, 11, 1, Kind::Public),
    ("CH-VS", "Immaculée Conception", 2016, 12, 8, Kind::Public),
    ("CH-VS", "Immaculée Conception", 2026, 12, 8, Kind::Public),
    ("CH-ZG", "Karfreitag", 2004, 4, 9, Kind::Public),
    ("CH-ZG", "Karfreitag", 2026, 4, 3, Kind::Public),
    ("CH-ZG", "Fronleichnam", 2004, 6, 10, Kind::Public),
    ("CH-ZG", "Fronleichnam", 2026, 6, 4, Kind::Public),
    ("CH-ZG", "Maria Himmelfahrt", 2004, 8, 15, Kind::Public),
    ("CH-ZG", "Maria Himmelfahrt", 2026, 8, 15, Kind::Public),
    ("CH-ZG", "Allerheiligen", 2004, 11, 1, Kind::Public),
    ("CH-ZG", "Allerheiligen", 2026, 11, 1, Kind::Public),
    ("CH-ZG", "Maria Empfängnis", 2004, 12, 8, Kind::Public),
    ("CH-ZG", "Maria Empfängnis", 2026, 12, 8, Kind::Public),
    ("CH-ZH", "Karfreitag", 2026, 4, 3, Kind::Public),
    ("CH-ZH", "Ostermontag", 2026, 4, 6, Kind::Public),
    ("CH-ZH", "1. Mai", 2026, 5, 1, Kind::Public),
    ("CH-ZH", "Pfingstmontag", 2026, 5, 25, Kind::Public),
    ("CH-ZH", "Stephanstag", 2026, 12, 26, Kind::Public),
    // LS 822.4, § 1, in force from 1 December 2000.
    ("CH-ZH", "Stephanstag", 2000, 12, 26, Kind::Public),
    ("CH-ZH", "Karfreitag", 2001, 4, 13, Kind::Public),
    ("CH-ZH", "1. Mai", 2001, 5, 1, Kind::Public),
];

/// `(region, local name, first year carried, last year of the gap before
/// it)`: the years between the two, where there are any, are absent.
const FIRST_YEARS: &[(&str, &str, i64, i64)] = &[
    ("CH-AG", "Karfreitag", 2013, 2012),
    ("CH-AI", "Karfreitag", 2011, 2010),
    ("CH-AI", "Ostermontag", 2011, 2010),
    ("CH-AI", "Pfingstmontag", 2011, 2010),
    ("CH-AI", "Fronleichnam", 2011, 2010),
    ("CH-AI", "Stephanstag", 2011, 2010),
    ("CH-AI", "Maria Himmelfahrt", 1982, 1981),
    ("CH-AI", "Allerheiligen", 1982, 1981),
    ("CH-AI", "Maria Empfängnis", 1982, 1981),
    ("CH-AR", "Karfreitag", 1966, 1965),
    ("CH-AR", "Ostermontag", 1966, 1965),
    ("CH-AR", "Pfingstmontag", 1966, 1965),
    ("CH-AR", "zweiter Weihnachtstag", 1967, 1966),
    ("CH-BE", "der 2. Januar", 1998, 1997),
    ("CH-BE", "Karfreitag", 1998, 1997),
    ("CH-BE", "Ostermontag", 1998, 1997),
    ("CH-BE", "Pfingstmontag", 1997, 1996),
    ("CH-BE", "der 26. Dezember", 1997, 1996),
    ("CH-BL", "Karfreitag", 2011, 2010),
    ("CH-BL", "Ostermontag", 2011, 2010),
    ("CH-BL", "1. Mai", 2011, 2010),
    ("CH-BL", "Pfingstmontag", 2011, 2010),
    ("CH-BL", "Stephanstag", 2011, 2010),
    ("CH-BS", "Karfreitag", 1994, 1993),
    ("CH-BS", "Ostermontag", 1994, 1993),
    ("CH-BS", "1. Mai", 1994, 1993),
    ("CH-BS", "Pfingstmontag", 1994, 1993),
    ("CH-BS", "Stephanstag", 1994, 1993),
    ("CH-FR", "Vendredi-Saint", 2011, 2010),
    ("CH-GE", "Vendredi saint", 1991, 1990),
    ("CH-GE", "Lundi de Pâques", 1991, 1990),
    ("CH-GE", "Lundi de Pentecôte", 1991, 1990),
    ("CH-GE", "Jeûne genevois", 1991, 1990),
    (
        "CH-GE",
        "31 Décembre, anniversaire de la restauration de la République",
        1991,
        1990,
    ),
    ("CH-GL", "Fahrtsfest", 2013, 2012),
    ("CH-GL", "Karfreitag", 2013, 2012),
    ("CH-GL", "Ostermontag", 2013, 2012),
    ("CH-GL", "Pfingstmontag", 2012, 2011),
    ("CH-GL", "Allerheiligen", 2012, 2011),
    ("CH-GL", "Stephanstag", 2012, 2011),
    ("CH-GR", "Karfreitag", 2006, 2005),
    ("CH-GR", "Ostermontag", 2006, 2005),
    ("CH-GR", "Pfingstmontag", 2006, 2005),
    ("CH-GR", "Stefanstag", 2006, 2005),
    ("CH-JU", "Vendredi-Saint", 2023, 2022),
    ("CH-JU", "Lundi de Pâques", 2023, 2022),
    ("CH-JU", "1er mai", 2023, 2022),
    ("CH-JU", "Lundi de Pentecôte", 2023, 2022),
    ("CH-JU", "Fête-Dieu", 2023, 2022),
    ("CH-JU", "2 janvier", 2023, 2022),
    ("CH-JU", "Assomption", 2023, 2022),
    ("CH-JU", "Toussaint", 2023, 2022),
    ("CH-JU", "le 23 juin", 2023, 2022),
    ("CH-LU", "Karfreitag", 1998, 1997),
    ("CH-LU", "Fronleichnam", 1998, 1997),
    ("CH-LU", "Mariä Himmelfahrt", 1997, 1996),
    ("CH-LU", "Allerheiligen", 1997, 1996),
    ("CH-LU", "Stefanstag", 1997, 1996),
    ("CH-LU", "Mariä Empfängnis", 1997, 1996),
    ("CH-NE", "le 2 janvier", 2010, 2009),
    ("CH-NE", "le 1er mars", 2010, 2009),
    ("CH-NE", "le 1er mai", 2010, 2009),
    ("CH-NE", "Vendredi Saint", 2010, 2009),
    ("CH-NE", "le 26 décembre", 2010, 2009),
    ("CH-NW", "Josefstag", 2006, 2005),
    ("CH-NW", "Karfreitag", 2006, 2005),
    ("CH-NW", "Fronleichnam", 2006, 2005),
    ("CH-NW", "Maria Himmelfahrt", 2006, 2005),
    ("CH-NW", "Allerheiligen", 2005, 2004),
    ("CH-NW", "Maria Empfängnis", 2005, 2004),
    ("CH-OW", "Karfreitag", 2008, 2007),
    ("CH-OW", "Fronleichnam", 2008, 2007),
    ("CH-OW", "Mariä Himmelfahrt", 2007, 2006),
    ("CH-OW", "Allerheiligen", 2007, 2006),
    ("CH-OW", "Mariä Empfängnis", 2007, 2006),
    ("CH-OW", "Bruderklausenfest", 2007, 2006),
    ("CH-SG", "Karfreitag", 2005, 2004),
    ("CH-SG", "Ostermontag", 2005, 2004),
    ("CH-SG", "Pfingstmontag", 2005, 2004),
    ("CH-SG", "Allerheiligen", 2004, 2003),
    ("CH-SG", "Stefanstag", 2004, 2003),
    ("CH-SH", "Karfreitag", 2011, 2010),
    ("CH-SH", "Ostermontag", 2011, 2010),
    ("CH-SH", "1. Mai", 2011, 2010),
    ("CH-SH", "Pfingstmontag", 2011, 2010),
    ("CH-SH", "Stephanstag", 2011, 2010),
    ("CH-SO", "Karfreitag", 2016, 2015),
    ("CH-SZ", "Josefstag", 2002, 2001),
    ("CH-SZ", "Karfreitag", 2002, 2001),
    ("CH-SZ", "Fronleichnam", 2002, 2001),
    ("CH-SZ", "Mariä Himmelfahrt", 2002, 2001),
    ("CH-SZ", "Allerheiligen", 2002, 2001),
    ("CH-SZ", "Heilige Drei Könige", 2002, 2001),
    ("CH-SZ", "Ostermontag", 2002, 2001),
    ("CH-SZ", "Pfingstmontag", 2002, 2001),
    ("CH-SZ", "Mariä Empfängnis", 2002, 2001),
    ("CH-SZ", "Stephanstag", 2002, 2001),
    ("CH-TG", "2. Januar", 2003, 2002),
    ("CH-TG", "Karfreitag", 2003, 2002),
    ("CH-TG", "Ostermontag", 2003, 2002),
    ("CH-TG", "1. Mai", 2003, 2002),
    ("CH-TG", "Pfingstmontag", 2003, 2002),
    ("CH-TG", "26. Dezember", 2003, 2002),
    ("CH-TI", "Epifania", 2012, 2011),
    ("CH-TI", "Lunedì di Pasqua", 2012, 2011),
    ("CH-TI", "Assunzione", 2011, 2010),
    ("CH-TI", "Ognissanti", 2011, 2010),
    ("CH-TI", "Santo Stefano", 2011, 2010),
    ("CH-TI", "San Giuseppe", 2010, 2009),
    ("CH-TI", "1° Maggio", 2010, 2009),
    ("CH-TI", "Lunedì di Pentecoste", 2010, 2009),
    ("CH-TI", "Corpus Domini", 2010, 2009),
    ("CH-TI", "SS. Pietro e Paolo", 2010, 2009),
    ("CH-TI", "Immacolata", 2010, 2009),
    ("CH-UR", "Karfreitag", 2002, 2001),
    ("CH-UR", "Fronleichnam", 2002, 2001),
    ("CH-UR", "Mariä Himmelfahrt", 2002, 2001),
    ("CH-UR", "Allerheiligen", 2002, 2001),
    ("CH-UR", "Mariä Empfängnis", 2002, 2001),
    ("CH-UR", "Dreikönigen", 2003, 2002),
    ("CH-UR", "Sankt-Josefs-Tag", 2003, 2002),
    ("CH-UR", "Ostermontag", 2003, 2002),
    ("CH-UR", "Pfingstmontag", 2003, 2002),
    ("CH-UR", "Sankt-Stefans-Tag", 2003, 2002),
    ("CH-VD", "le Vendredi-Saint", 2006, 2005),
    ("CH-VD", "le lundi de Pâques", 2006, 2005),
    ("CH-VD", "le lundi du Jeûne fédéral", 2006, 2005),
    ("CH-VD", "le 2 janvier", 2008, 2005),
    ("CH-VD", "le lundi de Pentecôte", 2008, 2005),
    ("CH-VS", "Saint-Joseph", 2017, 2016),
    ("CH-VS", "Fête-Dieu", 2017, 2016),
    ("CH-VS", "Assomption", 2017, 2016),
    ("CH-VS", "Toussaint", 2016, 2015),
    ("CH-VS", "Immaculée Conception", 2016, 2015),
    ("CH-ZG", "Karfreitag", 2004, 2003),
    ("CH-ZG", "Fronleichnam", 2004, 2003),
    ("CH-ZG", "Maria Himmelfahrt", 2004, 2003),
    ("CH-ZG", "Allerheiligen", 2004, 2003),
    ("CH-ZG", "Maria Empfängnis", 2004, 2003),
    ("CH-ZH", "Karfreitag", 2001, 2000),
    ("CH-ZH", "Ostermontag", 2001, 2000),
    ("CH-ZH", "1. Mai", 2001, 2000),
    ("CH-ZH", "Pfingstmontag", 2001, 2000),
    ("CH-ZH", "Stephanstag", 2000, 1999),
];

#[test]
fn every_cantonal_day_falls_where_its_law_puts_it() {
    for &(region, local_name, year, month, day, kind) in ANCHORS {
        let found: Vec<Holiday> = own_in_year(region, year)
            .into_iter()
            .filter(|holiday| holiday.local_name == local_name)
            .collect();
        assert_eq!(found.len(), 1, "{region} {local_name} {year}: {found:?}");
        let entry = found[0];
        assert_eq!(
            entry.date,
            ymd(year, month, day),
            "{region} {local_name} {year}"
        );
        assert_eq!(entry.kind, kind, "{region} {local_name}");
        assert_eq!(entry.regions, [region], "{region} {local_name}");
        assert!(!entry.source.is_empty(), "{region} {local_name}");
    }
}

/// Whether a canton's calendar for `year` reports a gap for the day.
fn has_gap(region: &str, local_name: &str, year: i64) -> bool {
    HolidayCalendar::for_year(&SWITZERLAND, Some(region), year)
        .gaps()
        .iter()
        .any(|gap| gap.local_name == local_name && gap.year == year)
}

#[test]
fn before_its_law_a_cantonal_day_is_a_gap() {
    for &(region, local_name, first, gap_until) in FIRST_YEARS {
        for year in [first - 1, gap_until] {
            assert!(
                own_in_year(region, year)
                    .iter()
                    .all(|holiday| holiday.local_name != local_name),
                "{region} {local_name} {year}"
            );
        }
        assert!(
            has_gap(region, local_name, gap_until),
            "{region} {local_name} {gap_until}"
        );
        assert_eq!(
            has_gap(region, local_name, first - 1),
            gap_until == first - 1,
            "{region} {local_name} {}",
            first - 1
        );
    }
}

#[test]
fn vaud_s_additions_of_2007_are_absent_in_the_text_before_them() {
    // Whit Monday: not in the law as in force from 1 January 2006, which
    // was read, so 2006 has no day and no gap; 2005, before that law, is a
    // gap; 2008 is its first year under the amendment of September 2007.
    assert!(!has_gap("CH-VD", "le lundi de Pentecôte", 2006));
    assert!(has_gap("CH-VD", "le lundi de Pentecôte", 2005));
    assert!(
        own_in_year("CH-VD", 2008)
            .iter()
            .any(|h| h.local_name == "le lundi de Pentecôte")
    );
}

#[test]
fn the_nationwide_days_are_the_four_every_canton_keeps() {
    let names: Vec<&str> = HolidayCalendar::for_year(&SWITZERLAND, None, 2026)
        .in_year(2026)
        .iter()
        .map(|holiday| holiday.local_name)
        .collect();
    assert_eq!(
        names,
        ["Neujahrstag", "Auffahrt", "Bundesfeier", "Weihnachtstag"]
    );
    // Good Friday, 3 April 2026: a day off in Bern, not in Ticino or the
    // Valais, whose laws do not keep it.
    let good_friday = ymd(2026, 4, 3);
    for (region, kept) in [("CH-BE", true), ("CH-TI", false), ("CH-VS", false)] {
        let calendar = HolidayCalendar::for_year(&SWITZERLAND, Some(region), 2026);
        assert_eq!(calendar.is_holiday(good_friday), kept, "{region}");
    }
}

#[test]
fn the_conditional_days_follow_their_conditions() {
    // Appenzell Ausserrhoden: no 26 December when Christmas is a Friday
    // (2026) or a Monday (2023); a Saturday 26 December in 2027.
    for (year, kept) in [(2023, false), (2026, false), (2027, true)] {
        let found = own_in_year("CH-AR", year)
            .into_iter()
            .any(|holiday| holiday.date == ymd(year, 12, 26));
        assert_eq!(found, kept, "CH-AR {year}");
    }
    // Appenzell Innerrhoden: those years are a gap.
    let gap = HolidayCalendar::for_year(&SWITZERLAND, Some("CH-AI"), 2026);
    assert!(gap.gaps().iter().any(|gap| gap.local_name == "Stephanstag"));
    // Neuchâtel: 2 January only after a Sunday 1 January (2023), and
    // 26 December only after a Sunday Christmas (2022).
    assert!(
        own_in_year("CH-NE", 2023)
            .iter()
            .any(|h| h.date == ymd(2023, 1, 2))
    );
    assert!(
        own_in_year("CH-NE", 2024)
            .iter()
            .all(|h| h.date != ymd(2024, 1, 2))
    );
    assert!(
        own_in_year("CH-NE", 2022)
            .iter()
            .any(|h| h.date == ymd(2022, 12, 26))
    );
    assert!(
        own_in_year("CH-NE", 2026)
            .iter()
            .all(|h| h.date != ymd(2026, 12, 26))
    );
    // Glarus: the first Thursday of April 2026 is the 2nd, in Holy Week,
    // so the Fahrtsfest is on the 9th.
    assert!(
        own_in_year("CH-GL", 2026)
            .iter()
            .any(|h| h.local_name == "Fahrtsfest" && h.date == ymd(2026, 4, 9))
    );
}

#[test]
fn a_rest_day_the_law_does_not_make_equal_to_sunday_is_no_day_off() {
    // Ticino's San Giuseppe, Thursday 19 March 2026, and Lucerne's
    // 8 December.
    let ticino = HolidayCalendar::for_year(&SWITZERLAND, Some("CH-TI"), 2026);
    assert!(ticino.is_business_day(ymd(2026, 3, 19)));
    let lucerne = HolidayCalendar::for_year(&SWITZERLAND, Some("CH-LU"), 2026);
    assert!(lucerne.is_business_day(ymd(2026, 12, 8)));
}

#[test]
fn solothurn_keeps_1_may_from_noon_as_a_half_day() {
    let day = |year| {
        HolidayCalendar::for_year(&SWITZERLAND, Some("CH-SO"), year)
            .on(gregorian::to_fixed(year, 5, 1).unwrap_or(Rd(0)))
    };
    let may = day(2026);
    assert_eq!(may.len(), 1);
    assert_eq!(may[0].kind, Kind::HalfDay);
    assert_eq!(may[0].local_name, "1. Mai (ab 12 Uhr)");
    assert!(!may[0].is_day_off());
    // 1 May 2026, a Friday, is a business day in Solothurn, as a half day is.
    let calendar = HolidayCalendar::for_year(&SWITZERLAND, Some("CH-SO"), 2026);
    assert!(calendar.is_business_day(gregorian::to_fixed(2026, 5, 1).unwrap_or(Rd(0))));
    // Before the law of 2016, a gap; in Bern, no half day.
    assert!(day(2015).is_empty());
    assert!(
        HolidayCalendar::for_year(&SWITZERLAND, Some("CH-SO"), 2015)
            .gaps()
            .iter()
            .any(|gap| gap.local_name == "1. Mai (ab 12 Uhr)")
    );
    assert!(
        HolidayCalendar::for_year(&SWITZERLAND, Some("CH-BE"), 2026)
            .all()
            .iter()
            .all(|holiday| holiday.kind != Kind::HalfDay)
    );
}

#[test]
fn the_days_every_canton_keeps_rest_on_each_canton_s_law() {
    let names = |region: Option<&str>, year| {
        HolidayCalendar::for_year(&SWITZERLAND, region, year)
            .gaps()
            .iter()
            .map(|gap| gap.name)
            .collect::<Vec<_>>()
    };
    // Nationwide the three days are every canton's, and Jura's law, the
    // latest read, is in force from 2023: before it, gaps.
    for name in ["New Year's Day", "Ascension", "Christmas Day"] {
        assert!(names(None, 2022).contains(&name), "{name}");
        assert!(!names(None, 2023).contains(&name), "{name}");
        // Bern's law of 1997 answers from 1998; Geneva's from 1991.
        assert!(!names(Some("CH-BE"), 1998).contains(&name), "{name}");
        assert!(names(Some("CH-BE"), 1997).contains(&name), "{name}");
        assert!(!names(Some("CH-GE"), 1991).contains(&name), "{name}");
    }
    // 1 August is equal to Sunday by the Arbeitsgesetz's Art. 20a, in force
    // from 1 August 2000; the Verordnung of 1994 was not read.
    assert!(names(None, 1999).contains(&"Swiss National Day"));
    assert!(!names(None, 2000).contains(&"Swiss National Day"));
    let august = |year| {
        HolidayCalendar::for_year(&SWITZERLAND, None, year)
            .is_holiday(gregorian::to_fixed(year, 8, 1).unwrap_or(Rd(0)))
    };
    assert!(august(2000));
    assert!(!august(1999));
    // A New Year's Day in 1800 is no answer, but a gap.
    assert!(names(None, 1800).contains(&"New Year's Day"));
}
