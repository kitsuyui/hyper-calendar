//! What a calendar's extra fields are called, and which cycle names their
//! values.
//!
//! A date carries, beside its era, year, month and day, the fields its
//! calendar counts besides them ([`hc_calendar::fields::ExtraFields`]): the Long
//! Count's five places, the Tamil year's name among sixty, the Julian Day
//! Number a day count is kept against. Each has an identifier, `baktun`,
//! `samvatsara`, `julian-day-number`, which is a key and never reader-facing
//! text. This module is what a reader is shown instead: a label for each
//! field, and, where the field counts the positions of one of the
//! calendar's named cycles, the name of the position it holds.
//!
//! A field that the calendar's sources write in the date is written there
//! by a template's `{extra:FIELD}` placeholder
//! ([`crate::names::DateTemplates`]); the rest are metadata, which the
//! lines that list a day's extra fields label here.
//!
//! # What is carried
//!
//! An English label for every extra field a registered calendar sets,
//! in the words its system document or module uses for it. A label in
//! another language is carried only where a source in that language names
//! the field; nothing here is translated. A locale without one is given the
//! English label, as an era no locale names is given its English name.
//!
//! # Locales
//!
//! [`label`] walks [`Locale::fallback`] and takes the first table in the
//! chain that names the field, then English's.

use core::fmt;

use hc_calendar::cycle::Sexagenary;
use hc_calendar::shape::{CycleShape, WEEKDAY};
use hc_calendar::{CalendarId, Weekday};

use crate::locale::Locale;
use crate::names::{self, NameContext, NameWidth};

/// One locale's labels for extra fields.
#[derive(Debug, Clone, Copy)]
pub struct FieldNames {
    /// The BCP 47 tag, spelled as [`crate::data::LOCALES`] spells it.
    pub tag: &'static str,
    /// Each labelled field: its identifier and its label.
    pub names: &'static [(&'static str, &'static str)],
}

impl FieldNames {
    /// The label this table gives a field.
    #[must_use]
    pub fn name_of(&self, field: &str) -> Option<&'static str> {
        self.names
            .iter()
            .find(|(id, _)| *id == field)
            .map(|(_, name)| *name)
    }
}

/// A field's label and the tag of the table that gave it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FieldLabel {
    /// What the locale calls the field.
    pub name: &'static str,
    /// The tag of the table that answered: `en` where no other did.
    pub tag: &'static str,
}

/// What `locale` calls an extra field: the first table in its fallback
/// chain that names it, else English's; `None` for a field no table names.
#[must_use]
pub fn label(locale: &Locale, field: &str) -> Option<FieldLabel> {
    let found = |tag: &str| {
        TABLES
            .iter()
            .filter(|table| table.tag == tag)
            .find_map(|table| {
                table.name_of(field).map(|name| FieldLabel {
                    name,
                    tag: table.tag,
                })
            })
    };
    locale
        .fallback()
        .find_map(|candidate| found(candidate.rendered()?.as_str()))
        .or_else(|| found("en"))
}

