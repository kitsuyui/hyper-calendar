//! How a calendar writes its dates whatever the language: the notations
//! a calendar's own sources give, as templates.
//!
//! Most calendars are written the way a language writes dates — *Bhadra
//! 16, 1948 Saka*, 令和8年9月21日 — and their templates belong to the
//! locale, in [`crate::names::LocaleData::templates`] and the entries that
//! serve a calendar. A few are written in a notation of their own that
//! does not change with the language: the Long Count's place values joined
//! by dots, `13.0.13.17.8`; the ISO 8601 week date, `2026-W39-7`. The
//! numbers in them are still written in the locale's numbering system, as
//! every number the renderer writes is.
//!
//! A notation is one level of the [`crate::names::TemplateChain`]: below
//! the entries that serve the calendar in a locale, which can still state
//! the language's own way of writing it, and above the locale's general
//! templates, which were written for dates with a year, a month and a day
//! and would drop the fields a notation is made of.
//!
//! Every notation names the source that writes dates so, in a comment
//! beside it. A calendar with none here and no locale's word for its dates
//! is written with its year, month and day, and its extra fields, which no
//! source puts in the date, stay out of the text; [`crate::fields`] labels
//! them for the lines that list them.

use hc_calendar::CalendarId;

use crate::names::DateTemplates;

/// A notation, and the calendars written in it.
#[derive(Debug, Clone, Copy)]
pub struct Notation {
    /// The calendars this notation writes, by registry identifier.
    pub calendars: &'static [CalendarId],
    /// The templates, with every field the notation leaves to the locale
    /// empty.
    pub templates: DateTemplates,
    /// Where the notation is stated, keyed as in `docs/references.bib`.
    pub source: &'static str,
}

/// The notation a calendar is written in, or [`DateTemplates::NONE`] for
/// a calendar with none of its own.
#[must_use]
pub fn templates_for(calendar: CalendarId) -> DateTemplates {
    NOTATIONS
        .iter()
        .find(|notation| notation.calendars.contains(&calendar))
        .map_or(DateTemplates::NONE, |notation| notation.templates)
}

/// A date template and nothing else.
const fn date(template: &'static str) -> DateTemplates {
    DateTemplates {
        date: template,
        ..DateTemplates::NONE
    }
}