/// What names the values of a field.
#[derive(Debug, Clone, Copy)]
pub enum ValueNames {
    /// The positions of one of the calendar's cycles, by the kind its
    /// [`hc_calendar::Calendar::cycles`] declares: the locale's names for
    /// them, else the calendar's own ([`names::position_name`]); a count of
    /// the seven-day week the calendar does not name itself takes the
    /// locale's weekday names, Monday being the first.
    Cycle(&'static str),
    /// The sixty stem-branch pairs, in the locale's reading of them
    /// ([`names::sexagenary_names`]), else English's, 甲子 being the first.
    Sexagenary,
    /// Names of the calendar's own, in the orthography its sources use,
    /// the first value's at index 0: what no cycle declares, such as the
    /// Thai ขึ้น and แรม of the two halves of a month.
    Own(&'static [&'static str]),
    /// Names of the calendar's own, as [`ValueNames::Own`], and the words
    /// a language's sources use for them: the first entry of `locales`
    /// whose tag is in the locale's fallback chain, else `own`. The
    /// Burmese halves are လဆန်း and လဆုတ် in Burmese and *waxing* and
    /// *waning* in English.
    Localized {
        /// The names in the calendar's own orthography.
        own: &'static [&'static str],
        /// Each language's names, by the tag of its table in
        /// [`crate::data::LOCALES`].
        locales: &'static [(&'static str, &'static [&'static str])],
    },
    /// A flag, 0 or 1, named by the words [`FLAG_WORDS`] gives the locale,
    /// else English's *no* and *yes*: whether a day is outside the week,
    /// whether an Icelandic summer has its extra week. A flag whose
    /// calendar has words of its own for the two states, the Burmese half's
    /// *waxing* and *waning*, is named by those instead.
    Flag,
    /// A flag that a date writes as a word when it is set and leaves
    /// unwritten when it is not: the Burmese late month, *Late* Tagu,
    /// နှောင်းတန်ခူး. Inside a date — [`NameContext::Format`] — the set
    /// flag is `own`, or the word of the first entry of `locales` whose tag
    /// is in the locale's fallback chain, with whatever space the language
    /// puts after it, and the clear flag is nothing. On its own —
    /// [`NameContext::Standalone`], as a day's extra fields are listed —
    /// it is a [`ValueNames::Flag`].
    Marker {
        /// The word in the calendar's own orthography.
        own: &'static str,
        /// Each language's word, by the tag of its table in
        /// [`crate::data::LOCALES`].
        locales: &'static [(&'static str, &'static str)],
    },
}

/// The words a flag's two values are read as, 0 first, by the tag of the
/// locale's table in [`crate::data::LOCALES`]. Only English's are carried:
/// they are the words of the field labels' own language, which the labels
/// themselves are, and no source was read for another language's.
pub static FLAG_WORDS: &[(&str, [&str; 2])] = &[("en", ["no", "yes"])];

/// A field whose values are named, and what names them.
#[derive(Debug, Clone, Copy)]
pub struct FieldValues {
    /// The calendars whose field this is, by registry identifier.
    pub calendars: &'static [CalendarId],
    /// The field's identifier; `day` names the day of the month of a
    /// calendar whose day is a position in a named cycle, the Pawukon's
    /// seven-day week.
    pub field: &'static str,
    /// The value the field holds at the first name: `1` for a count from
    /// one, `0` for an index.
    pub first: i64,
    /// What names the values.
    pub names: ValueNames,
}

/// What names the values of a field of a calendar, if anything does.
#[must_use]
pub fn values_of(calendar: CalendarId, field: &str) -> Option<&'static FieldValues> {
    VALUES
        .iter()
        .find(|entry| entry.field == field && entry.calendars.contains(&calendar))
}

/// Write the name of the value a field holds, where the field's values
/// are named ([`values_of`]) and this one has a name, and say whether it
/// did; write nothing and answer `false` for a number that is only a
/// number.
///
/// # Errors
///
/// Only what the sink returns.
#[allow(clippy::too_many_arguments)]
pub fn write_value_name(
    locale: &Locale,
    calendar: CalendarId,
    cycles: &[CycleShape],
    field: &str,
    value: i64,
    width: NameWidth,
    context: NameContext,
    out: &mut dyn fmt::Write,
) -> Result<bool, fmt::Error> {
    let Some(named) = values_of(calendar, field) else {
        return Ok(false);
    };
    let Some(index) = value
        .checked_sub(named.first)
        .and_then(|index| usize::try_from(index).ok())
    else {
        return Ok(false);
    };
    let name = match named.names {
        ValueNames::Own(_) | ValueNames::Localized { .. } => own_name(locale, named, index),
        ValueNames::Marker { own, locales } if context == NameContext::Format => match index {
            0 => Some(""),
            1 => Some(
                locale
                    .fallback()
                    .find_map(|candidate| {
                        let tag = candidate.rendered()?;
                        locales
                            .iter()
                            .find(|(name, _)| *name == tag.as_str())
                            .map(|(_, word)| *word)
                    })
                    .unwrap_or(own),
            ),
            _ => None,
        },
        ValueNames::Flag | ValueNames::Marker { .. } => {
            let words = locale
                .fallback()
                .find_map(|candidate| {
                    let tag = candidate.rendered()?;
                    FLAG_WORDS
                        .iter()
                        .find(|(name, _)| *name == tag.as_str())
                        .map(|(_, words)| words)
                })
                .or_else(|| FLAG_WORDS.first().map(|(_, words)| words));
            words.and_then(|words| words.get(index)).copied()
        }
        ValueNames::Cycle(kind) => {
            cycles
                .iter()
                .find(|shape| shape.kind == kind)
                .and_then(|shape| {
                    names::position_name(locale, calendar, shape, index, width, context).or_else(
                        || {
                            let number = u8::try_from(index + 1).ok()?;
                            let weekday = Weekday::from_iso_number(number)?;
                            (kind == WEEKDAY)
                                .then(|| names::weekday_name(locale, weekday, width, context))
                                .flatten()
                        },
                    )
                })
        }
        ValueNames::Sexagenary => {
            if index >= 60 {
                return Ok(false);
            }
            // A locale with no reading of the cycle takes English's, as a
            // name the locale lacks is taken from English everywhere else.
            let position = Sexagenary::from_index(value - named.first);
            let english = names::english();
            let Some((reading, (stem, branch))) = names::sexagenary_names(locale, position)
                .map(|pair| (*locale, pair))
                .or_else(|| {
                    names::sexagenary_names(&english, position).map(|pair| (english, pair))
                })
            else {
                return Ok(false);
            };
            out.write_str(stem)?;
            out.write_str(names::sexagenary_joiner(&reading))?;
            out.write_str(branch)?;
            return Ok(true);
        }
    };
    match name {
        Some(name) => {
            out.write_str(name)?;
            Ok(true)
        }
        None => Ok(false),
    }
}

/// The name of the value at `index` of a field named in the calendar's own
/// words, [`ValueNames::Own`] or [`ValueNames::Localized`], in `locale`.
fn own_name(locale: &Locale, named: &FieldValues, index: usize) -> Option<&'static str> {
    match named.names {
        ValueNames::Own(own) => own.get(index).copied(),
        ValueNames::Localized { own, locales } => locale
            .fallback()
            .find_map(|candidate| {
                let tag = candidate.rendered()?;
                locales
                    .iter()
                    .find(|(name, _)| *name == tag.as_str())
                    .map(|(_, names)| *names)
            })
            .unwrap_or(own)
            .get(index)
            .copied(),
        _ => None,
    }
}

/// The name a field's value has in the calendar's own words —
/// [`ValueNames::Own`] or [`ValueNames::Localized`] — in `locale`, or
/// `None` for a field named otherwise, or not at all. Two fields of one
/// date with the same such name say the same thing: the Burmese phase
/// *waning* is the half of the month *waning*, and a date that writes one
/// has written the other.
#[must_use]
pub fn own_value_name(
    locale: &Locale,
    calendar: CalendarId,
    field: &str,
    value: i64,
) -> Option<&'static str> {
    let named = values_of(calendar, field)?;
    let index = usize::try_from(value.checked_sub(named.first)?).ok()?;
    own_name(locale, named, index)
}

/// A flag, 0 or 1, read as words.
const fn flag(calendars: &'static [CalendarId], field: &'static str) -> FieldValues {
    FieldValues {
        calendars,
        field,
        first: 0,
        names: ValueNames::Flag,
    }
}

/// The three Hindu calendars that name their year in the southern
/// sixty-year cycle, `samvatsara`. The pūrṇimānta calendar names it in the
/// northern, Bārhaspatya, cycle, under `barhaspatya-samvatsara`: the same
/// sixty names, reckoned differently, thirteen apart in 2026
/// (`docs/systems/hindu-calendars.md`).
const SAMVATSARA_CALENDARS: &[CalendarId] = &[
    CalendarId("hindu-lunar"),
    CalendarId("hindu-lunar-surya-siddhanta"),
    CalendarId("hindu-solar-tamil"),
];