/// Every notation.
pub static NOTATIONS: &[Notation] = &[
    // The week date, year, week and weekday: `2021-W53-5`.
    Notation {
        calendars: &[CalendarId("iso8601-week")],
        templates: date("{year:4}-W{extra:week:2}-{extra:day-of-week:1}"),
        source: "ISO 8601 as hc_calendars_solar::iso_week writes it, `2021-W53-5`; the ISO text was not read",
    },
    // The ordinal date, year and day of the year: `2026-263`.
    Notation {
        calendars: &[CalendarId("iso8601-ordinal")],
        templates: date("{year:4}-{extra:day-of-year:3}"),
        source: "ISO 8601 as hc_calendars_solar::ordinal writes it, `YYYY-DDD`; the ISO text was not read",
    },
    // Stata's `%tw` date, the year and its week: `1960w1`, `0100w1`.
    Notation {
        calendars: &[CalendarId("stata-week")],
        templates: date("{year:4}w{extra:week}"),
        source: "stata-help-datetime-functions",
    },
    // Palmen's long form, *Thursday Beta January 2019*.
    Notation {
        calendars: &[CalendarId("week-and-month")],
        templates: date("{extra:day-of-week} {extra:week-of-month} {month} {year}"),
        source: "palmen-week-and-month",
    },
    // Meyer's `year-quarter-triday-day SLT`, `95-3-15-2 SLT`, and the lunar
    // calendar's `cycle-year-month-triday-day LLT`, `0-098-07-09-2 LLT`.
    Notation {
        calendars: &[CalendarId("liberalia-triday-solar")],
        templates: date("{year:1}-{month:1}-{extra:triday:2}-{extra:day-of-triday:1} SLT"),
        source: "meyer-liberalia-triday",
    },
    Notation {
        calendars: &[CalendarId("liberalia-triday-lunar")],
        templates: date(
            "{extra:cycle}-{extra:year-of-cycle:3}-{month:2}-{extra:triday:2}-{extra:day-of-triday:1} LLT",
        ),
        source: "meyer-liberalia-triday",
    },
    // Meyer's cycle-year-month-day, `102-25-01-01 MP`.
    Notation {
        calendars: &[CalendarId("meyer-palmen")],
        templates: date("{extra:cycle}-{extra:year-of-cycle:2}-{month:2}-{day:2} {era}"),
        source: "meyer-mpslc",
    },
    // Palmen's cycle-yerm(month(night, `21-05(03(30`.
    Notation {
        calendars: &[CalendarId("yerm")],
        templates: date("{extra:cycle}-{extra:yerm-of-cycle:2}({month:2}({day:2}"),
        source: "palmen-yerm",
    },
    // The Long Count's five places, `9.12.11.5.18`.
    Notation {
        calendars: &[
            CalendarId("maya-longcount"),
            CalendarId("maya-longcount-gmt2"),
            CalendarId("maya-longcount-584286"),
        ],
        templates: date("{extra:baktun}.{extra:katun}.{extra:tun}.{extra:uinal}.{extra:kin}"),
        source: "wikipedia-long-count",
    },
    // The Tzolkʼin's number and day sign, `6 Ben`.
    Notation {
        calendars: &[
            CalendarId("maya-tzolkin"),
            CalendarId("maya-tzolkin-gmt2"),
            CalendarId("maya-tzolkin-584286"),
        ],
        templates: date("{extra:tzolkin_number} {extra:tzolkin_name}"),
        source: "reingold2018code, wikipedia-tzolkin",
    },
    // The Haabʼ's day and month, the day counted from its seating, `0 Pop`.
    Notation {
        calendars: &[
            CalendarId("maya-haab"),
            CalendarId("maya-haab-gmt2"),
            CalendarId("maya-haab-584286"),
        ],
        templates: date("{day:0-based} {month}"),
        source: "wikipedia-haab",
    },
    // The Calendar Round, the Tzolkʼin then the Haabʼ, `8 Ajaw 13 Pop`.
    Notation {
        calendars: &[
            CalendarId("maya-round"),
            CalendarId("maya-round-gmt2"),
            CalendarId("maya-round-584286"),
        ],
        templates: date("{extra:tzolkin_number} {extra:tzolkin_name} {day:0-based} {month}"),
        source: "wikipedia-pakal, martin2012",
    },
    // The tonalpohualli's number and sign, `1 Coatl`.
    Notation {
        calendars: &[CalendarId("aztec-tonalpohualli")],
        templates: date("{extra:tonalpohualli_number} {extra:tonalpohualli_sign}"),
        source: "azteccalendar",
    },
    // The day and month, and the year by its bearer: "3 gonaa in the year
    // 12 Wind", 12 ee; Caso's year 10 Tecpatl.
    Notation {
        calendars: &[CalendarId("zapotec-yza"), CalendarId("mixtec-year")],
        templates: date("{day} {month}, {extra:year_bearer_number} {extra:year_bearer}"),
        source: "tavarez2008, caso1967",
    },
    // The seven-day, five-day and thirty-week names, *Buda Kliwon Dungulan*.
    Notation {
        calendars: &[CalendarId("balinese-pawukon")],
        templates: date("{day} {extra:pancawara} {month}"),
        source: "idntimes-galungan-2018",
    },
    // The weekday and the pasaran, *Selasa Wage*.
    Notation {
        calendars: &[CalendarId("javanese-pasaran")],
        templates: date("{extra:dina} {extra:pasaran}"),
        source: "wikipedia-javanese-calendar",
    },
    // The six-day name, then the seven-day, *Fo-Dwo*.
    Notation {
        calendars: &[CalendarId("akan")],
        templates: date("{extra:nnanson}-{extra:nnawotwe}"),
        source: "wikipedia-akan-calendar",
    },
    // The day's stem and branch, 甲子, in the locale's reading.
    Notation {
        calendars: &[CalendarId("sexagenary")],
        templates: date("{extra:sexagenary}"),
        source: "nao-rekiwiki-kanshi",
    },
    // The day, the month, the year and the year's name in the windu,
    // "1 Sura 1959 Dal".
    Notation {
        calendars: &[
            CalendarId("javanese"),
            CalendarId("javanese-yogyakarta"),
            CalendarId("javanese-aboge"),
        ],
        templates: date("{day} {month} {year:1} {extra:taun}"),
        source: "tanaya1971, kompas-suro-1959",
    },
    // The half, the day of the half, ค่ำ and the month, แรม 1 ค่ำ
    // เดือนแปด, as wikipedia-th-thai-lunar writes them, then the Buddhist
    // year. The year is the library's own addition, as
    // `hc_calendars_regional::thai_lunar`'s display writes it: the page
    // writes no year after the month, and a full date elsewhere writes the
    // year by its animal, its decade and the Chula Sakarat, "ขึ้น 12 ค่ำ
    // เดือนอ้าย ปีเถาะ นพศก จ.ศ. 1289" (Wikipedia (th), the article on King
    // Bhumibol Adulyadej, retrieved 2026-09-28), which the calendar does
    // not carry.
    Notation {
        calendars: &[CalendarId("thai-lunar")],
        templates: date("{extra:waning} {extra:fortnight-day} ค่ำ {month} {year:1}"),
        source: "wikipedia-th-thai-lunar for the half, the day and the month; the year after them is the library's own, which no source read writes there",
    },
    // The month, the half, the day and ຄ່ຳ, then ປີ and the year, in the
    // order of Dupertuis's glossary: ເດືອນຫ້າ ຂຶ້ນ ໑໑ ຄ່ຳ ປີ ໑໓໔໓.
    Notation {
        calendars: &[CalendarId("lao")],
        templates: date("{month} {extra:waning} {extra:fortnight-day} ຄ່ຳ ປີ {year:1}"),
        source: "dupertuis1981",
    },
];