/// The Tzolkʼin and the Calendar Round, under the three correlations.
const TZOLKIN_CALENDARS: &[CalendarId] = &[
    CalendarId("maya-tzolkin"),
    CalendarId("maya-tzolkin-gmt2"),
    CalendarId("maya-tzolkin-584286"),
    CalendarId("maya-round"),
    CalendarId("maya-round-gmt2"),
    CalendarId("maya-round-584286"),
];

/// The 819-day count, under the three correlations.
const COUNT_819_CALENDARS: &[CalendarId] = &[
    CalendarId("maya-819"),
    CalendarId("maya-819-gmt2"),
    CalendarId("maya-819-584286"),
];

/// The two year-bearer calendars of Oaxaca.
const YEAR_BEARER_CALENDARS: &[CalendarId] =
    &[CalendarId("zapotec-yza"), CalendarId("mixtec-year")];

/// The three Javanese reckonings.
const JAVANESE_CALENDARS: &[CalendarId] = &[
    CalendarId("javanese"),
    CalendarId("javanese-yogyakarta"),
    CalendarId("javanese-aboge"),
];

/// The three French Republican calendars.
const FRENCH_REPUBLICAN_CALENDARS: &[CalendarId] = &[
    CalendarId("french-republican-arithmetic"),
    CalendarId("french-republican-arithmetic-richards"),
    CalendarId("french-republican-equinox"),
];

/// The two Liberalia Triday calendars.
const LIBERALIA_CALENDARS: &[CalendarId] = &[
    CalendarId("liberalia-triday-solar"),
    CalendarId("liberalia-triday-lunar"),
];

/// The calendars whose `day-of-week` is the ISO weekday, Monday 1 to
/// Sunday 7. The Qumran calendar's counts from the first day to the
/// Sabbath and is not among them.
const ISO_WEEKDAY_CALENDARS: &[CalendarId] = &[
    CalendarId("iso8601"),
    CalendarId("iso8601-week"),
    CalendarId("symmetry454"),
    CalendarId("symmetry010"),
    CalendarId("hanke-henry"),
    CalendarId("hermetic-leap-week"),
    CalendarId("week-and-month"),
    CalendarId("tabot"),
];

/// The calendars of the East Asian lunisolar engine, whose year and month
/// carry their stem-branch pair as an index from 甲子.
const LUNISOLAR_CALENDARS: &[CalendarId] = &[
    CalendarId("chinese"),
    CalendarId("dangi"),
    CalendarId("dangi-kasi"),
    CalendarId("vietnamese"),
    CalendarId("japanese-senmyo"),
    CalendarId("japanese-jokyo"),
    CalendarId("japanese-horyaku"),
    CalendarId("japanese-kansei"),
    CalendarId("japanese-tenpo"),
];

/// A field counting a cycle from one.
const fn counts(
    calendars: &'static [CalendarId],
    field: &'static str,
    cycle: &'static str,
) -> FieldValues {
    FieldValues {
        calendars,
        field,
        first: 1,
        names: ValueNames::Cycle(cycle),
    }
}

/// Every field whose values are named.
///
/// A cycle's names are the calendar's own shape's, or a locale's where
/// `crate::data` carries one; each `Own` list cites where it was read.
pub static VALUES: &[FieldValues] = &[
    counts(SAMVATSARA_CALENDARS, "samvatsara", "samvatsara"),
    counts(
        &[CalendarId("hindu-lunar-purnimanta")],
        "barhaspatya-samvatsara",
        "samvatsara",
    ),
    counts(TZOLKIN_CALENDARS, "tzolkin_name", "day-sign"),
    counts(
        &[CalendarId("aztec-tonalpohualli")],
        "tonalpohualli_sign",
        "day-sign",
    ),
    counts(COUNT_819_CALENDARS, "direction", "direction"),
    counts(YEAR_BEARER_CALENDARS, "year_bearer", "year-bearer"),
    // The Pawukon's day of the month is its seven-day week, the
    // saptawara, and each of its other weeks is a cycle of the same name.
    counts(&[CalendarId("balinese-pawukon")], "day", WEEKDAY),
    counts(&[CalendarId("balinese-pawukon")], "dwiwara", "dwiwara"),
    counts(&[CalendarId("balinese-pawukon")], "triwara", "triwara"),
    counts(&[CalendarId("balinese-pawukon")], "caturwara", "caturwara"),
    counts(&[CalendarId("balinese-pawukon")], "pancawara", "pancawara"),
    counts(&[CalendarId("balinese-pawukon")], "sadwara", "sadwara"),
    counts(&[CalendarId("balinese-pawukon")], "astawara", "astawara"),
    counts(&[CalendarId("balinese-pawukon")], "sangawara", "sangawara"),
    counts(&[CalendarId("balinese-pawukon")], "dasawara", "dasawara"),
    // The pasaran calendar's seven-day week runs from Ahad, Sunday, as its
    // shape names it.
    counts(&[CalendarId("javanese-pasaran")], "dina", WEEKDAY),
    counts(&[CalendarId("javanese-pasaran")], "pasaran", "pasaran"),
    counts(&[CalendarId("akan")], "nnanson", "nnanson"),
    // The seven-day names as the pair writes them, *Fo-Dwo*: the short
    // forms of `hc_calendars_regional::akan::NNAWOTWE_SHORT`, Sunday
    // first, from Wikipedia, "Akan calendar" (wikipedia-akan-calendar).
    FieldValues {
        calendars: &[CalendarId("akan")],
        field: "nnawotwe",
        first: 1,
        names: ValueNames::Own(&["Kwasi", "Dwo", "Bena", "Wukuo", "Ya", "Afi", "Mene"]),
    },
    counts(JAVANESE_CALENDARS, "taun", "taun"),
    counts(JAVANESE_CALENDARS, "windu", "windu"),
    counts(&[CalendarId("qumran")], "mishmar", "mishmar"),
    counts(FRENCH_REPUBLICAN_CALENDARS, "day-of-decade", "decade-day"),
    counts(LIBERALIA_CALENDARS, "day-of-triday", "triday-day"),
    counts(&[CalendarId("archetypes")], "day-of-tweek", "tweek-day"),
    counts(ISO_WEEKDAY_CALENDARS, "day-of-week", WEEKDAY),
    counts(
        &[CalendarId("week-and-month")],
        "week-of-month",
        "week-of-month",
    ),
    FieldValues {
        calendars: LUNISOLAR_CALENDARS,
        field: "sexagenary_year",
        first: 0,
        names: ValueNames::Sexagenary,
    },
    FieldValues {
        calendars: LUNISOLAR_CALENDARS,
        field: "sexagenary_month",
        first: 0,
        names: ValueNames::Sexagenary,
    },
    FieldValues {
        calendars: &[CalendarId("sexagenary")],
        field: "sexagenary",
        first: 1,
        names: ValueNames::Sexagenary,
    },
    // The two halves of the month, `waning` 0 for the waxing half: ขึ้น
    // and แรม as `docs/systems/thai-lunar.md` writes them from
    // wikipedia-th-thai-lunar, ຂຶ້ນ and ແຮມ as `hc_calendars_regional::lao`
    // writes them from dupertuis1981, កើត and រោច as
    // `hc_calendars_regional::khmer` writes them from tum-chhankitek, and
    // in English Tum's *keit* and *roaj*, "1 keit Bos"; the Burmese လဆန်း
    // and လဆုတ်, *waxing* and *waning* in English, from
    // wikipedia-burmese-calendar, "waxing (လဆန်း) and waning (လဆုတ်)".
    FieldValues {
        calendars: &[CalendarId("thai-lunar")],
        field: "waning",
        first: 0,
        names: ValueNames::Own(&["ขึ้น", "แรม"]),
    },
    FieldValues {
        calendars: &[CalendarId("lao")],
        field: "waning",
        first: 0,
        names: ValueNames::Own(&["ຂຶ້ນ", "ແຮມ"]),
    },
    FieldValues {
        calendars: &[CalendarId("khmer")],
        field: "waning",
        first: 0,
        names: ValueNames::Localized {
            own: &["កើត", "រោច"],
            locales: &[("en", &["keit", "roaj"])],
        },
    },
    FieldValues {
        calendars: &[CalendarId("burmese")],
        field: "waning",
        first: 0,
        names: ValueNames::Localized {
            own: &["လဆန်း", "လဆုတ်"],
            locales: &[("en", &["waxing", "waning"])],
        },
    },
    // The late Tagu and Kason of the Burmese year, which a date writes as
    // နှောင်း before the month, «၁၃၇၈ ခုနှစ်၊ နှောင်းတန်ခူးလဆန်း ၂ ရက်»
    // (wikipedia-burmese-calendar, 29 March 2017), a term Burmese Wikipedia's
    // နှစ်ဆန်းတစ်ရက်နေ့ gives for both months, «နှောင်းတန်ခူး သို့မဟုတ်
    // နှောင်းကဆုန်» (read 2026-09-28), and English as *Late*: "Late Tagu"
    // and "Late Kason" are the month names of Yan Naing Aye's own calendar
    // code (yan9a/mmcal, `ceMmDateTime.js`, read 2026-09-28), whose
    // blog glosses *Hnaung Tagu* as "Late Tagu" (yannaingaye2013).
    FieldValues {
        calendars: &[CalendarId("burmese")],
        field: "late",
        first: 0,
        names: ValueNames::Marker {
            own: "နှောင်း",
            locales: &[("en", "Late ")],
        },
    },
    // The flags, read as *no* and *yes*: whether an Icelandic summer has
    // its extra week, whether a day of the three perennial calendars is
    // outside the week, whether an imperial-year day is before the count's
    // adoption on 1 January 1873, and whether a day of the continuous week
    // is its common rest day. The Burmese half is named by its own words
    // above.
    flag(
        &[
            CalendarId("icelandic"),
            CalendarId("icelandic-julian"),
            CalendarId("icelandic-friday"),
            CalendarId("icelandic-julian-friday"),
            CalendarId("icelandic-medieval"),
            CalendarId("icelandic-almanac"),
        ],
        "sumarauki",
    ),
    flag(
        &[
            CalendarId("international-fixed"),
            CalendarId("positivist"),
            CalendarId("world-calendar"),
        ],
        "outside-the-week",
    ),
    flag(&[CalendarId("japanese-imperial")], "proleptic"),
    flag(&[CalendarId("soviet-week")], "rest-day"),
    // The Burmese phases: in English as `hc_calendars_regional::burmese`
    // writes a date, "Nayon waxing 3, 1374 ME", from yannaingaye2013, and
    // in Burmese as wikipedia-burmese-calendar names the halves and the
    // days that end them, လဆန်း, လပြည့် for the full moon, လဆုတ် and
    // လကွယ် for the new moon.
    FieldValues {
        calendars: &[CalendarId("burmese")],
        field: "phase",
        first: 0,
        names: ValueNames::Localized {
            own: &["လဆန်း", "လပြည့်", "လဆုတ်", "လကွယ်"],
            locales: &[("en", &["waxing", "full moon", "waning", "new moon"])],
        },
    },
];

/// Every table of labels.
///
/// English's labels are the words the calendars' system documents and
/// modules use for their fields, from the sources they cite: `baktun` is
/// the Long Count's place as `docs/systems/mesoamerican-counts.md` names
/// it, `samvatsara` and `barhaspatya-samvatsara` the Hindu year's name in
/// the southern and the northern cycle, in Sewell and Dikshit's words for
/// them, a samvatsara "in luni-solar or southern reckoning" (`sewell1896`,
/// the example of 1752) and a "Barhaspatya samvatsara" (Art. 54) of "the
/// northern cycle" (the example of 1822), as
/// `docs/systems/hindu-calendars.md` names them, `indiction` the
/// Byzantine cycle as `hc_calendars_solar::byzantine` does. No other
/// language's table is carried yet: no source in another language naming
/// these fields was read.
pub static TABLES: &[FieldNames] = &[FieldNames {
    tag: "en",
    names: &[
        ("amete-alem-year", "Amete Alem year"),
        ("amli-year", "Amli year"),
        ("astawara", "Astawara"),
        ("astronomical-year", "Astronomical year"),
        ("baktun", "Baktun"),
        (
            "barhaspatya-samvatsara",
            "Barhaspatya samvatsara (northern cycle)",
        ),
        ("base-year", "Base year"),
        ("branch", "Earthly branch"),
        ("caturwara", "Caturwara"),
        ("common-era-year", "Common Era year"),
        ("cycle", "Cycle"),
        ("cycle-year", "Year of the cycle"),
        ("dasawara", "Dasawara"),
        ("day-of-decade", "Day of the décade"),
        ("day-of-stata-week", "Day of the Stata week"),
        ("day-of-triday", "Day of the triday"),
        ("day-of-tweek", "Day of the tweek"),
        ("day-of-week", "Day of the week"),
        ("day-of-year", "Day of the year"),
        ("decade", "Décade"),
        ("dina", "Weekday"),
        ("direction", "Direction"),
        ("dwiwara", "Dwiwara"),
        ("eastern-day", "Eastern day"),
        ("eastern-month", "Eastern month"),
        ("elapsed", "Days since the station"),
        ("fortnight-day", "Day of the fortnight"),
        ("gregorian-year", "Gregorian year"),
        ("indiction", "Indiction"),
        ("julian-day-number", "Julian Day Number"),
        ("julian-year", "Julian year"),
        ("katun", "Katun"),
        ("kin", "Kin"),
        ("kull-i-shay", "Kull-i-Shayʼ"),
        ("kurup", "Kurup"),
        ("late", "Late month"),
        ("laukika-year", "Laukika year"),
        ("mishmar", "Mishmar"),
        ("modified-julian-day", "Modified Julian Day"),
        ("nabonassar-year", "Nabonassar year"),
        ("neptu", "Neptu"),
        ("nnanson", "Nnanson"),
        ("nnawotwe", "Nnawɔtwe"),
        ("olympiad", "Olympiad"),
        ("outside-the-week", "Outside the week"),
        ("pancawara", "Pancawara"),
        ("pasaran", "Pasaran"),
        ("phase", "Phase"),
        ("printed-year", "Printed year"),
        ("proleptic", "Proleptic"),
        ("quarter", "Quarter"),
        ("regnal-year", "Year of the reign"),
        ("related-gregorian-year", "Related Gregorian year"),
        ("rest-day", "Common rest day"),
        ("sadwara", "Sadwara"),
        ("samvatsara", "Samvatsara (southern reckoning)"),
        ("sangawara", "Sangawara"),
        ("season", "Misseri"),
        ("sexagenary", "Stem and branch"),
        ("sexagenary_month", "Stem and branch of the month"),
        ("sexagenary_year", "Stem and branch of the year"),
        ("signed-year", "Signed year"),
        ("soviet-week-day", "Day of the continuous week"),
        ("stata-weekly", "Stata weekly date"),
        ("station", "Station"),
        ("stem", "Heavenly stem"),
        ("sumarauki", "Sumarauki"),
        ("taun", "Year of the windu"),
        ("tiruvalluvar-year", "Tiruvalluvar year"),
        ("tonalpohualli_number", "Tonalpohualli number"),
        ("tonalpohualli_sign", "Tonalpohualli day sign"),
        ("triday", "Triday"),
        ("triwara", "Triwara"),
        ("tun", "Tun"),
        ("tweek", "Tweek"),
        ("tzolkin_name", "Tzolkʼin day sign"),
        ("tzolkin_number", "Tzolkʼin number"),
        ("uinal", "Uinal"),
        ("vahid", "Váḥid"),
        ("vikrama-year", "Vikrama year"),
        ("waning", "Half of the month"),
        ("week", "Week"),
        ("week-length", "Length of the week"),
        ("week-of-month", "Week of the month"),
        ("week-year", "Week-numbering year"),
        ("windu", "Windu"),
        ("year-of-cycle", "Year of the cycle"),
        ("year-of-olympiad", "Year of the Olympiad"),
        ("year-of-vahid", "Year of the Váḥid"),
        ("year_bearer", "Year bearer"),
        ("year_bearer_number", "Year bearer's number"),
        ("year_of_cycle", "Year of the cycle"),
        ("yerm-of-cycle", "Yerm of the cycle"),
    ],
}];
