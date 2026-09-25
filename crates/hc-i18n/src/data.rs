//! The locale data itself.
//!
//! Everything in this module is `&'static` data and nothing in it is logic.
//! One language is one [`LocaleData`] value plus one line in [`LOCALES`];
//! the lookup in [`crate::names`] does not know which languages exist and
//! gains no branch when a new one arrives.
//!
//! # Provenance
//!
//! The vocabulary follows the Unicode CLDR common locale data (the
//! `main/<locale>.xml` `calendars` sections) and is hand-checked, not
//! generated: it is a subset chosen for calendar work, and where CLDR offers
//! several alternatives the most widely used one is taken. Fields a locale
//! does not state are left empty on purpose, because an empty field
//! inherits, and inheriting is more correct than copying.
//!
//! What that means in practice:
//!
//! * A locale entry covers months, weekdays, day periods, eras and — for
//!   some locales — quarters. Finer CLDR day periods, date patterns,
//!   interval patterns and relative-time strings are not here.
//! * Non-Gregorian vocabulary is carried for the calendars where the names
//!   genuinely differ: Hijri months in Arabic and English, Hebrew months in
//!   Hebrew and English, Babylonian months in English, the Chinese lunisolar months in both Chinese
//!   scripts, their Japanese traditional names, and the Japanese era names.
//! * Locales exist for a calendar's own language: Amharic for the Ethiopic
//!   calendar, Coptic for the Coptic, Burmese for the Burmese, Tibetan for
//!   the Tibetan, Nepali for the Bikram Sambat and Nepal Sambat, Sanskrit
//!   and Hindi for the Hindu lunisolar and Vikrami solar calendars, Tamil,
//!   Malayalam and Bengali for the regional solar calendars, Yucatec Maya
//!   and Nahuatl for the Mesoamerican counts, Balinese and Javanese for the
//!   Pawukon and the pasaran, Syriac for the Assyrian calendar, Kabyle and
//!   Standard Moroccan Tamazight for the Berber calendar, and Mandaic for
//!   the Mandaean. Their Gregorian vocabulary is CLDR's where CLDR has the
//!   locale (CLDR 48, `common/main/<locale>.xml`, read 2026-09-25 and
//!   2026-09-26 from the `release-48` tag of github.com/unicode-org/cldr);
//!   CLDR has no `cop`, `ban`, `yua`, `nah` or `mid`, so those entries state
//!   only what the calendars' own sources say. Where a CLDR value is the
//!   inheritance marker `↑↑↑`, the value it resolves to under CLDR's aliases
//!   (abbreviated to wide, format narrow to stand-alone narrow) is what is
//!   written here, and a width that resolves to the wide form is left empty.
//! * The sexagenary cycle is written in whichever of the readings
//!   `hc_calendar::cycle::readings` catalogues the locale uses; only the
//!   zodiac animals are spelled here, for Chinese in both scripts, Japanese,
//!   Korean, Vietnamese and English.

use hc_calendar::cycle::readings;
use hc_calendar::{CalendarId, Weekday};

use crate::casing::CasingStyle;
use crate::direction::Direction;
use crate::names::{
    CalendarNames, ContextualNames, CycleNames, EraNames, LocaleData, SexagenaryNames, WidthSet,
};

// --- construction helpers -------------------------------------------------
//
// These exist only to keep a locale entry readable; they add no behaviour.

const fn widths(
    wide: &'static [&'static str],
    abbreviated: &'static [&'static str],
    narrow: &'static [&'static str],
) -> WidthSet {
    WidthSet {
        wide,
        abbreviated,
        short: &[],
        narrow,
    }
}

const fn weekday_widths(
    wide: &'static [&'static str],
    abbreviated: &'static [&'static str],
    short: &'static [&'static str],
    narrow: &'static [&'static str],
) -> WidthSet {
    WidthSet {
        wide,
        abbreviated,
        short,
        narrow,
    }
}

/// The calendars that share the Gregorian month names.
///
/// They differ in how they count years, not in what they call the months,
/// so one vocabulary entry serves all of them. Every identifier here must be
/// one the registry answers to, or its entry is inert; `hyper-calendar`'s
/// vocabulary test checks that.
const GREGORIAN_MONTH_CALENDARS: &[CalendarId] = &[
    CalendarId("gregory"),
    CalendarId("julian"),
    CalendarId("revised-julian"),
    CalendarId("buddhist"),
    CalendarId("roc"),
    CalendarId("juche"),
    CalendarId("holocene"),
    CalendarId("korean-regnal"),
    CalendarId("japanese-imperial"),
    CalendarId("japanese"),
    CalendarId("japanese-northern"),
    CalendarId("japanese-southern"),
    CalendarId("japanese-proclaimed"),
    CalendarId("roman-auc"),
    CalendarId("byzantine"),
    CalendarId("symmetry454"),
    CalendarId("symmetry010"),
    CalendarId("world-calendar"),
    CalendarId("julian-gregorian-catholic"),
    CalendarId("julian-gregorian-fr"),
    CalendarId("julian-gregorian-nl"),
    CalendarId("julian-gregorian-de-catholic"),
    CalendarId("julian-gregorian-hu"),
    CalendarId("julian-gregorian-de-protestant"),
    CalendarId("julian-gregorian-gb"),
    CalendarId("julian-gregorian-se"),
    CalendarId("julian-gregorian-bg"),
    CalendarId("julian-gregorian-ru"),
    CalendarId("julian-gregorian-ro"),
    CalendarId("julian-gregorian-gr"),
    CalendarId("swedish-1700"),
];

/// A month cycle from names already shaped into widths and contexts.
const fn month_cycle(names: ContextualNames) -> CycleNames {
    CycleNames::new(hc_calendar::shape::MONTH, names)
}

/// A month cycle from a plain list of names, the same in every width.
const fn months(names: &'static [&'static str]) -> CycleNames {
    month_cycle(ContextualNames::same(widths(names, &[], &[])))
}

/// Any other cycle a calendar declares — a day-sign, a pasaran, a wuku —
/// from a plain list of names, the same in every width.
const fn cycle(kind: &'static str, names: &'static [&'static str]) -> CycleNames {
    CycleNames::new(kind, ContextualNames::same(widths(names, &[], &[])))
}

/// The five Hijri calendars, which share one set of Arabic month names.
const ISLAMIC_CALENDARS: &[CalendarId] = &[
    CalendarId("islamic-civil"),
    CalendarId("islamic-tbla"),
    CalendarId("islamic-umalqura"),
    CalendarId("islamic-rgsa"),
    CalendarId("islamic-fatimid"),
];

/// The Hebrew calendar.
const HEBREW_CALENDARS: &[CalendarId] = &[CalendarId("hebrew")];

/// The Babylonian calendar of the Seleucid era.
const BABYLONIAN_CALENDARS: &[CalendarId] = &[CalendarId("babylonian")];

/// The Thai Buddhist calendar, which counts years its own way and names
/// the months as the Gregorian calendar does — so it appears both here,
/// for its era, and in [`GREGORIAN_MONTH_CALENDARS`], for its months.
const BUDDHIST_CALENDARS: &[CalendarId] = &[CalendarId("buddhist")];

/// The lunisolar calendars whose months are numbered rather than named, and
/// so share one set of names: First Month, 正月, 정월.
const NUMBERED_LUNISOLAR_CALENDARS: &[CalendarId] = &[
    CalendarId("chinese"),
    CalendarId("chinese-regnal"),
    CalendarId("tibetan"),
    CalendarId("dangi"),
    CalendarId("vietnamese"),
    CalendarId("japanese-tenpo"),
    CalendarId("japanese-kansei"),
    CalendarId("japanese-horyaku"),
    CalendarId("japanese-jokyo"),
    CalendarId("japanese-senmyo"),
];

/// The Solar Hijri calendar.
///
/// The registry has it twice: `persian-arithmetic`, the Birashk
/// arithmetic calendar, and `persian`, the astronomical calendar of
/// `hc-calendars-equinox`. Their months have the same names.
const PERSIAN_CALENDARS: &[CalendarId] = &[CalendarId("persian-arithmetic"), CalendarId("persian")];

/// The Coptic calendar.
const COPTIC_CALENDARS: &[CalendarId] = &[CalendarId("coptic")];

/// The Ethiopic calendar.
const ETHIOPIC_CALENDARS: &[CalendarId] = &[CalendarId("ethiopic")];

/// The Burmese calendar.
const BURMESE_CALENDARS: &[CalendarId] = &[CalendarId("burmese")];

/// The Rumi calendar.
const RUMI_CALENDARS: &[CalendarId] = &[CalendarId("rumi")];

/// The Nanakshahi calendar.
const NANAKSHAHI_CALENDARS: &[CalendarId] = &[CalendarId("nanakshahi")];

/// The two Armenian calendars, which share one set of month names.
const ARMENIAN_CALENDARS: &[CalendarId] = &[CalendarId("armenian"), CalendarId("armenian-fixed")];

/// The Japanese era calendars, in every court reading and both era
/// reckonings — they share their month names.
const JAPANESE_CALENDARS: &[CalendarId] = &[
    CalendarId("japanese"),
    CalendarId("japanese-northern"),
    CalendarId("japanese-southern"),
    CalendarId("japanese-proclaimed"),
];

const fn gregorian(
    months: &'static [CycleNames],
    eras: EraNames,
    quarters: ContextualNames,
) -> CalendarNames {
    CalendarNames {
        calendars: GREGORIAN_MONTH_CALENDARS,
        cycles: months,
        leap_month_prefix: "",
        eras,
        quarters,
    }
}

const fn lunisolar(
    calendars: &'static [CalendarId],
    months: &'static [CycleNames],
    leap_month_prefix: &'static str,
) -> CalendarNames {
    CalendarNames {
        calendars,
        cycles: months,
        leap_month_prefix,
        eras: EraNames::EMPTY,
        quarters: ContextualNames::EMPTY,
    }
}

const fn dated(
    calendars: &'static [CalendarId],
    months: &'static [CycleNames],
    era_codes: &'static [&'static str],
    era_names: &'static [&'static str],
) -> CalendarNames {
    CalendarNames {
        calendars,
        cycles: months,
        leap_month_prefix: "",
        eras: EraNames {
            codes: era_codes,
            names: widths(era_names, &[], &[]),
            calendars: &[],
        },
        quarters: ContextualNames::EMPTY,
    }
}

/// The calendars that count years in BCE/CE.
///
/// A subset of [`GREGORIAN_MONTH_CALENDARS`], because the Buddhist calendar
/// counts in BE, the Minguo calendar in 民國 and the Japanese imperial one
/// in 皇紀, while all three write the months exactly as the Gregorian
/// calendar does.
const GREGORIAN_ERA_CALENDARS: &[CalendarId] = &[
    CalendarId("gregory"),
    CalendarId("iso8601-week"),
    CalendarId("iso8601-ordinal"),
    CalendarId("julian"),
    CalendarId("revised-julian"),
];

const fn gregorian_eras(
    wide: &'static [&'static str],
    abbreviated: &'static [&'static str],
    narrow: &'static [&'static str],
) -> EraNames {
    EraNames {
        codes: &["bc", "ad"],
        names: widths(wide, abbreviated, narrow),
        calendars: GREGORIAN_ERA_CALENDARS,
    }
}

/// The five modern Japanese eras, oldest first.
///
/// Earlier nengō exist in the hundreds; the Japanese calendar as an official
/// civil calendar starts with Meiji, and this is the set every implementation
/// agrees on.
const JAPANESE_ERA_CODES: &[&str] = &["meiji", "taisho", "showa", "heisei", "reiwa"];
/// The Latin-letter era abbreviations shared by Japanese and English data.
const JAPANESE_ERA_NARROW: &[&str] = &["M", "T", "S", "H", "R"];

// --- root -----------------------------------------------------------------

/// The floor of every lookup.
///
/// CLDR's own root locale names months `M01`…`M12` rather than inventing
/// English ones, and this follows it: a caller that reaches root is told
/// plainly that no language claimed the field.
pub static ROOT: LocaleData = LocaleData {
    tag: "und",
    direction: Direction::LeftToRight,
    numbering: "latn",
    first_day_of_week: Weekday::Monday,
    casing: CasingStyle::Standard,
    capitalises_month_names: true,
    weekdays: ContextualNames::same(weekday_widths(
        &["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"],
        &[],
        &[],
        &["M", "T", "W", "T", "F", "S", "S"],
    )),
    day_periods: ContextualNames::same(widths(&["AM", "PM"], &[], &["a", "p"])),
    cycle: SexagenaryNames::EMPTY,
    calendars: &[gregorian(
        &[month_cycle(ContextualNames::same(widths(
            &[
                "M01", "M02", "M03", "M04", "M05", "M06", "M07", "M08", "M09", "M10", "M11", "M12",
            ],
            &[],
            &[
                "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12",
            ],
        )))],
        gregorian_eras(&["BCE", "CE"], &[], &[]),
        ContextualNames::same(widths(&["Q1", "Q2", "Q3", "Q4"], &[], &[])),
    )],
};

// --- Amharic --------------------------------------------------------------
//
// The language of the Ethiopic calendar. Gregorian vocabulary from CLDR 48
// `common/main/am.xml`, `calendar type="gregorian"`; the Ethiopic months
// from its `calendar type="ethiopic"`, which spells the thirteenth ጳጉሜን
// where the calendar's own table (Wikipedia, "Ethiopian calendar") has
// ጳጐሜን, so this is an override and not a repeat. CLDR's `am` carries no
// Amharic names for the Ethiopic eras — its `ethiopic` eras are the
// inheritance marker, resolving to root's Latin "AA" and "AM" — so none are
// written here; the Amharic ዓመተ ዓለም and ዓመተ ምሕረት are what CLDR gives as
// the *Gregorian* era names, and they are carried there. The Coptic
// calendar's era abbreviation ዓ/ም is CLDR's `calendar type="coptic"`.
// Ethiopia's week begins on Sunday (CLDR 48 `supplementalData.xml`,
// `weekData/firstDay`, ET) and its default numbering system is `latn`.

const AM_CALENDARS: &[CalendarNames] = &[
    gregorian(
        &[month_cycle(ContextualNames::same(widths(
            &[
                "ጃንዋሪ",
                "ፌብሩዋሪ",
                "ማርች",
                "ኤፕሪል",
                "ሜይ",
                "ጁን",
                "ጁላይ",
                "ኦገስት",
                "ሴፕቴምበር",
                "ኦክቶበር",
                "ኖቬምበር",
                "ዲሴምበር",
            ],
            &[
                "ጃን",
                "ፌብ",
                "ማርች",
                "ኤፕሪ",
                "ሜይ",
                "ጁን",
                "ጁላይ",
                "ኦገስ",
                "ሴፕቴ",
                "ኦክቶ",
                "ኖቬም",
                "ዲሴም",
            ],
            &["ጃ", "ፌ", "ማ", "ኤ", "ሜ", "ጁ", "ጁ", "ኦ", "ሴ", "ኦ", "ኖ", "ዲ"],
        )))],
        gregorian_eras(&["ዓመተ ዓለም", "ዓመተ ምሕረት"], &["ዓ/ዓ", "ዓ/ም"], &[]),
        ContextualNames::same(widths(
            &["1ኛው ሩብ", "2ኛው ሩብ", "3ኛው ሩብ", "4ኛው ሩብ"],
            &["ሩብ1", "ሩብ2", "ሩብ3", "ሩብ4"],
            &[],
        )),
    ),
    dated(
        ETHIOPIC_CALENDARS,
        &[months(&[
            "መስከረም",
            "ጥቅምት",
            "ኅዳር",
            "ታኅሣሥ",
            "ጥር",
            "የካቲት",
            "መጋቢት",
            "ሚያዝያ",
            "ግንቦት",
            "ሰኔ",
            "ሐምሌ",
            "ነሐሴ",
            "ጳጉሜን",
        ])],
        &[],
        &[],
    ),
    dated(COPTIC_CALENDARS, &[], &["am"], &["ዓ/ም"]),
];

const AM: LocaleData = LocaleData {
    tag: "am",
    direction: Direction::LeftToRight,
    numbering: "latn",
    first_day_of_week: Weekday::Sunday,
    casing: CasingStyle::Standard,
    capitalises_month_names: false,
    weekdays: ContextualNames::same(weekday_widths(
        &["ሰኞ", "ማክሰኞ", "ረቡዕ", "ሐሙስ", "ዓርብ", "ቅዳሜ", "እሑድ"],
        &["ሰኞ", "ማክሰ", "ረቡዕ", "ሐሙስ", "ዓርብ", "ቅዳሜ", "እሑድ"],
        &["ሰ", "ማ", "ረ", "ሐ", "ዓ", "ቅ", "እ"],
        &["ሰ", "ማ", "ረ", "ሐ", "ዓ", "ቅ", "እ"],
    )),
    day_periods: ContextualNames::same(widths(&["ጥዋት", "ከሰዓት"], &[], &["ጠ", "ከ"])),
    cycle: SexagenaryNames::EMPTY,
    calendars: AM_CALENDARS,
};

// --- Arabic ---------------------------------------------------------------

const AR_CALENDARS: &[CalendarNames] = &[
    gregorian(
        &[month_cycle(ContextualNames::same(widths(
            &[
                "يناير",
                "فبراير",
                "مارس",
                "أبريل",
                "مايو",
                "يونيو",
                "يوليو",
                "أغسطس",
                "سبتمبر",
                "أكتوبر",
                "نوفمبر",
                "ديسمبر",
            ],
            &[],
            &[],
        )))],
        gregorian_eras(&["قبل الميلاد", "ميلادي"], &["ق.م", "م"], &[]),
        ContextualNames::EMPTY,
    ),
    dated(
        ISLAMIC_CALENDARS,
        &[months(&[
            "محرم",
            "صفر",
            "ربيع الأول",
            "ربيع الآخر",
            "جمادى الأولى",
            "جمادى الآخرة",
            "رجب",
            "شعبان",
            "رمضان",
            "شوال",
            "ذو القعدة",
            "ذو الحجة",
        ])],
        &["ah"],
        &["هـ"],
    ),
    // The Coptic months as Egyptian Arabic prints them, an override of the
    // Coptic-script names the calendar declares for itself. Source: the
    // months table of Wikipedia, "Coptic calendar", retrieved 2026-09-22,
    // which follows Hinds and Badawi, A Dictionary of Egyptian Arabic.
    dated(
        COPTIC_CALENDARS,
        &[months(&[
            "توت",
            "بابه",
            "هاتور",
            "كياك",
            "طوبه",
            "أمشير",
            "برمهات",
            "برموده",
            "بشنس",
            "بأونه",
            "أبيب",
            "مسرا",
            "نسيئ",
        ])],
        &[],
        &[],
    ),
];

const AR: LocaleData = LocaleData {
    tag: "ar",
    direction: Direction::RightToLeft,
    numbering: "arab",
    first_day_of_week: Weekday::Saturday,
    casing: CasingStyle::Standard,
    capitalises_month_names: false,
    weekdays: ContextualNames::same(weekday_widths(
        &[
            "الاثنين",
            "الثلاثاء",
            "الأربعاء",
            "الخميس",
            "الجمعة",
            "السبت",
            "الأحد",
        ],
        &[],
        &[],
        &["ن", "ث", "ر", "خ", "ج", "س", "ح"],
    )),
    day_periods: ContextualNames::same(widths(&["ص", "م"], &[], &[])),
    cycle: SexagenaryNames::EMPTY,
    calendars: AR_CALENDARS,
};

// --- Balinese -------------------------------------------------------------
//
// The language of the Pawukon. CLDR has no `ban` locale, so this entry has
// no Gregorian vocabulary — the empty Gregorian entry below is there
// because every locale states that calendar, and it inherits — and no day
// periods or eras. The ten cycles are the names
// `hc_calendars_regional::balinese_pawukon` declares with its shape, which
// follow Reingold and Dershowitz, *Calendrical Calculations* (4th ed.,
// 2018), §10.6, as that module's Sources say; the locale restates them so
// that the language, and not only the calendar, claims them. The seven
// Saptawara, Redite (Sunday) to Saniscara (Saturday), are the locale's
// weekdays too, Monday first as this crate orders them. No source read
// prints any of the names in Balinese script (Wikipedia, "Pawukon
// calendar", read 2026-09-26, has none), so none are carried. The week
// begins on Sunday because CLDR 48 `weekData/firstDay` says so for ID, and
// there is no CLDR locale to say otherwise; numbering is `latn`, this
// crate having no Balinese numerals.

const BAN_WUKU: &[&str] = &[
    "Sinta",
    "Landep",
    "Ukir",
    "Kulantir",
    "Taulu",
    "Gumbreg",
    "Wariga",
    "Warigadean",
    "Julungwangi",
    "Sungsang",
    "Dungulan",
    "Kuningan",
    "Langkir",
    "Medangsia",
    "Pujut",
    "Pahang",
    "Krulut",
    "Merakih",
    "Tambir",
    "Medangkungan",
    "Matal",
    "Uye",
    "Menail",
    "Prangbakat",
    "Bala",
    "Ugu",
    "Wayang",
    "Kelawu",
    "Dukut",
    "Watugunung",
];

const BAN_SAPTAWARA: &[&str] = &[
    "Redite",
    "Coma",
    "Anggara",
    "Buda",
    "Wraspati",
    "Sukra",
    "Saniscara",
];

const BAN_CALENDARS: &[CalendarNames] = &[
    gregorian(&[], EraNames::EMPTY, ContextualNames::EMPTY),
    CalendarNames {
        calendars: &[CalendarId("balinese-pawukon")],
        cycles: &[
            months(BAN_WUKU),
            cycle("ekawara", &["Luang"]),
            cycle("dwiwara", &["Menga", "Pepet"]),
            cycle("triwara", &["Pasah", "Beteng", "Kajeng"]),
            cycle("caturwara", &["Sri", "Laba", "Jaya", "Menala"]),
            cycle("pancawara", &["Umanis", "Paing", "Pon", "Wage", "Kliwon"]),
            cycle(
                "sadwara",
                &["Tungleh", "Aryang", "Urukung", "Paniron", "Was", "Maulu"],
            ),
            cycle(hc_calendar::shape::WEEKDAY, BAN_SAPTAWARA),
            cycle(
                "astawara",
                &[
                    "Sri", "Indra", "Guru", "Yama", "Ludra", "Brahma", "Kala", "Uma",
                ],
            ),
            cycle(
                "sangawara",
                &[
                    "Dangu", "Jangur", "Gigis", "Nohan", "Ogan", "Erangan", "Urungan", "Tulus",
                    "Dadi",
                ],
            ),
            cycle(
                "dasawara",
                &[
                    "Pandita", "Pati", "Suka", "Duka", "Sri", "Manuh", "Manusa", "Raja", "Dewa",
                    "Raksasa",
                ],
            ),
        ],
        leap_month_prefix: "",
        eras: EraNames::EMPTY,
        quarters: ContextualNames::EMPTY,
    },
];

const BAN: LocaleData = LocaleData {
    tag: "ban",
    direction: Direction::LeftToRight,
    numbering: "latn",
    first_day_of_week: Weekday::Sunday,
    casing: CasingStyle::Standard,
    capitalises_month_names: true,
    weekdays: ContextualNames::same(weekday_widths(
        &[
            "Coma",
            "Anggara",
            "Buda",
            "Wraspati",
            "Sukra",
            "Saniscara",
            "Redite",
        ],
        &[],
        &[],
        &[],
    )),
    day_periods: ContextualNames::EMPTY,
    cycle: SexagenaryNames::EMPTY,
    calendars: BAN_CALENDARS,
};

// --- Bengali --------------------------------------------------------------
//
// The language of the Bengali solar calendar. Gregorian vocabulary from
// CLDR 48 `common/main/bn.xml`, `calendar type="gregorian"`: the wide
// months, the abbreviated ones (March, May and June resolving to the wide
// form), the stand-alone narrow ones, the weekdays, the format-wide and
// stand-alone quarters, and the eras, whose wide BC form is the inheritance
// marker and resolves to the abbreviation. CLDR's `bn` carries no day
// periods of its own — its am/pm are the inheritance marker, resolving to
// root's — so none are written. The twelve months of its `calendar
// type="indian"`, Chaitra first, are keyed to the national calendar with
// CLDR's era abbreviation সাল; the same twelve words, Boishakh first, are
// the Bengali solar year's, and are keyed to `hindu-solar-bengali` and
// `bangladeshi` — the same order and, letter for letter, the same
// spellings the months table of Wikipedia, "Bangladeshi national
// calendar", prints, which `hc_calendars_solar::bangladeshi` declares with
// its shape; the locale states them so that the language, and not only
// the calendar, claims them. The week begins on Sunday (CLDR 48
// `weekData/firstDay`, BD and IN alike) and the default numbering system
// is `beng`.

const BN_SAKA_MONTHS: &[&str] = &[
    "চৈত্র",
    "বৈশাখ",
    "জ্যৈষ্ঠ",
    "আষাঢ়",
    "শ্রাবণ",
    "ভাদ্র",
    "আশ্বিন",
    "কার্তিক",
    "অগ্রহায়ণ",
    "পৌষ",
    "মাঘ",
    "ফাল্গুন",
];

const BN_BANGABDA_MONTHS: &[&str] = &[
    "বৈশাখ",
    "জ্যৈষ্ঠ",
    "আষাঢ়",
    "শ্রাবণ",
    "ভাদ্র",
    "আশ্বিন",
    "কার্তিক",
    "অগ্রহায়ণ",
    "পৌষ",
    "মাঘ",
    "ফাল্গুন",
    "চৈত্র",
];

const BN_CALENDARS: &[CalendarNames] = &[
    gregorian(
        &[month_cycle(ContextualNames::same(widths(
            &[
                "জানুয়ারি",
                "ফেব্রুয়ারি",
                "মার্চ",
                "এপ্রিল",
                "মে",
                "জুন",
                "জুলাই",
                "আগস্ট",
                "সেপ্টেম্বর",
                "অক্টোবর",
                "নভেম্বর",
                "ডিসেম্বর",
            ],
            &[
                "জানু",
                "ফেব",
                "মার্চ",
                "এপ্রি",
                "মে",
                "জুন",
                "জুল",
                "আগ",
                "সেপ",
                "অক্টো",
                "নভে",
                "ডিসে",
            ],
            &[
                "জা",
                "ফে",
                "মা",
                "এ",
                "মে",
                "জুন",
                "জু",
                "আ",
                "সে",
                "অ",
                "ন",
                "ডি",
            ],
        )))],
        gregorian_eras(&["খ্রিস্টপূর্ব", "খ্রিস্টাব্দ"], &["খ্রিস্টপূর্ব", "খৃষ্টাব্দ"], &[]),
        ContextualNames {
            format: widths(
                &["ত্রৈমাসিক", "দ্বিতীয় ত্রৈমাসিক", "তৃতীয় ত্রৈমাসিক", "চতুর্থ ত্রৈমাসিক"],
                &[],
                &[],
            ),
            standalone: widths(&[], &["Q1", "Q2", "Q3", "Q4"], &["১", "২", "৩", "৪"]),
        },
    ),
    dated(
        &[CalendarId("indian")],
        &[months(BN_SAKA_MONTHS)],
        &["saka"],
        &["সাল"],
    ),
    dated(
        &[CalendarId("hindu-solar-bengali"), CalendarId("bangladeshi")],
        &[months(BN_BANGABDA_MONTHS)],
        &[],
        &[],
    ),
];

const BN: LocaleData = LocaleData {
    tag: "bn",
    direction: Direction::LeftToRight,
    numbering: "beng",
    first_day_of_week: Weekday::Sunday,
    casing: CasingStyle::Standard,
    capitalises_month_names: false,
    weekdays: ContextualNames::same(weekday_widths(
        &[
            "সোমবার",
            "মঙ্গলবার",
            "বুধবার",
            "বৃহস্পতিবার",
            "শুক্রবার",
            "শনিবার",
            "রবিবার",
        ],
        &["সোম", "মঙ্গল", "বুধ", "বৃহস্পতি", "শুক্র", "শনি", "রবি"],
        &[],
        &["সো", "ম", "বু", "বৃ", "শু", "শ", "র"],
    )),
    day_periods: ContextualNames::EMPTY,
    cycle: SexagenaryNames::EMPTY,
    calendars: BN_CALENDARS,
};

// --- Tibetan --------------------------------------------------------------
//
// The language of the Tibetan calendar. Gregorian vocabulary from CLDR 48
// `common/main/bo.xml`, `calendar type="gregorian"`: CLDR names the
// Gregorian months by number — ཟླ་བ་དང་པོ, "first month" — and writes the
// stand-alone form with a closing tsheg, which is kept, as are the tshegs
// that end the weekday, day-period and era names. Only `eraAbbr` is
// stated, and CLDR's `eraNames` alias to it, so the abbreviations stand as
// the wide names too. The week begins on Sunday (CLDR 48 `weekData`, CN
// and IN alike) and the default numbering system inherits root's `latn`.
//
// The Tibetan calendar numbers its months (Janson, "Tibetan calendar
// mathematics", 2014, Section 5, which `hc_calendars_lunar::tibetan`
// follows), so the CLDR ordinal month names are keyed to it as English's
// "First Month" is keyed to the numbered lunisolar calendars — the same
// twelve words for the same twelve numbers, and not a translation. CLDR
// has no `tibetan` calendar and no word for the doubled month, so the
// leap-month prefix is empty. The sixty-year names are not here: the
// calendar module carries them in English, and this crate's cycle model
// names stems and branches, not elements.

const BO_MONTHS: &[&str] = &[
    "ཟླ་བ་དང་པོ",
    "ཟླ་བ་གཉིས་པ",
    "ཟླ་བ་གསུམ་པ",
    "ཟླ་བ་བཞི་པ",
    "ཟླ་བ་ལྔ་པ",
    "ཟླ་བ་དྲུག་པ",
    "ཟླ་བ་བདུན་པ",
    "ཟླ་བ་བརྒྱད་པ",
    "ཟླ་བ་དགུ་པ",
    "ཟླ་བ་བཅུ་པ",
    "ཟླ་བ་བཅུ་གཅིག་པ",
    "ཟླ་བ་བཅུ་གཉིས་པ",
];

const BO_MONTHS_STANDALONE: &[&str] = &[
    "ཟླ་བ་དང་པོ་",
    "ཟླ་བ་གཉིས་པ་",
    "ཟླ་བ་གསུམ་པ་",
    "ཟླ་བ་བཞི་པ་",
    "ཟླ་བ་ལྔ་པ་",
    "ཟླ་བ་དྲུག་པ་",
    "ཟླ་བ་བདུན་པ་",
    "ཟླ་བ་བརྒྱད་པ་",
    "ཟླ་བ་དགུ་པ་",
    "ཟླ་བ་བཅུ་པ་",
    "ཟླ་བ་བཅུ་གཅིག་པ་",
    "ཟླ་བ་བཅུ་གཉིས་པ་",
];

const BO_MONTHS_ABBREVIATED: &[&str] = &[
    "ཟླ་༡",
    "ཟླ་༢",
    "ཟླ་༣",
    "ཟླ་༤",
    "ཟླ་༥",
    "ཟླ་༦",
    "ཟླ་༧",
    "ཟླ་༨",
    "ཟླ་༩",
    "ཟླ་༡༠",
    "ཟླ་༡༡",
    "ཟླ་༡༢",
];

const BO_CALENDARS: &[CalendarNames] = &[
    gregorian(
        &[month_cycle(ContextualNames {
            format: widths(BO_MONTHS, BO_MONTHS_ABBREVIATED, &[]),
            standalone: widths(BO_MONTHS_STANDALONE, &[], &[]),
        })],
        gregorian_eras(&["སྤྱི་ལོ་སྔོན་", "སྤྱི་ལོ་"], &[], &[]),
        ContextualNames::same(widths(
            &[
                "དུས་ཚིགས་དང་པོ།",
                "དུས་ཚིགས་གཉིས་པ།",
                "དུས་ཚིགས་གསུམ་པ།",
                "དུས་ཚིགས་བཞི་པ།",
            ],
            &[],
            &[],
        )),
    ),
    lunisolar(
        &[CalendarId("tibetan")],
        &[month_cycle(ContextualNames {
            format: widths(BO_MONTHS, BO_MONTHS_ABBREVIATED, &[]),
            standalone: widths(BO_MONTHS_STANDALONE, &[], &[]),
        })],
        "",
    ),
];

const BO: LocaleData = LocaleData {
    tag: "bo",
    direction: Direction::LeftToRight,
    numbering: "latn",
    first_day_of_week: Weekday::Sunday,
    casing: CasingStyle::Standard,
    capitalises_month_names: false,
    weekdays: ContextualNames::same(weekday_widths(
        &[
            "གཟའ་ཟླ་བ་",
            "གཟའ་མིག་དམར་",
            "གཟའ་ལྷག་པ་",
            "གཟའ་ཕུར་བུ་",
            "གཟའ་པ་སངས་",
            "གཟའ་སྤེན་པ་",
            "གཟའ་ཉི་མ་",
        ],
        &[
            "ཟླ་བ་",
            "མིག་དམར་",
            "ལྷག་པ་",
            "ཕུར་བུ་",
            "པ་སངས་",
            "སྤེན་པ་",
            "ཉི་མ་",
        ],
        &[],
        &["ཟླ", "མིག", "ལྷག", "ཕུར", "སངས", "སྤེན", "ཉི"],
    )),
    day_periods: ContextualNames::same(widths(&["སྔ་དྲོ་", "ཕྱི་དྲོ་"], &[], &[])),
    cycle: SexagenaryNames::EMPTY,
    calendars: BO_CALENDARS,
};

// --- Coptic ---------------------------------------------------------------
//
// The language of the Coptic calendar. CLDR has no `cop` locale, so this
// entry has no Gregorian vocabulary — the empty Gregorian entry below is
// there because every locale states that calendar, and it inherits — and no
// weekdays: the only Coptic weekday name found with a source was Monday
// (ⲡⲓⲥⲛⲁⲩ, Wiktionary's category of Coptic days of the week, read
// 2026-09-25), which is not a week. The thirteen months are the Bohairic
// column of the months table of Wikipedia, "Coptic calendar", read
// 2026-09-25, which prints Bohairic and Sahidic side by side and cites
// Černý, Coptic Etymological Dictionary (1976) and Vycichl, Dictionnaire
// étymologique de la langue copte (1983) for the names' origins, neither
// read here. Bohairic is the dialect of the Coptic Orthodox liturgy, and
// these are the same forms `hc_calendars_solar::coptic` declares with its
// shape; the locale states them so that the language, and not only the
// calendar, claims them. The Sahidic forms (Ⲑⲟⲟⲩⲧ, Ⲡⲁⲱⲡⲉ …) are not
// carried. No source read gives the era's name in Coptic script. The
// week begins on Saturday because CLDR 48 `weekData/firstDay` says so for
// EG, the only region the language is spoken in, and there is no CLDR
// locale to say otherwise; numbering is `latn` because this crate has no
// Coptic numerals.

const COP: LocaleData = LocaleData {
    tag: "cop",
    direction: Direction::LeftToRight,
    numbering: "latn",
    first_day_of_week: Weekday::Saturday,
    casing: CasingStyle::Standard,
    capitalises_month_names: true,
    weekdays: ContextualNames::EMPTY,
    day_periods: ContextualNames::EMPTY,
    cycle: SexagenaryNames::EMPTY,
    calendars: &[
        gregorian(&[], EraNames::EMPTY, ContextualNames::EMPTY),
        dated(
            COPTIC_CALENDARS,
            &[months(&[
                "Ⲑⲱⲟⲩⲧ",
                "Ⲡⲁⲟⲡⲓ",
                "Ⲁⲑⲱⲣ",
                "Ⲭⲟⲓⲁⲕ",
                "Ⲧⲱⲃⲓ",
                "Ⲙⲉϣⲓⲣ",
                "Ⲡⲁⲣⲉⲙϩⲁⲧ",
                "Ⲫⲁⲣⲙⲟⲩⲑⲓ",
                "Ⲡⲁϣⲟⲛⲥ",
                "Ⲡⲁⲱⲛⲓ",
                "Ⲉⲡⲓⲡ",
                "Ⲙⲉⲥⲱⲣⲓ",
                "Ⲡⲓⲕⲟⲩϫⲓ ⲛ̀ⲁ̀ⲃⲟⲧ",
            ])],
            &[],
            &[],
        ),
    ],
};

// --- Czech ----------------------------------------------------------------
//
// Czech is here for the same reason Russian is: its months take a genitive
// inside a date (1. ledna) and a nominative on their own (leden).

const CS: LocaleData = LocaleData {
    tag: "cs",
    direction: Direction::LeftToRight,
    numbering: "latn",
    first_day_of_week: Weekday::Monday,
    casing: CasingStyle::Standard,
    capitalises_month_names: false,
    weekdays: ContextualNames::same(weekday_widths(
        &[
            "pondělí",
            "úterý",
            "středa",
            "čtvrtek",
            "pátek",
            "sobota",
            "neděle",
        ],
        &["po", "út", "st", "čt", "pá", "so", "ne"],
        &[],
        &["P", "Ú", "S", "Č", "P", "S", "N"],
    )),
    day_periods: ContextualNames::same(widths(&["dop.", "odp."], &[], &[])),
    cycle: SexagenaryNames::EMPTY,
    calendars: &[gregorian(
        &[month_cycle(ContextualNames {
            format: widths(
                &[
                    "ledna",
                    "února",
                    "března",
                    "dubna",
                    "května",
                    "června",
                    "července",
                    "srpna",
                    "září",
                    "října",
                    "listopadu",
                    "prosince",
                ],
                &[
                    "led", "úno", "bře", "dub", "kvě", "čvn", "čvc", "srp", "zář", "říj", "lis",
                    "pro",
                ],
                &[],
            ),
            standalone: widths(
                &[
                    "leden",
                    "únor",
                    "březen",
                    "duben",
                    "květen",
                    "červen",
                    "červenec",
                    "srpen",
                    "září",
                    "říjen",
                    "listopad",
                    "prosinec",
                ],
                &[],
                &[],
            ),
        })],
        gregorian_eras(
            &["před naším letopočtem", "našeho letopočtu"],
            &["př. n. l.", "n. l."],
            &[],
        ),
        ContextualNames::EMPTY,
    )],
};

// --- German ---------------------------------------------------------------

const DE: LocaleData = LocaleData {
    tag: "de",
    direction: Direction::LeftToRight,
    numbering: "latn",
    first_day_of_week: Weekday::Monday,
    casing: CasingStyle::Standard,
    // German month names are nouns, so they are capitalised everywhere.
    capitalises_month_names: true,
    weekdays: ContextualNames::same(weekday_widths(
        &[
            "Montag",
            "Dienstag",
            "Mittwoch",
            "Donnerstag",
            "Freitag",
            "Samstag",
            "Sonntag",
        ],
        &["Mo.", "Di.", "Mi.", "Do.", "Fr.", "Sa.", "So."],
        &["Mo", "Di", "Mi", "Do", "Fr", "Sa", "So"],
        &["M", "D", "M", "D", "F", "S", "S"],
    )),
    day_periods: ContextualNames::same(widths(&["AM", "PM"], &[], &[])),
    cycle: SexagenaryNames::EMPTY,
    calendars: &[gregorian(
        &[month_cycle(ContextualNames::same(widths(
            &[
                "Januar",
                "Februar",
                "März",
                "April",
                "Mai",
                "Juni",
                "Juli",
                "August",
                "September",
                "Oktober",
                "November",
                "Dezember",
            ],
            &[
                "Jan.", "Feb.", "März", "Apr.", "Mai", "Juni", "Juli", "Aug.", "Sept.", "Okt.",
                "Nov.", "Dez.",
            ],
            &["J", "F", "M", "A", "M", "J", "J", "A", "S", "O", "N", "D"],
        )))],
        gregorian_eras(&["v. Chr.", "n. Chr."], &[], &["v", "n"]),
        ContextualNames::same(widths(
            &["1. Quartal", "2. Quartal", "3. Quartal", "4. Quartal"],
            &["Q1", "Q2", "Q3", "Q4"],
            &[],
        )),
    )],
};

// --- English --------------------------------------------------------------

const EN_CALENDARS: &[CalendarNames] = &[
    gregorian(
        &[month_cycle(ContextualNames::same(widths(
            &[
                "January",
                "February",
                "March",
                "April",
                "May",
                "June",
                "July",
                "August",
                "September",
                "October",
                "November",
                "December",
            ],
            &[
                "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
            ],
            &["J", "F", "M", "A", "M", "J", "J", "A", "S", "O", "N", "D"],
        )))],
        gregorian_eras(
            &["Before Christ", "Anno Domini"],
            &["BC", "AD"],
            &["B", "A"],
        ),
        ContextualNames::same(widths(
            &["1st quarter", "2nd quarter", "3rd quarter", "4th quarter"],
            &["Q1", "Q2", "Q3", "Q4"],
            &[],
        )),
    ),
    CalendarNames {
        calendars: JAPANESE_CALENDARS,
        cycles: &[],
        leap_month_prefix: "",
        eras: EraNames {
            codes: JAPANESE_ERA_CODES,
            names: widths(
                &["Meiji", "Taisho", "Showa", "Heisei", "Reiwa"],
                &[],
                JAPANESE_ERA_NARROW,
            ),
            calendars: &[],
        },
        quarters: ContextualNames::EMPTY,
    },
    dated(
        ISLAMIC_CALENDARS,
        &[months(&[
            "Muharram",
            "Safar",
            "Rabi I",
            "Rabi II",
            "Jumada I",
            "Jumada II",
            "Rajab",
            "Sha'ban",
            "Ramadan",
            "Shawwal",
            "Dhu al-Qi'dah",
            "Dhu al-Hijjah",
        ])],
        &["ah"],
        &["AH"],
    ),
    dated(
        HEBREW_CALENDARS,
        &[months(&[
            "Tishri", "Heshvan", "Kislev", "Tevet", "Shevat", "Adar I", "Adar", "Nisan", "Iyar",
            "Sivan", "Tamuz", "Av", "Elul",
        ])],
        &["am"],
        &["AM"],
    ),
    // CLDR has no Babylonian vocabulary. The months are the Akkadian names
    // in the normalisation R. H. van Gent's converter of Parker and
    // Dubberstein's tables prints (webspace.science.uu.nl/~gent0113/babylon/,
    // read 2026-09-25), the sixth spelt as it spells the intercalary one; an
    // intercalary month is "second Ulūlu" or "second Addāru", the converter's
    // "Ulūlu II" and "Addāru II" put into English word order.
    CalendarNames {
        calendars: BABYLONIAN_CALENDARS,
        cycles: &[months(&[
            "Nīsannu",
            "Ayyāru",
            "Sīmannu",
            "Duʾūzu",
            "Ābu",
            "Ulūlu",
            "Tašrītu",
            "Araḫsamna",
            "Kisilīmu",
            "Ṭebētu",
            "Šabāṭu",
            "Addāru",
        ])],
        leap_month_prefix: "second ",
        eras: EraNames {
            codes: &["se"],
            names: widths(&["SE"], &[], &[]),
            calendars: &[],
        },
        quarters: ContextualNames::EMPTY,
    },
    lunisolar(
        NUMBERED_LUNISOLAR_CALENDARS,
        &[months(&[
            "First Month",
            "Second Month",
            "Third Month",
            "Fourth Month",
            "Fifth Month",
            "Sixth Month",
            "Seventh Month",
            "Eighth Month",
            "Ninth Month",
            "Tenth Month",
            "Eleventh Month",
            "Twelfth Month",
        ])],
        "leap ",
    ),
    // Four calendars whose own names are in another script, romanised the
    // way English-language sources print them. Each is an override of the
    // names the calendar declares for itself: Coptic-derived rather than
    // Arabic-derived for the Coptic months; the Encyclopaedia Aethiopica
    // transliteration for the Ethiopic; Hübschmann-Meillet-Benveniste for
    // the Armenian; the usual romanisation for the Persian.
    dated(
        COPTIC_CALENDARS,
        &[months(&[
            "Thout",
            "Paopi",
            "Hathor",
            "Koiak",
            "Tobi",
            "Meshir",
            "Paremhat",
            "Parmouti",
            "Pashons",
            "Paoni",
            "Epip",
            "Mesori",
            "Pi Kogi Enavot",
        ])],
        &[],
        &[],
    ),
    dated(
        ETHIOPIC_CALENDARS,
        &[months(&[
            "Mäskäräm",
            "Ṭəqəmt",
            "Ḫədar",
            "Taḫśaś",
            "Ṭərr",
            "Yäkatit",
            "Mägabit",
            "Miyazya",
            "Gənbot",
            "Säne",
            "Ḥamle",
            "Nähase",
            "Ṗagumen",
        ])],
        &[],
        &[],
    ),
    dated(
        BURMESE_CALENDARS,
        &[months(&[
            "Tagu",
            "Kason",
            "Nayon",
            "Waso",
            "Wagaung",
            "Tawthalin",
            "Thadingyut",
            "Tazaungmon",
            "Nadaw",
            "Pyatho",
            "Tabodwe",
            "Tabaung",
        ])],
        &[],
        &[],
    ),
    dated(
        RUMI_CALENDARS,
        &[months(&[
            "Kânûn-ı Sânî",
            "Şubat",
            "Mart",
            "Nisan",
            "Mayıs",
            "Haziran",
            "Temmuz",
            "Ağustos",
            "Eylül",
            "Teşrin-i Evvel",
            "Teşrin-i Sânî",
            "Kânûn-ı Evvel",
        ])],
        &[],
        &[],
    ),
    dated(
        NANAKSHAHI_CALENDARS,
        &[months(&[
            "Chet", "Vaisakh", "Jeth", "Harh", "Sawan", "Bhadon", "Assu", "Kattak", "Maghar",
            "Poh", "Magh", "Phaggan",
        ])],
        &[],
        &[],
    ),
    // The months as Wikipedia's "Bangladeshi national calendar" links
    // them; `hc_calendars_solar::bangladeshi` has the Bengali script.
    dated(
        &[CalendarId("bangladeshi")],
        &[months(&[
            "Boishakh",
            "Joishtho",
            "Asharh",
            "Srabon",
            "Bhadro",
            "Ashvin",
            "Kartik",
            "Ogrohayon",
            "Poush",
            "Magh",
            "Falgun",
            "Choitro",
        ])],
        &["bangabda"],
        &["Bangabda"],
    ),
    dated(
        ARMENIAN_CALENDARS,
        &[months(&[
            "Nawasard",
            "Hoṙi",
            "Sahmi",
            "Trē",
            "Kʿałocʿ",
            "Aracʿ",
            "Mehekan",
            "Areg",
            "Ahekan",
            "Mareri",
            "Margacʿ",
            "Hroticʿ",
            "Aweleacʿ",
        ])],
        &[],
        &[],
    ),
    dated(
        PERSIAN_CALENDARS,
        &[months(&[
            "Farvardin",
            "Ordibehesht",
            "Khordad",
            "Tir",
            "Mordad",
            "Shahrivar",
            "Mehr",
            "Aban",
            "Azar",
            "Dey",
            "Bahman",
            "Esfand",
        ])],
        &[],
        &[],
    ),
    // Badíʿ has nineteen months, which the old twelve-or-thirteen assertion
    // rejected; it is ordinary data now, checked against the length the
    // calendar declares. Its month names are the Bahá'í transliteration
    // English-language texts use, which is why they are a locale's and not
    // the calendar's own. (The French Republican months and décade days,
    // which once sat beside these, are the calendar's own French and are
    // declared with its shape in `hc-calendars-solar`.)
    // The Hindu lunisolar months, Chaitra first, in the English
    // transliteration the Rashtriya Panchang's English edition uses; the
    // calendar's own names are Devanagari, declared in `hc-calendars-indic`.
    // An intercalary month is "Adhika Śrāvaṇa", so the prefix is "Adhika ".
    CalendarNames {
        calendars: &[
            CalendarId("hindu-lunar"),
            CalendarId("hindu-lunar-purnimanta"),
        ],
        cycles: &[months(&[
            "Chaitra",
            "Vaisakha",
            "Jyaishtha",
            "Ashadha",
            "Sravana",
            "Bhadra",
            "Asvina",
            "Kartika",
            "Agrahayana",
            "Pausha",
            "Magha",
            "Phalguna",
        ])],
        leap_month_prefix: "Adhika ",
        eras: EraNames {
            codes: &["saka"],
            names: widths(&["Saka"], &[], &[]),
            calendars: &[],
        },
        quarters: ContextualNames::EMPTY,
    },
    // The Old Hindu lunisolar calendar's months are those same twelve, and
    // its intercalary month is likewise "Adhika X"; it declares none of its
    // own, because the mean-motion calendar is arithmetic and its months are
    // named for the solar month that begins within them. The Old Hindu solar
    // calendar is not here: its months are the sidereal signs, and it
    // declares their Sanskrit names with its shape. Both count in the Kali
    // Yuga, so the era serves the two of them.
    CalendarNames {
        calendars: &[CalendarId("hindu-old-lunar")],
        cycles: &[months(&[
            "Chaitra",
            "Vaisakha",
            "Jyaishtha",
            "Ashadha",
            "Sravana",
            "Bhadra",
            "Asvina",
            "Kartika",
            "Agrahayana",
            "Pausha",
            "Magha",
            "Phalguna",
        ])],
        leap_month_prefix: "Adhika ",
        eras: EraNames {
            codes: &["kali-yuga"],
            names: widths(&["Kali Yuga"], &[], &[]),
            calendars: &[CalendarId("hindu-old-lunar"), CalendarId("hindu-old-solar")],
        },
        quarters: ContextualNames::EMPTY,
    },
    // Nepal Sambat's months are the amānta months under their Newar names,
    // Kachhalā first, as Wikipedia's "Nepal Sambat" romanizes them; the
    // Devanagari and Newa-script names are the calendar's own, in
    // `hc-calendars-indic`. The tradition calls an intercalary month Analā,
    // and the source does not say how it is written beside the month it
    // doubles, so no prefix is supplied rather than one invented.
    CalendarNames {
        calendars: &[CalendarId("nepal-sambat")],
        cycles: &[months(&[
            "Kachhalā",
            "Thinlā",
            "Pwanhelā",
            "Silā",
            "Chilā",
            "Chaulā",
            "Bachhalā",
            "Tachhalā",
            "Dilā",
            "Gunlā",
            "Yanlā",
            "Kaulā",
        ])],
        leap_month_prefix: "",
        eras: EraNames {
            codes: &["nepal-sambat"],
            names: widths(&["Nepal Sambat"], &["NS"], &[]),
            calendars: &[],
        },
        quarters: ContextualNames::EMPTY,
    },
    CalendarNames {
        calendars: &[CalendarId("bikram-sambat")],
        // The Nepali forms Wikipedia's "Vikram Samvat" lists beside the
        // Sanskrit names; `hc_calendars_indic::bikram_sambat` has the
        // gazette's Devanagari.
        cycles: &[months(&[
            "Baisakh", "Jeth", "Asar", "Saaun", "Bhadau", "Aasoj", "Kattik", "Mangsir", "Push",
            "Maagh", "Falgun", "Chait",
        ])],
        leap_month_prefix: "",
        eras: EraNames {
            codes: &["bikram-sambat"],
            names: widths(&["Bikram Sambat"], &["BS"], &[]),
            calendars: &[],
        },
        quarters: ContextualNames::EMPTY,
    },
    CalendarNames {
        calendars: &[
            CalendarId("bahai-arithmetic"),
            CalendarId("bahai"),
            CalendarId("bahai-astronomical"),
        ],
        cycles: &[months(&[
            "Bahá",
            "Jalál",
            "Jamál",
            "ʻAẓamat",
            "Núr",
            "Raḥmat",
            "Kalimát",
            "Kamál",
            "Asmáʼ",
            "ʻIzzat",
            "Mashíyyat",
            "ʻIlm",
            "Qudrat",
            "Qawl",
            "Masáʼil",
            "Sharaf",
            "Sulṭán",
            "Mulk",
            "ʻAláʼ",
        ])],
        leap_month_prefix: "",
        eras: EraNames {
            codes: &["be"],
            names: widths(&["BE"], &[], &[]),
            calendars: &[],
        },
        quarters: ContextualNames::EMPTY,
    },
];

const EN: LocaleData = LocaleData {
    tag: "en",
    direction: Direction::LeftToRight,
    numbering: "latn",
    first_day_of_week: Weekday::Monday,
    casing: CasingStyle::Standard,
    capitalises_month_names: true,
    weekdays: ContextualNames::same(weekday_widths(
        &[
            "Monday",
            "Tuesday",
            "Wednesday",
            "Thursday",
            "Friday",
            "Saturday",
            "Sunday",
        ],
        &["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"],
        &["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"],
        &["M", "T", "W", "T", "F", "S", "S"],
    )),
    day_periods: ContextualNames::same(widths(&["AM", "PM"], &[], &["a", "p"])),
    cycle: SexagenaryNames {
        reading: Some(&readings::PINYIN),
        zodiac: Some(&[
            "Rat", "Ox", "Tiger", "Rabbit", "Dragon", "Snake", "Horse", "Goat", "Monkey",
            "Rooster", "Dog", "Pig",
        ]),
    },
    calendars: EN_CALENDARS,
};

// --- Spanish --------------------------------------------------------------

const ES: LocaleData = LocaleData {
    tag: "es",
    direction: Direction::LeftToRight,
    numbering: "latn",
    first_day_of_week: Weekday::Monday,
    casing: CasingStyle::Standard,
    capitalises_month_names: false,
    weekdays: ContextualNames::same(weekday_widths(
        &[
            "lunes",
            "martes",
            "miércoles",
            "jueves",
            "viernes",
            "sábado",
            "domingo",
        ],
        &["lun", "mar", "mié", "jue", "vie", "sáb", "dom"],
        &["LU", "MA", "MI", "JU", "VI", "SA", "DO"],
        &["L", "M", "X", "J", "V", "S", "D"],
    )),
    day_periods: ContextualNames::same(widths(&["a. m.", "p. m."], &[], &[])),
    cycle: SexagenaryNames::EMPTY,
    calendars: &[gregorian(
        &[month_cycle(ContextualNames::same(widths(
            &[
                "enero",
                "febrero",
                "marzo",
                "abril",
                "mayo",
                "junio",
                "julio",
                "agosto",
                "septiembre",
                "octubre",
                "noviembre",
                "diciembre",
            ],
            &[
                "ene", "feb", "mar", "abr", "may", "jun", "jul", "ago", "sept", "oct", "nov", "dic",
            ],
            &["E", "F", "M", "A", "M", "J", "J", "A", "S", "O", "N", "D"],
        )))],
        gregorian_eras(
            &["antes de Cristo", "después de Cristo"],
            &["a. C.", "d. C."],
            &[],
        ),
        ContextualNames::same(widths(
            &[
                "1.º trimestre",
                "2.º trimestre",
                "3.º trimestre",
                "4.º trimestre",
            ],
            &["T1", "T2", "T3", "T4"],
            &[],
        )),
    )],
};

// --- Persian --------------------------------------------------------------

const FA: LocaleData = LocaleData {
    tag: "fa",
    direction: Direction::RightToLeft,
    numbering: "arabext",
    first_day_of_week: Weekday::Saturday,
    casing: CasingStyle::Standard,
    capitalises_month_names: false,
    weekdays: ContextualNames::same(weekday_widths(
        &[
            "دوشنبه",
            "سه‌شنبه",
            "چهارشنبه",
            "پنجشنبه",
            "جمعه",
            "شنبه",
            "یکشنبه",
        ],
        &[],
        &[],
        &["د", "س", "چ", "پ", "ج", "ش", "ی"],
    )),
    day_periods: ContextualNames::same(widths(&["قبل‌ازظهر", "بعدازظهر"], &[], &[])),
    cycle: SexagenaryNames::EMPTY,
    calendars: &[
        gregorian(
            &[month_cycle(ContextualNames::same(widths(
                &[
                    "ژانویه",
                    "فوریه",
                    "مارس",
                    "آوریل",
                    "مه",
                    "ژوئن",
                    "ژوئیه",
                    "اوت",
                    "سپتامبر",
                    "اکتبر",
                    "نوامبر",
                    "دسامبر",
                ],
                &[],
                &[],
            )))],
            gregorian_eras(&["قبل از میلاد", "میلادی"], &["ق.م.", "م."], &[]),
            ContextualNames::EMPTY,
        ),
        // The month names are the calendar's own, declared with its shape in
        // `hc-calendars-solar`; only the era is a Persian word.
        dated(PERSIAN_CALENDARS, &[], &["ap"], &["ه.ش."]),
    ],
};

// --- French ---------------------------------------------------------------

const FR: LocaleData = LocaleData {
    tag: "fr",
    direction: Direction::LeftToRight,
    numbering: "latn",
    first_day_of_week: Weekday::Monday,
    casing: CasingStyle::Standard,
    capitalises_month_names: false,
    weekdays: ContextualNames::same(weekday_widths(
        &[
            "lundi", "mardi", "mercredi", "jeudi", "vendredi", "samedi", "dimanche",
        ],
        &["lun.", "mar.", "mer.", "jeu.", "ven.", "sam.", "dim."],
        &["lu", "ma", "me", "je", "ve", "sa", "di"],
        &["L", "M", "M", "J", "V", "S", "D"],
    )),
    day_periods: ContextualNames::same(widths(&["AM", "PM"], &[], &[])),
    cycle: SexagenaryNames::EMPTY,
    calendars: &[gregorian(
        &[month_cycle(ContextualNames::same(widths(
            &[
                "janvier",
                "février",
                "mars",
                "avril",
                "mai",
                "juin",
                "juillet",
                "août",
                "septembre",
                "octobre",
                "novembre",
                "décembre",
            ],
            &[
                "janv.", "févr.", "mars", "avr.", "mai", "juin", "juil.", "août", "sept.", "oct.",
                "nov.", "déc.",
            ],
            &["J", "F", "M", "A", "M", "J", "J", "A", "S", "O", "N", "D"],
        )))],
        gregorian_eras(
            &["avant Jésus-Christ", "après Jésus-Christ"],
            &["av. J.-C.", "ap. J.-C."],
            &[],
        ),
        ContextualNames::same(widths(
            &[
                "1er trimestre",
                "2e trimestre",
                "3e trimestre",
                "4e trimestre",
            ],
            &["T1", "T2", "T3", "T4"],
            &[],
        )),
    )],
};

// --- Hebrew ---------------------------------------------------------------

const HE_CALENDARS: &[CalendarNames] = &[
    gregorian(
        &[month_cycle(ContextualNames::same(widths(
            &[
                "ינואר",
                "פברואר",
                "מרץ",
                "אפריל",
                "מאי",
                "יוני",
                "יולי",
                "אוגוסט",
                "ספטמבר",
                "אוקטובר",
                "נובמבר",
                "דצמבר",
            ],
            &[],
            &[],
        )))],
        gregorian_eras(&["לפני הספירה", "לספירה"], &[], &[]),
        ContextualNames::EMPTY,
    ),
    dated(
        HEBREW_CALENDARS,
        &[months(&[
            "תשרי",
            "חשוון",
            "כסלו",
            "טבת",
            "שבט",
            "אדר א׳",
            "אדר",
            "ניסן",
            "אייר",
            "סיוון",
            "תמוז",
            "אב",
            "אלול",
        ])],
        &["am"],
        &["לבריאת העולם"],
    ),
];

const HE: LocaleData = LocaleData {
    tag: "he",
    direction: Direction::RightToLeft,
    numbering: "latn",
    first_day_of_week: Weekday::Sunday,
    casing: CasingStyle::Standard,
    capitalises_month_names: false,
    weekdays: ContextualNames::same(weekday_widths(
        &[
            "יום שני",
            "יום שלישי",
            "יום רביעי",
            "יום חמישי",
            "יום שישי",
            "יום שבת",
            "יום ראשון",
        ],
        &[
            "יום ב׳",
            "יום ג׳",
            "יום ד׳",
            "יום ה׳",
            "יום ו׳",
            "שבת",
            "יום א׳",
        ],
        &[],
        &["ב׳", "ג׳", "ד׳", "ה׳", "ו׳", "ש׳", "א׳"],
    )),
    day_periods: ContextualNames::same(widths(&["לפנה״צ", "אחה״צ"], &[], &[])),
    cycle: SexagenaryNames::EMPTY,
    calendars: HE_CALENDARS,
};

// --- Hindi ----------------------------------------------------------------
//
// Gregorian vocabulary from CLDR 48 `common/main/hi.xml`. The twelve months
// of its `calendar type="indian"`, Chaitra first, are keyed to the national
// calendar with CLDR's era abbreviation शक. The national calendar's months
// bear the lunar months' names (docs/systems/hindu-calendars.md, after the
// Calendar Reform Committee's report of 1955), so the same twelve Hindi
// forms are keyed to the amānta and pūrṇimānta calendars too, where the
// calendar's own Devanagari — आषाढ, आश्विन and मार्गशीर्ष, from the Rashtriya
// Panchang's Sanskrit edition — has the Hindi spellings आषाढ़, अश्विन and
// अग्रहायण as CLDR writes them; the locale's forms are the override. The
// intercalary month is adhika: the prefix अधिक is the first element of
// अधिकमास as Wikipedia, "Adhik Maas", read 2026-09-26, spells the Sanskrit,
// kept apart from the month name as the English "Adhika " is. The Vikrami
// solar months of Punjab, Haryana and Odisha are the same names from
// Vaiśākha (`hc_seasons::zodiac::rashi::VIKRAMI`, after the Rashtriya
// Panchang's Punjab and Odisha column), so the list is keyed to
// `hindu-solar-vikrami` starting one month on. Not carried: the rāśi names
// in Devanagari, which no source read prints (the Old Hindu solar calendar
// keeps its IAST names), and the twenty-seven nakṣatras, which no calendar
// declares as a cycle.

const HI_SAKA_MONTHS: &[&str] = &[
    "चैत्र",
    "वैशाख",
    "ज्येष्ठ",
    "आषाढ़",
    "श्रावण",
    "भाद्रपद",
    "अश्विन",
    "कार्तिक",
    "अग्रहायण",
    "पौष",
    "माघ",
    "फाल्गुन",
];

const HI_VIKRAMI_MONTHS: &[&str] = &[
    "वैशाख",
    "ज्येष्ठ",
    "आषाढ़",
    "श्रावण",
    "भाद्रपद",
    "अश्विन",
    "कार्तिक",
    "अग्रहायण",
    "पौष",
    "माघ",
    "फाल्गुन",
    "चैत्र",
];

const HI_CALENDARS: &[CalendarNames] = &[
    gregorian(
        &[month_cycle(ContextualNames::same(widths(
            &[
                "जनवरी",
                "फ़रवरी",
                "मार्च",
                "अप्रैल",
                "मई",
                "जून",
                "जुलाई",
                "अगस्त",
                "सितंबर",
                "अक्तूबर",
                "नवंबर",
                "दिसंबर",
            ],
            &[
                "जन",
                "फ़र",
                "मार्च",
                "अप्रैल",
                "मई",
                "जून",
                "जुल",
                "अग",
                "सित",
                "अक्तू",
                "नव",
                "दिस",
            ],
            &[],
        )))],
        gregorian_eras(&["ईसा-पूर्व", "ईसवी"], &[], &[]),
        ContextualNames::EMPTY,
    ),
    dated(
        &[CalendarId("indian")],
        &[months(HI_SAKA_MONTHS)],
        &["saka"],
        &["शक"],
    ),
    lunisolar(
        &[
            CalendarId("hindu-lunar"),
            CalendarId("hindu-lunar-purnimanta"),
        ],
        &[months(HI_SAKA_MONTHS)],
        "अधिक ",
    ),
    dated(
        &[CalendarId("hindu-solar-vikrami")],
        &[months(HI_VIKRAMI_MONTHS)],
        &[],
        &[],
    ),
];

const HI: LocaleData = LocaleData {
    tag: "hi",
    direction: Direction::LeftToRight,
    numbering: "latn",
    first_day_of_week: Weekday::Sunday,
    casing: CasingStyle::Standard,
    capitalises_month_names: false,
    weekdays: ContextualNames::same(weekday_widths(
        &[
            "सोमवार",
            "मंगलवार",
            "बुधवार",
            "गुरुवार",
            "शुक्रवार",
            "शनिवार",
            "रविवार",
        ],
        &["सोम", "मंगल", "बुध", "गुरु", "शुक्र", "शनि", "रवि"],
        &[],
        &["सो", "मं", "बु", "गु", "शु", "श", "र"],
    )),
    day_periods: ContextualNames::same(widths(&["पूर्वाह्न", "अपराह्न"], &[], &[])),
    cycle: SexagenaryNames::EMPTY,
    calendars: HI_CALENDARS,
};

// --- Indonesian -----------------------------------------------------------

const ID: LocaleData = LocaleData {
    tag: "id",
    direction: Direction::LeftToRight,
    numbering: "latn",
    first_day_of_week: Weekday::Sunday,
    casing: CasingStyle::Standard,
    capitalises_month_names: true,
    weekdays: ContextualNames::same(weekday_widths(
        &[
            "Senin", "Selasa", "Rabu", "Kamis", "Jumat", "Sabtu", "Minggu",
        ],
        &["Sen", "Sel", "Rab", "Kam", "Jum", "Sab", "Min"],
        &[],
        &["S", "S", "R", "K", "J", "S", "M"],
    )),
    day_periods: ContextualNames::same(widths(&["AM", "PM"], &[], &[])),
    cycle: SexagenaryNames::EMPTY,
    calendars: &[gregorian(
        &[month_cycle(ContextualNames::same(widths(
            &[
                "Januari",
                "Februari",
                "Maret",
                "April",
                "Mei",
                "Juni",
                "Juli",
                "Agustus",
                "September",
                "Oktober",
                "November",
                "Desember",
            ],
            &[
                "Jan", "Feb", "Mar", "Apr", "Mei", "Jun", "Jul", "Agu", "Sep", "Okt", "Nov", "Des",
            ],
            &[],
        )))],
        gregorian_eras(&["Sebelum Masehi", "Masehi"], &["SM", "M"], &[]),
        ContextualNames::EMPTY,
    )],
};

// --- Italian --------------------------------------------------------------

const IT: LocaleData = LocaleData {
    tag: "it",
    direction: Direction::LeftToRight,
    numbering: "latn",
    first_day_of_week: Weekday::Monday,
    casing: CasingStyle::Standard,
    capitalises_month_names: false,
    weekdays: ContextualNames::same(weekday_widths(
        &[
            "lunedì",
            "martedì",
            "mercoledì",
            "giovedì",
            "venerdì",
            "sabato",
            "domenica",
        ],
        &["lun", "mar", "mer", "gio", "ven", "sab", "dom"],
        &[],
        &["L", "M", "M", "G", "V", "S", "D"],
    )),
    day_periods: ContextualNames::same(widths(&["AM", "PM"], &[], &[])),
    cycle: SexagenaryNames::EMPTY,
    calendars: &[gregorian(
        &[month_cycle(ContextualNames::same(widths(
            &[
                "gennaio",
                "febbraio",
                "marzo",
                "aprile",
                "maggio",
                "giugno",
                "luglio",
                "agosto",
                "settembre",
                "ottobre",
                "novembre",
                "dicembre",
            ],
            &[
                "gen", "feb", "mar", "apr", "mag", "giu", "lug", "ago", "set", "ott", "nov", "dic",
            ],
            &["G", "F", "M", "A", "M", "G", "L", "A", "S", "O", "N", "D"],
        )))],
        gregorian_eras(&["avanti Cristo", "dopo Cristo"], &["a.C.", "d.C."], &[]),
        ContextualNames::EMPTY,
    )],
};

// --- Japanese -------------------------------------------------------------

const JA_MONTHS: &[&str] = &[
    "1月", "2月", "3月", "4月", "5月", "6月", "7月", "8月", "9月", "10月", "11月", "12月",
];

const JA_CALENDARS: &[CalendarNames] = &[
    gregorian(
        &[month_cycle(ContextualNames::same(widths(
            JA_MONTHS,
            &[],
            &[
                "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12",
            ],
        )))],
        gregorian_eras(&["紀元前", "西暦"], &[], &[]),
        ContextualNames::same(widths(
            &["第1四半期", "第2四半期", "第3四半期", "第4四半期"],
            &["Q1", "Q2", "Q3", "Q4"],
            &[],
        )),
    ),
    CalendarNames {
        calendars: JAPANESE_CALENDARS,
        cycles: &[],
        leap_month_prefix: "",
        eras: EraNames {
            codes: JAPANESE_ERA_CODES,
            names: widths(
                &["明治", "大正", "昭和", "平成", "令和"],
                &[],
                JAPANESE_ERA_NARROW,
            ),
            calendars: &[],
        },
        quarters: ContextualNames::EMPTY,
    },
    // The traditional lunisolar month names, still used for seasonal and
    // literary dates: 師走 is December in feeling, the twelfth lunar month in
    // fact.
    lunisolar(
        NUMBERED_LUNISOLAR_CALENDARS,
        &[months(&[
            "睦月",
            "如月",
            "弥生",
            "卯月",
            "皐月",
            "水無月",
            "文月",
            "葉月",
            "長月",
            "神無月",
            "霜月",
            "師走",
        ])],
        "閏",
    ),
];

const JA: LocaleData = LocaleData {
    tag: "ja",
    direction: Direction::LeftToRight,
    numbering: "latn",
    first_day_of_week: Weekday::Sunday,
    casing: CasingStyle::Standard,
    capitalises_month_names: false,
    weekdays: ContextualNames::same(weekday_widths(
        &[
            "月曜日",
            "火曜日",
            "水曜日",
            "木曜日",
            "金曜日",
            "土曜日",
            "日曜日",
        ],
        &["月", "火", "水", "木", "金", "土", "日"],
        &[],
        &[],
    )),
    day_periods: ContextualNames::same(widths(&["午前", "午後"], &[], &[])),
    cycle: SexagenaryNames {
        reading: Some(&readings::HAN),
        zodiac: Some(&[
            "鼠", "牛", "虎", "兎", "竜", "蛇", "馬", "羊", "猿", "鶏", "犬", "猪",
        ]),
    },
    calendars: JA_CALENDARS,
};

// --- Javanese -------------------------------------------------------------
//
// The language of the pasaran. Gregorian vocabulary from CLDR 48
// `common/main/jv.xml`: the wide and abbreviated months (May's abbreviation
// resolving to the wide form), the stand-alone narrow ones, the wide and
// abbreviated weekdays (Sunday's abbreviation resolving to the wide form;
// the narrow forms are not carried because Saturday's is the inheritance
// marker and resolves to the wide form), the day periods, the eras and the
// quarters. The five pasaran days are keyed to `javanese-pasaran` as
// `hc_calendars_regional::javanese_pasaran` declares them, Legi first,
// which the "Five-day week" table of Wikipedia, "Javanese calendar", read
// 2026-09-26, prints in the same forms; the seven dina are that page's
// "Seven-day week" table, Ahad (Sunday) first as the module orders them,
// which spells Monday Senin where the module has Senen and gives Minggu
// beside Ahad for Sunday. That page also prints every one of these names
// in Javanese script (ꦊꦒꦶ, ꦱꦼꦤꦶꦤ꧀ …); they are not carried, because `jv` is
// CLDR's Latin-script locale and a `jv-Java` entry would have no Gregorian
// vocabulary of its own to stand beside them. The week begins on Sunday
// (CLDR 48 `weekData/firstDay`, ID) and the numbering system is `latn`.

const JV_CALENDARS: &[CalendarNames] = &[
    gregorian(
        &[month_cycle(ContextualNames::same(widths(
            &[
                "Januari",
                "Februari",
                "Maret",
                "April",
                "Mei",
                "Juni",
                "Juli",
                "Agustus",
                "September",
                "Oktober",
                "November",
                "Desember",
            ],
            &[
                "Jan", "Feb", "Mar", "Apr", "Mei", "Jun", "Jul", "Agt", "Sep", "Okt", "Nov", "Des",
            ],
            &["J", "F", "M", "A", "M", "J", "J", "A", "S", "O", "N", "D"],
        )))],
        gregorian_eras(&["Sakdurunge Masehi", "Masehi"], &["SM", "M"], &[]),
        ContextualNames::same(widths(
            &[
                "triwulan kaping pisan",
                "triwulan kaping loro",
                "triwulan kaping telu",
                "triwulan kaping papat",
            ],
            &["TW1", "TW2", "TW3", "TW4"],
            &[],
        )),
    ),
    CalendarNames {
        calendars: &[CalendarId("javanese-pasaran")],
        cycles: &[
            cycle("pasaran", &["Legi", "Pahing", "Pon", "Wage", "Kliwon"]),
            cycle(
                hc_calendar::shape::WEEKDAY,
                &[
                    "Ahad", "Senin", "Selasa", "Rebo", "Kemis", "Jemuwah", "Setu",
                ],
            ),
        ],
        leap_month_prefix: "",
        eras: EraNames::EMPTY,
        quarters: ContextualNames::EMPTY,
    },
];

const JV: LocaleData = LocaleData {
    tag: "jv",
    direction: Direction::LeftToRight,
    numbering: "latn",
    first_day_of_week: Weekday::Sunday,
    casing: CasingStyle::Standard,
    capitalises_month_names: true,
    weekdays: ContextualNames::same(weekday_widths(
        &["Senin", "Selasa", "Rabu", "Kamis", "Jumat", "Sabtu", "Ahad"],
        &["Sen", "Sel", "Rab", "Kam", "Jum", "Sab", "Ahad"],
        &[],
        &[],
    )),
    day_periods: ContextualNames::same(widths(&["Isuk", "Wengi"], &[], &[])),
    cycle: SexagenaryNames::EMPTY,
    calendars: JV_CALENDARS,
};

// --- Kabyle ---------------------------------------------------------------
//
// A language of the Berber agrarian calendar. Gregorian vocabulary from
// CLDR 48 `common/main/kab.xml`: the wide, abbreviated and stand-alone
// narrow months, the weekdays (one form in every width), the day periods
// (the abbreviated form joins its two words with a narrow no-break space,
// as CLDR writes it), the eras and the quarters in both contexts. The
// agrarian calendar is the Julian year under the Latin-derived names of
// the months: `hc_calendars_solar::berber` declares them in the Kabyle
// forms Wikipedia's "Berber calendar" tabulates, and the Encyclopédie
// berbère's "Calendrier" (1992), which that module cites, says the names
// are the Julian ones — so CLDR's Kabyle Gregorian months are the
// calendar's months in this language, and the same list is keyed to
// `berber`. Two spellings differ from the module's: CLDR writes Fuṛar and
// Nunembeṛ where the module has Furar and Wambeṛ, and the locale's forms
// are the override. The week begins on Saturday (CLDR 48
// `weekData/firstDay`, DZ) and the numbering system is `latn`.

const KAB_MONTHS: &[&str] = &[
    "Yennayer",
    "Fuṛar",
    "Meɣres",
    "Yebrir",
    "Mayyu",
    "Yunyu",
    "Yulyu",
    "Ɣuct",
    "Ctembeṛ",
    "Tubeṛ",
    "Nunembeṛ",
    "Duǧembeṛ",
];

const KAB_MONTHS_ABBREVIATED: &[&str] = &[
    "Yen", "Fur", "Meɣ", "Yeb", "May", "Yun", "Yul", "Ɣuc", "Cte", "Tub", "Nun", "Duǧ",
];

const KAB_MONTHS_NARROW: &[&str] = &["Y", "F", "M", "Y", "M", "Y", "Y", "Ɣ", "C", "T", "N", "D"];

const KAB_CALENDARS: &[CalendarNames] = &[
    gregorian(
        &[month_cycle(ContextualNames::same(widths(
            KAB_MONTHS,
            KAB_MONTHS_ABBREVIATED,
            KAB_MONTHS_NARROW,
        )))],
        gregorian_eras(
            &["send talalit n Ɛisa", "seld talalit n Ɛisa"],
            &["snd. T.Ɛ", "sld. T.Ɛ"],
            &[],
        ),
        ContextualNames {
            format: widths(
                &[
                    "akraḍaggur amenzu",
                    "akraḍaggur wis-sin",
                    "akraḍaggur wis-kraḍ",
                    "akraḍaggur wis-kuẓ",
                ],
                &["Kḍg1", "Kḍg2", "Kḍg3", "Kḍg4"],
                &[],
            ),
            standalone: widths(
                &["akraḍyur 1u", "akraḍyur w2", "akraḍyur w3", "akraḍyur w4"],
                &["Kḍy1", "Kḍy2", "Kḍy3", "Kḍy4"],
                &[],
            ),
        },
    ),
    dated(
        &[CalendarId("berber")],
        &[month_cycle(ContextualNames::same(widths(
            KAB_MONTHS,
            KAB_MONTHS_ABBREVIATED,
            KAB_MONTHS_NARROW,
        )))],
        &[],
        &[],
    ),
];

const KAB: LocaleData = LocaleData {
    tag: "kab",
    direction: Direction::LeftToRight,
    numbering: "latn",
    first_day_of_week: Weekday::Saturday,
    casing: CasingStyle::Standard,
    capitalises_month_names: true,
    weekdays: ContextualNames::same(weekday_widths(
        &["Arim", "Aram", "Ahad", "Amhad", "Sem", "Sed", "Acer"],
        &[],
        &[],
        &[],
    )),
    day_periods: ContextualNames::same(widths(
        &["n tufat", "n tmeddit"],
        &["n\u{202f}tufat", "n\u{202f}tmeddit"],
        &["f", "m"],
    )),
    cycle: SexagenaryNames::EMPTY,
    calendars: KAB_CALENDARS,
};

// --- Korean ---------------------------------------------------------------

const KO: LocaleData = LocaleData {
    tag: "ko",
    direction: Direction::LeftToRight,
    numbering: "latn",
    first_day_of_week: Weekday::Sunday,
    casing: CasingStyle::Standard,
    capitalises_month_names: false,
    weekdays: ContextualNames::same(weekday_widths(
        &[
            "월요일",
            "화요일",
            "수요일",
            "목요일",
            "금요일",
            "토요일",
            "일요일",
        ],
        &["월", "화", "수", "목", "금", "토", "일"],
        &[],
        &[],
    )),
    day_periods: ContextualNames::same(widths(&["오전", "오후"], &[], &[])),
    cycle: SexagenaryNames {
        reading: Some(&readings::HANGUL),
        zodiac: Some(&[
            "쥐",
            "소",
            "호랑이",
            "토끼",
            "용",
            "뱀",
            "말",
            "양",
            "원숭이",
            "닭",
            "개",
            "돼지",
        ]),
    },
    calendars: &[gregorian(
        &[month_cycle(ContextualNames::same(widths(
            &[
                "1월", "2월", "3월", "4월", "5월", "6월", "7월", "8월", "9월", "10월", "11월",
                "12월",
            ],
            &[],
            &[],
        )))],
        gregorian_eras(&["기원전", "서기"], &[], &[]),
        ContextualNames::same(widths(
            &["제1분기", "제2분기", "제3분기", "제4분기"],
            &["1분기", "2분기", "3분기", "4분기"],
            &[],
        )),
    )],
};

// --- Mandaic --------------------------------------------------------------
//
// The language of the Mandaean calendar. CLDR has no `mid` locale, so this
// entry has no Gregorian vocabulary, no day periods and no eras. The
// weekdays are the seven Mandaic forms of the days-of-the-week table of
// Wikipedia, "Mandaean calendar", read 2026-09-26 — Habšaba, Sunday, to
// Yuma ḏ-Šafta, Saturday — Monday first as this crate orders them. The
// months are not carried: that page prints the twelve zodiacal months in
// Mandaic script (ࡃࡀࡅࡋࡀ, ࡍࡅࡍࡀ …, Daula to Gadia) but no Mandaic form of the
// Parwanaia, and `hc_calendars_solar::mandaean` declares a
// thirteen-position month cycle with the Parwanaia ninth, which a list of
// twelve cannot fill; no other source read prints the script. Mandaic
// runs right to left. The week begins on Saturday because CLDR 48
// `weekData/firstDay` says so for IQ, where the language is spoken, and
// there is no CLDR locale to say otherwise; numbering is `latn`, this
// crate having no Mandaic numerals.

const MID: LocaleData = LocaleData {
    tag: "mid",
    direction: Direction::RightToLeft,
    numbering: "latn",
    first_day_of_week: Weekday::Saturday,
    casing: CasingStyle::Standard,
    capitalises_month_names: false,
    weekdays: ContextualNames::same(weekday_widths(
        &[
            "ࡕࡓࡉࡍ ࡄࡀࡁࡔࡀࡁࡀ",
            "ࡕࡋࡀࡕࡀ ࡄࡀࡁࡔࡀࡁࡀ",
            "ࡀࡓࡁࡀ ࡄࡀࡁࡔࡀࡁࡀ",
            "ࡄࡀࡌࡔࡀ ࡄࡀࡁࡔࡀࡁࡀ",
            "ࡉࡅࡌࡀ ࡃ ࡓࡀࡄࡀࡈࡉࡀ",
            "ࡔࡀࡐࡕࡀ",
            "ࡄࡀࡁࡔࡀࡁࡀ",
        ],
        &[],
        &[],
        &[],
    )),
    day_periods: ContextualNames::EMPTY,
    cycle: SexagenaryNames::EMPTY,
    calendars: &[gregorian(&[], EraNames::EMPTY, ContextualNames::EMPTY)],
};

// --- Malayalam ------------------------------------------------------------
//
// The language of the Kollam-era calendar. Gregorian vocabulary from CLDR
// 48 `common/main/ml.xml`: the wide months, the abbreviated ones (May, June
// and July resolving to the wide form), the stand-alone narrow ones, the
// weekdays in four widths, the quarters and the eras; CLDR's `ml` has no
// day periods of its own. Several forms carry a zero-width non-joiner, as
// CLDR writes them. The twelve months of its `calendar type="indian"` are
// the national calendar's Sanskrit names in Malayalam (ചൈത്രം …), keyed to
// `indian` with the era abbreviation ശക; they are not the Kollam year's
// months, which are the twelve of the months table of Wikipedia,
// "Malayalam calendar", read 2026-09-26, Chingam first as
// `hc_calendars_indic::hindu_solar::MALAYALAM` counts them, keyed to
// `hindu-solar-malayalam`. The week begins on Sunday (CLDR 48
// `weekData/firstDay`, IN); numbering is `latn`, CLDR's default for `ml`,
// this crate having no `mlym`.

const ML_CALENDARS: &[CalendarNames] = &[
    gregorian(
        &[month_cycle(ContextualNames::same(widths(
            &[
                "ജനുവരി",
                "ഫെബ്രുവരി",
                "മാർച്ച്",
                "ഏപ്രിൽ",
                "മേയ്",
                "ജൂൺ",
                "ജൂലൈ",
                "ഓഗസ്റ്റ്",
                "സെപ്റ്റംബർ",
                "ഒക്\u{200c}ടോബർ",
                "നവംബർ",
                "ഡിസംബർ",
            ],
            &[
                "ജനു",
                "ഫെബ്രു",
                "മാർ",
                "ഏപ്രി",
                "മേയ്",
                "ജൂൺ",
                "ജൂലൈ",
                "ഓഗ",
                "സെപ്റ്റം",
                "ഒക്ടോ",
                "നവം",
                "ഡിസം",
            ],
            &[
                "ജ",
                "ഫെ",
                "മാ",
                "ഏ",
                "മെ",
                "ജൂൺ",
                "ജൂ",
                "ഓ",
                "സെ",
                "ഒ",
                "ന",
                "ഡി",
            ],
        )))],
        gregorian_eras(
            &["ക്രിസ്\u{200c}തുവിന് മുമ്പ്", "ആന്നോ ഡൊമിനി"],
            &["ബിസി", "എഡി"],
            &[],
        ),
        ContextualNames::same(widths(
            &["ഒന്നാം പാദം", "രണ്ടാം പാദം", "മൂന്നാം പാദം", "നാലാം പാദം"],
            &[],
            &[],
        )),
    ),
    dated(
        &[CalendarId("indian")],
        &[months(&[
            "ചൈത്രം",
            "വൈശാഖം",
            "ജ്യേഷ്ഠം",
            "ആഷാഢം",
            "ശ്രാവണം",
            "ഭാദ്രം",
            "ആശ്വിനം",
            "കാർത്തികം",
            "മാർഗശീർഷം",
            "പൗഷം",
            "മാഘം",
            "ഫാൽഗുനം",
        ])],
        &["saka"],
        &["ശക"],
    ),
    dated(
        &[CalendarId("hindu-solar-malayalam")],
        &[months(&[
            "ചിങ്ങം",
            "കന്നി",
            "തുലാം",
            "വൃശ്ചികം",
            "ധനു",
            "മകരം",
            "കുംഭം",
            "മീനം",
            "മേടം",
            "ഇടവം",
            "മിഥുനം",
            "കർക്കടകം",
        ])],
        &[],
        &[],
    ),
];

const ML: LocaleData = LocaleData {
    tag: "ml",
    direction: Direction::LeftToRight,
    numbering: "latn",
    first_day_of_week: Weekday::Sunday,
    casing: CasingStyle::Standard,
    capitalises_month_names: false,
    weekdays: ContextualNames::same(weekday_widths(
        &[
            "തിങ്കളാഴ്\u{200c}ച",
            "ചൊവ്വാഴ്ച",
            "ബുധനാഴ്\u{200c}ച",
            "വ്യാഴാഴ്\u{200c}ച",
            "വെള്ളിയാഴ്\u{200c}ച",
            "ശനിയാഴ്\u{200c}ച",
            "ഞായറാഴ്\u{200c}ച",
        ],
        &["തിങ്കൾ", "ചൊവ്വ", "ബുധൻ", "വ്യാഴം", "വെള്ളി", "ശനി", "ഞായർ"],
        &["തി", "ചൊ", "ബു", "വ്യാ", "വെ", "ശ", "ഞാ"],
        &["തി", "ചൊ", "ബു", "വ്യാ", "വെ", "ശ", "ഞാ"],
    )),
    day_periods: ContextualNames::EMPTY,
    cycle: SexagenaryNames::EMPTY,
    calendars: ML_CALENDARS,
};

// --- Burmese --------------------------------------------------------------
//
// The language of the Burmese calendar. Gregorian vocabulary from CLDR 48
// `common/main/my.xml`, `calendar type="gregorian"`; the abbreviated
// weekdays are the inheritance marker there and so resolve to the wide
// forms, which is why that width is empty. Release 48 prints the
// abbreviated AD era with its vowel signs out of order (အဒေီ); the form here
// is the one CLDR's main branch has corrected it to (commit 49b5089,
// 2026-09-24). Myanmar's week begins on Sunday (CLDR 48 `weekData`, MM)
// and the default numbering system is `mymr`. CLDR carries no Burmese
// calendar and no Buddhist-era names for `my`.
//
// The twelve Burmese months are those of the months table of Wikipedia,
// "Burmese calendar", read 2026-09-25 — the same forms
// `hc_calendars_regional::burmese::MONTHS` holds, which the calendar's
// numbered shape does not declare, so this entry is what lets a Burmese
// date print in Burmese. The page names the intercalary month only as
// ဝါထပ်, "watat", and does not spell "Second Waso" in Burmese script, so
// the leap-month prefix is empty.

const MY_CALENDARS: &[CalendarNames] = &[
    gregorian(
        &[month_cycle(ContextualNames::same(widths(
            &[
                "ဇန်နဝါရီ",
                "ဖေဖော်ဝါရီ",
                "မတ်",
                "ဧပြီ",
                "မေ",
                "ဇွန်",
                "ဇူလိုင်",
                "ဩဂုတ်",
                "စက်တင်ဘာ",
                "အောက်တိုဘာ",
                "နိုဝင်ဘာ",
                "ဒီဇင်ဘာ",
            ],
            &[
                "ဇန်",
                "ဖေ",
                "မတ်",
                "ဧ",
                "မေ",
                "ဇွန်",
                "ဇူ",
                "ဩ",
                "စက်",
                "အောက်",
                "နို",
                "ဒီ",
            ],
            &["ဇ", "ဖ", "မ", "ဧ", "မ", "ဇ", "ဇ", "ဩ", "စ", "အ", "န", "ဒ"],
        )))],
        gregorian_eras(&["ခရစ်တော် မပေါ်မီနှစ်", "ခရစ်နှစ်"], &["ဘီစီ", "အေဒီ"], &[]),
        ContextualNames::same(widths(
            &["ပထမ သုံးလပတ်", "ဒုတိယ သုံးလပတ်", "တတိယ သုံးလပတ်", "စတုတ္ထ သုံးလပတ်"],
            &["Q1", "Q2", "Q3", "Q4"],
            &["ပ", "ဒု", "တ", "စ"],
        )),
    ),
    dated(
        BURMESE_CALENDARS,
        &[months(&[
            "တန်ခူး",
            "ကဆုန်",
            "နယုန်",
            "ဝါဆို",
            "ဝါခေါင်",
            "တော်သလင်း",
            "သီတင်းကျွတ်",
            "တန်ဆောင်မုန်း",
            "နတ်တော်",
            "ပြာသို",
            "တပို့တွဲ",
            "တပေါင်း",
        ])],
        &[],
        &[],
    ),
];

const MY: LocaleData = LocaleData {
    tag: "my",
    direction: Direction::LeftToRight,
    numbering: "mymr",
    first_day_of_week: Weekday::Sunday,
    casing: CasingStyle::Standard,
    capitalises_month_names: false,
    weekdays: ContextualNames::same(weekday_widths(
        &[
            "တနင်္လာ",
            "အင်္ဂါ",
            "ဗုဒ္ဓဟူး",
            "ကြာသပတေး",
            "သောကြာ",
            "စနေ",
            "တနင်္ဂနွေ",
        ],
        &[],
        &["လာ", "ဂါ", "ဟူး", "တေး", "ကြာ", "နေ", "နွေ"],
        &["တ", "အ", "ဗ", "က", "သ", "စ", "တ"],
    )),
    day_periods: ContextualNames::same(widths(&["နံနက်", "ညနေ"], &[], &[])),
    cycle: SexagenaryNames::EMPTY,
    calendars: MY_CALENDARS,
};

// --- Nahuatl --------------------------------------------------------------
//
// The language of the Aztec counts. CLDR has no `nah` locale, so this
// entry has no Gregorian vocabulary, weekdays, day periods or eras. The
// twenty day-signs and the nineteen months are the forms
// `hc_calendars_regional::aztec` declares with its shape: the day-signs
// Cipactli to Xochitl as Wikipedia, "Tonalpohualli", lists them and the
// months Izcalli to Nemontemi in Reingold and Dershowitz's numbering
// (*Calendrical Calculations*, 4th ed., 2018), as
// docs/systems/mesoamerican-counts.md sets out; the locale restates them
// so that the language, and not only the calendar, claims them. The
// spelling is the unmarked one of those sources, without vowel length or
// saltillo. The week begins on Sunday (CLDR 48 `weekData/firstDay`, MX)
// and numbering is `latn`.

const NAH_CALENDARS: &[CalendarNames] = &[
    gregorian(&[], EraNames::EMPTY, ContextualNames::EMPTY),
    CalendarNames {
        calendars: &[CalendarId("aztec-tonalpohualli")],
        cycles: &[cycle(
            "day-sign",
            &[
                "Cipactli",
                "Ehecatl",
                "Calli",
                "Cuetzpalin",
                "Coatl",
                "Miquiztli",
                "Mazatl",
                "Tochtli",
                "Atl",
                "Itzcuintli",
                "Ozomatli",
                "Malinalli",
                "Acatl",
                "Ocelotl",
                "Cuauhtli",
                "Cozcacuauhtli",
                "Ollin",
                "Tecpatl",
                "Quiahuitl",
                "Xochitl",
            ],
        )],
        leap_month_prefix: "",
        eras: EraNames::EMPTY,
        quarters: ContextualNames::EMPTY,
    },
    dated(
        &[CalendarId("aztec-xiuhpohualli")],
        &[months(&[
            "Izcalli",
            "Atlcahualo",
            "Tlacaxipehualiztli",
            "Tozoztontli",
            "Hueytozoztli",
            "Toxcatl",
            "Etzalcualiztli",
            "Tecuilhuitontli",
            "Hueytecuilhuitl",
            "Tlaxochimaco",
            "Xocotlhuetzi",
            "Ochpaniztli",
            "Teotleco",
            "Tepeilhuitl",
            "Quecholli",
            "Panquetzaliztli",
            "Atemoztli",
            "Tititl",
            "Nemontemi",
        ])],
        &[],
        &[],
    ),
];

const NAH: LocaleData = LocaleData {
    tag: "nah",
    direction: Direction::LeftToRight,
    numbering: "latn",
    first_day_of_week: Weekday::Sunday,
    casing: CasingStyle::Standard,
    capitalises_month_names: true,
    weekdays: ContextualNames::EMPTY,
    day_periods: ContextualNames::EMPTY,
    cycle: SexagenaryNames::EMPTY,
    calendars: NAH_CALENDARS,
};

// --- Nepali ---------------------------------------------------------------
//
// The language of the Bikram Sambat, and the language Nepal Sambat is
// printed in beside Newar. Gregorian vocabulary from CLDR 48
// `common/main/ne.xml`, `calendar type="gregorian"`: the abbreviated months
// are the inheritance marker, resolving to the wide forms, and the narrow
// months are the stand-alone narrow list except February, which the format
// list spells फेब where the stand-alone list has a doubled vowel sign; only
// `eraAbbr` is stated, so it stands as the wide era names too. The
// quarters differ between contexts, पहिलो inside a date and प्रथम on their
// own. Nepal's week begins on Sunday (CLDR 48 `weekData`, NP) and the
// default numbering system is `deva`.
//
// The Bikram Sambat months are the Government of Nepal's spellings, from
// the Ministry of Home Affairs's holiday notices in the Nepal Rajpatra
// (Khaṇḍa 72 No. 65, 73 No. 54, 74 No. 59 and 75 No. 67, Part 5) as
// `hc_calendars_indic::bikram_sambat::MONTHS_DEVANAGARI` carries them; the
// calendar declares its English names with its shape, so the Devanagari
// needs a locale. CLDR's `ne` has no Bikram Sambat; its `indian` (Śaka)
// calendar lists the same Nepali forms, Chaitra first, differing only in
// मङसिर for the gazette's मङ्सिर, and is not used. No source read names
// the era in Devanagari, so none is written.
//
// The Nepal Sambat months are the Devanagari column of the months table of
// Wikipedia, "Nepal Sambat", read 2026-09-25 — the Newar names as Nepal's
// Devanagari prints them, which is what a Nepali reader sees; CLDR has no
// `new` (Newar) locale to key them to instead, and the Newa-script forms
// stay with the calendar module. The table names the intercalary month
// अनला, Analā, as a month of its own rather than as a prefix, so the
// leap-month prefix is empty.

const NE_CALENDARS: &[CalendarNames] = &[
    gregorian(
        &[month_cycle(ContextualNames::same(widths(
            &[
                "जनवरी",
                "फेब्रुअरी",
                "मार्च",
                "अप्रिल",
                "मे",
                "जुन",
                "जुलाई",
                "अगस्ट",
                "सेप्टेम्बर",
                "अक्टोबर",
                "नोभेम्बर",
                "डिसेम्बर",
            ],
            &[],
            &[
                "जन",
                "फेब",
                "मार्च",
                "अप्र",
                "मे",
                "जुन",
                "जुल",
                "अग",
                "सेप",
                "अक्टो",
                "नोभे",
                "डिसे",
            ],
        )))],
        gregorian_eras(&["ईसा पूर्व", "सन्"], &[], &[]),
        ContextualNames {
            format: widths(
                &[
                    "पहिलो त्रैमासिक",
                    "दोस्रो त्रैमासिक",
                    "तेस्रो त्रैमासिक",
                    "चौथो त्रैमासिक",
                ],
                &[],
                &[],
            ),
            standalone: widths(
                &[
                    "प्रथम त्रैमासिक",
                    "द्वितीय त्रैमासिक",
                    "तृतीय त्रैमासिक",
                    "चतुर्थ त्रैमासिक",
                ],
                &[],
                &["१", "२", "३", "४"],
            ),
        },
    ),
    dated(
        &[CalendarId("bikram-sambat")],
        &[months(&[
            "वैशाख",
            "जेठ",
            "असार",
            "साउन",
            "भदौ",
            "असोज",
            "कात्तिक",
            "मङ्सिर",
            "पुस",
            "माघ",
            "फागुन",
            "चैत",
        ])],
        &[],
        &[],
    ),
    lunisolar(
        &[CalendarId("nepal-sambat")],
        &[months(&[
            "कछला",
            "थिंला",
            "प्वँहेला",
            "सिला",
            "चिला",
            "चौला",
            "बछला",
            "तछला",
            "दिला",
            "गुंला",
            "ञंला",
            "कौला",
        ])],
        "",
    ),
];

const NE: LocaleData = LocaleData {
    tag: "ne",
    direction: Direction::LeftToRight,
    numbering: "deva",
    first_day_of_week: Weekday::Sunday,
    casing: CasingStyle::Standard,
    capitalises_month_names: false,
    weekdays: ContextualNames::same(weekday_widths(
        &[
            "सोमबार",
            "मङ्गलबार",
            "बुधबार",
            "बिहिबार",
            "शुक्रबार",
            "शनिबार",
            "आइतबार",
        ],
        &["सोम", "मङ्गल", "बुध", "बिहि", "शुक्र", "शनि", "आइत"],
        &[],
        &["सो", "म", "बु", "बि", "शु", "श", "आ"],
    )),
    day_periods: ContextualNames::same(widths(&["पूर्वाह्न", "अपराह्न"], &[], &[])),
    cycle: SexagenaryNames::EMPTY,
    calendars: NE_CALENDARS,
};

// --- Dutch ----------------------------------------------------------------

const NL: LocaleData = LocaleData {
    tag: "nl",
    direction: Direction::LeftToRight,
    numbering: "latn",
    first_day_of_week: Weekday::Monday,
    casing: CasingStyle::Standard,
    capitalises_month_names: false,
    weekdays: ContextualNames::same(weekday_widths(
        &[
            "maandag",
            "dinsdag",
            "woensdag",
            "donderdag",
            "vrijdag",
            "zaterdag",
            "zondag",
        ],
        &["ma", "di", "wo", "do", "vr", "za", "zo"],
        &[],
        &["M", "D", "W", "D", "V", "Z", "Z"],
    )),
    day_periods: ContextualNames::same(widths(&["a.m.", "p.m."], &[], &[])),
    cycle: SexagenaryNames::EMPTY,
    calendars: &[gregorian(
        &[month_cycle(ContextualNames::same(widths(
            &[
                "januari",
                "februari",
                "maart",
                "april",
                "mei",
                "juni",
                "juli",
                "augustus",
                "september",
                "oktober",
                "november",
                "december",
            ],
            &[
                "jan", "feb", "mrt", "apr", "mei", "jun", "jul", "aug", "sep", "okt", "nov", "dec",
            ],
            &["J", "F", "M", "A", "M", "J", "J", "A", "S", "O", "N", "D"],
        )))],
        gregorian_eras(
            &["voor Christus", "na Christus"],
            &["v.Chr.", "n.Chr."],
            &[],
        ),
        ContextualNames::EMPTY,
    )],
};

// --- Polish ---------------------------------------------------------------

const PL: LocaleData = LocaleData {
    tag: "pl",
    direction: Direction::LeftToRight,
    numbering: "latn",
    first_day_of_week: Weekday::Monday,
    casing: CasingStyle::Standard,
    capitalises_month_names: false,
    weekdays: ContextualNames::same(weekday_widths(
        &[
            "poniedziałek",
            "wtorek",
            "środa",
            "czwartek",
            "piątek",
            "sobota",
            "niedziela",
        ],
        &["pon.", "wt.", "śr.", "czw.", "pt.", "sob.", "niedz."],
        &[],
        &["p", "w", "ś", "c", "p", "s", "n"],
    )),
    day_periods: ContextualNames::same(widths(&["AM", "PM"], &[], &[])),
    cycle: SexagenaryNames::EMPTY,
    calendars: &[gregorian(
        &[month_cycle(ContextualNames {
            format: widths(
                &[
                    "stycznia",
                    "lutego",
                    "marca",
                    "kwietnia",
                    "maja",
                    "czerwca",
                    "lipca",
                    "sierpnia",
                    "września",
                    "października",
                    "listopada",
                    "grudnia",
                ],
                &[
                    "sty", "lut", "mar", "kwi", "maj", "cze", "lip", "sie", "wrz", "paź", "lis",
                    "gru",
                ],
                &[],
            ),
            standalone: widths(
                &[
                    "styczeń",
                    "luty",
                    "marzec",
                    "kwiecień",
                    "maj",
                    "czerwiec",
                    "lipiec",
                    "sierpień",
                    "wrzesień",
                    "październik",
                    "listopad",
                    "grudzień",
                ],
                &[],
                &[],
            ),
        })],
        gregorian_eras(&["przed naszą erą", "naszej ery"], &["p.n.e.", "n.e."], &[]),
        ContextualNames::EMPTY,
    )],
};

// --- Portuguese -----------------------------------------------------------

const PT: LocaleData = LocaleData {
    tag: "pt",
    direction: Direction::LeftToRight,
    numbering: "latn",
    first_day_of_week: Weekday::Monday,
    casing: CasingStyle::Standard,
    capitalises_month_names: false,
    weekdays: ContextualNames::same(weekday_widths(
        &[
            "segunda-feira",
            "terça-feira",
            "quarta-feira",
            "quinta-feira",
            "sexta-feira",
            "sábado",
            "domingo",
        ],
        &["seg.", "ter.", "qua.", "qui.", "sex.", "sáb.", "dom."],
        &[],
        &["S", "T", "Q", "Q", "S", "S", "D"],
    )),
    day_periods: ContextualNames::same(widths(&["AM", "PM"], &[], &[])),
    cycle: SexagenaryNames::EMPTY,
    calendars: &[gregorian(
        &[month_cycle(ContextualNames::same(widths(
            &[
                "janeiro",
                "fevereiro",
                "março",
                "abril",
                "maio",
                "junho",
                "julho",
                "agosto",
                "setembro",
                "outubro",
                "novembro",
                "dezembro",
            ],
            &[
                "jan.", "fev.", "mar.", "abr.", "mai.", "jun.", "jul.", "ago.", "set.", "out.",
                "nov.", "dez.",
            ],
            &["J", "F", "M", "A", "M", "J", "J", "A", "S", "O", "N", "D"],
        )))],
        gregorian_eras(
            &["antes de Cristo", "depois de Cristo"],
            &["a.C.", "d.C."],
            &[],
        ),
        ContextualNames::EMPTY,
    )],
};

// --- Russian --------------------------------------------------------------
//
// The textbook case for the format/standalone split: a Russian date reads
// «12 сентября», genitive, while a calendar heading reads «Сентябрь».

const RU: LocaleData = LocaleData {
    tag: "ru",
    direction: Direction::LeftToRight,
    numbering: "latn",
    first_day_of_week: Weekday::Monday,
    casing: CasingStyle::Standard,
    capitalises_month_names: false,
    weekdays: ContextualNames::same(weekday_widths(
        &[
            "понедельник",
            "вторник",
            "среда",
            "четверг",
            "пятница",
            "суббота",
            "воскресенье",
        ],
        &["пн", "вт", "ср", "чт", "пт", "сб", "вс"],
        &[],
        &["П", "В", "С", "Ч", "П", "С", "В"],
    )),
    day_periods: ContextualNames::same(widths(&["AM", "PM"], &[], &[])),
    cycle: SexagenaryNames::EMPTY,
    calendars: &[gregorian(
        &[month_cycle(ContextualNames {
            format: widths(
                &[
                    "января",
                    "февраля",
                    "марта",
                    "апреля",
                    "мая",
                    "июня",
                    "июля",
                    "августа",
                    "сентября",
                    "октября",
                    "ноября",
                    "декабря",
                ],
                &[
                    "янв.",
                    "февр.",
                    "мар.",
                    "апр.",
                    "мая",
                    "июн.",
                    "июл.",
                    "авг.",
                    "сент.",
                    "окт.",
                    "нояб.",
                    "дек.",
                ],
                &[],
            ),
            standalone: widths(
                &[
                    "январь",
                    "февраль",
                    "март",
                    "апрель",
                    "май",
                    "июнь",
                    "июль",
                    "август",
                    "сентябрь",
                    "октябрь",
                    "ноябрь",
                    "декабрь",
                ],
                &[
                    "янв.",
                    "февр.",
                    "март",
                    "апр.",
                    "май",
                    "июнь",
                    "июль",
                    "авг.",
                    "сент.",
                    "окт.",
                    "нояб.",
                    "дек.",
                ],
                &[],
            ),
        })],
        gregorian_eras(
            &["до Рождества Христова", "от Рождества Христова"],
            &["до н. э.", "н. э."],
            &[],
        ),
        ContextualNames::same(widths(
            &["1-й квартал", "2-й квартал", "3-й квартал", "4-й квартал"],
            &["1-й кв.", "2-й кв.", "3-й кв.", "4-й кв."],
            &[],
        )),
    )],
};

// --- Sanskrit -------------------------------------------------------------
//
// The language of the Hindu calendars' own names. Gregorian vocabulary
// from CLDR 48 `common/main/sa.xml`, `calendar type="gregorian"`: the wide
// months, which end in मासः, the narrow months, the wide day periods and
// the quarters. Two widths are left to inherit because CLDR 48 prints an
// ASCII colon where a visarga (ः) belongs — the abbreviated months
// (जनवरी: …) and Thursday's wide weekday (गुरुवासर:, beside सोमवासरः and
// the rest) — and it is still so on CLDR main as of 2026-09-26; carrying a
// source's typo is not faithfulness, so the forms wait until CLDR corrects
// them. The abbreviated months therefore inherit the wide form, and the
// weekdays inherit as a whole: a locale that states any weekday must
// state the wide seven, and the wide seven cannot be stated without
// Thursday, so the abbreviated and narrow forms CLDR does print correctly
// go with it. The eras are not written: CLDR's
// default forms are the inheritance marker, resolving to root's Latin BCE
// and CE, and the Devanagari इ.स.पू. and संवत् are its `alt="variant"`
// forms. The twelve lunar months are the Devanagari the amānta calendar
// declares with its shape — the month headings of the Rashtriya Panchang's
// Sanskrit edition and Wikipedia, "Hindu calendar", as
// `hc_calendars_indic::hindu_lunar::MONTHS` cites them — keyed to the
// amānta and pūrṇimānta calendars so that the language, and not only the
// calendar, claims them, with the intercalary prefix अधिक, the first
// element of अधिकमास as Wikipedia, "Adhik Maas", read 2026-09-26, spells
// the Sanskrit, kept apart from the month name as the English "Adhika "
// is. The Vikrami solar months are the same names from Vaiśākha
// (`hc_seasons::zodiac::rashi::VIKRAMI`, after the Rashtriya Panchang's
// Punjab and Odisha column), keyed to `hindu-solar-vikrami`. Not carried:
// the rāśi names in Devanagari, which no source read prints (the Old Hindu
// solar calendar keeps its IAST names), and the twenty-seven nakṣatras,
// which no calendar declares as a cycle. The week begins on Sunday (CLDR 48
// `weekData/firstDay`, IN) and the default numbering system is `deva`.

const SA_LUNAR_MONTHS: &[&str] = &[
    "चैत्र",
    "वैशाख",
    "ज्येष्ठ",
    "आषाढ",
    "श्रावण",
    "भाद्रपद",
    "आश्विन",
    "कार्तिक",
    "मार्गशीर्ष",
    "पौष",
    "माघ",
    "फाल्गुन",
];

const SA_VIKRAMI_MONTHS: &[&str] = &[
    "वैशाख",
    "ज्येष्ठ",
    "आषाढ",
    "श्रावण",
    "भाद्रपद",
    "आश्विन",
    "कार्तिक",
    "मार्गशीर्ष",
    "पौष",
    "माघ",
    "फाल्गुन",
    "चैत्र",
];

const SA_CALENDARS: &[CalendarNames] = &[
    gregorian(
        &[month_cycle(ContextualNames::same(widths(
            &[
                "जनवरीमासः",
                "फरवरीमासः",
                "मार्चमासः",
                "अप्रैलमासः",
                "मईमासः",
                "जूनमासः",
                "जुलाईमासः",
                "अगस्तमासः",
                "सितंबरमासः",
                "अक्तूबरमासः",
                "नवंबरमासः",
                "दिसंबरमासः",
            ],
            &[],
            &[
                "ज", "फ", "मा", "अ", "म", "जू", "जु", "अ", "सि", "अ", "न", "दि",
            ],
        )))],
        EraNames::EMPTY,
        ContextualNames::same(widths(
            &[
                "प्रथम त्रैमासिक",
                "द्वितीय त्रैमासिक",
                "तृतीय त्रैमासिक",
                "चतुर्थ त्रैमासिक",
            ],
            &["त्रैमासिक1", "त्रैमासिक2", "त्रैमासिक3", "त्रैमासिक4"],
            &[],
        )),
    ),
    lunisolar(
        &[
            CalendarId("hindu-lunar"),
            CalendarId("hindu-lunar-purnimanta"),
        ],
        &[months(SA_LUNAR_MONTHS)],
        "अधिक ",
    ),
    dated(
        &[CalendarId("hindu-solar-vikrami")],
        &[months(SA_VIKRAMI_MONTHS)],
        &[],
        &[],
    ),
];

const SA: LocaleData = LocaleData {
    tag: "sa",
    direction: Direction::LeftToRight,
    numbering: "deva",
    first_day_of_week: Weekday::Sunday,
    casing: CasingStyle::Standard,
    capitalises_month_names: false,
    weekdays: ContextualNames::EMPTY,
    day_periods: ContextualNames::same(widths(&["पूर्वाह्न", "अपराह्न"], &[], &[])),
    cycle: SexagenaryNames::EMPTY,
    calendars: SA_CALENDARS,
};

// --- Syriac ---------------------------------------------------------------
//
// The language of the Assyrian calendar. Gregorian vocabulary from CLDR 48
// `common/main/syr.xml`: the wide months, which are the Syriac months
// Kānōn ʾḤrāy to Kānōn Qḏīm under the Gregorian numbering, the
// abbreviated ones (only the two Tishrins and two Kanoons abbreviate; the
// rest resolve to the wide form), the stand-alone narrow ones, the
// weekdays (Saturday's abbreviation resolving to the wide form), the
// quarters, the day periods and the eras; several abbreviations begin
// with the Syriac abbreviation mark U+070F and end in a zero-width
// non-joiner, as CLDR writes them. The Assyrian months are the Syriac
// column of the months table of Wikipedia, "Assyrian calendar", read
// 2026-09-26, which `hc_calendars_solar::assyrian` cites, in its
// vocalised East Syriac forms, Neesan first as the calendar counts, with
// ܛܲܒܵܚ for the fifth month where the table offers ܐܵܒ or ܛܲܒܵܚ and the
// module's own name is Tabakh; the two Tishrins and two Kanoons are
// numbered with the letters ܐ and ܒ as the table prints them. CLDR's
// unvocalised Gregorian forms are not reused for the calendar because its
// August is ܐܒ. No Syriac form of the era AY was found in a source read.
// Syriac runs right to left; the week begins on Saturday (CLDR 48
// `weekData/firstDay`, IQ) and the numbering system is `latn`.

const SYR_CALENDARS: &[CalendarNames] = &[
    gregorian(
        &[month_cycle(ContextualNames::same(widths(
            &[
                "ܟܢܘܢ ܐܚܪܝܐ",
                "ܫܒܛ",
                "ܐܕܪ",
                "ܢܝܣܢ",
                "ܐܝܪ",
                "ܚܙܝܪܢ",
                "ܬܡܘܙ",
                "ܐܒ",
                "ܐܝܠܘܠ",
                "ܬܫܪܝܢ ܩܕܡܝܐ",
                "ܬܫܪܝܢ ܐܚܪܝܐ",
                "ܟܢܘܢ ܩܕܡܝܐ",
            ],
            &[
                "ܟܢܘܢ ܒ",
                "ܫܒܛ",
                "ܐܕܪ",
                "ܢܝܣܢ",
                "ܐܝܪ",
                "ܚܙܝܪܢ",
                "ܬܡܘܙ",
                "ܐܒ",
                "ܐܝܠܘܠ",
                "ܬܫܪܝܢ ܐ",
                "ܬܫܪܝܢ ܒ",
                "ܟܢܘܢ ܐ",
            ],
            &["ܟ", "ܫ", "ܐ", "ܢ", "ܐ", "ܚ", "ܬ", "ܐ", "ܐ", "ܬ", "ܬ", "ܟ"],
        )))],
        gregorian_eras(
            &["ܩܕܡ ܡܫܝܚܐ", "ܫܢܬܐ ܡܪܢܝܬܐ"],
            &["\u{70f}ܩܡ\u{200c}", "\u{70f}ܫܡ\u{200c}"],
            &[],
        ),
        ContextualNames::same(widths(
            &["ܪܘܒܥܐ ܩܕܡܝܐ", "ܪܘܒܥܐ ܬܪܝܢܐ", "ܪܘܒܥܐ ܬܠܝܬܝܐ", "ܪܘܒܥܐ ܪܒܝܥܝܐ"],
            &["\u{70f}ܪ ܐ", "\u{70f}ܪ ܒ", "\u{70f}ܪ ܓ", "\u{70f}ܪ ܕ"],
            &[],
        )),
    ),
    dated(
        &[CalendarId("assyrian")],
        &[months(&[
            "ܢܝܼܣܵܢ",
            "ܐܝܼܵܪ",
            "ܚܙܝܼܪܵܢ",
            "ܬܲܡܘܼܙ",
            "ܛܲܒܵܚ",
            "ܐܝܼܠܘܼܠ",
            "ܬܸܫܪܝܼܢ ܐ",
            "ܬܸܫܪܝܼܢ ܒ",
            "ܟܵܢܘܿܢ ܐ",
            "ܟܵܢܘܿܢ ܒ",
            "ܫܒ݂ܵܛ",
            "ܐܵܕܲܪ",
        ])],
        &[],
        &[],
    ),
];

const SYR: LocaleData = LocaleData {
    tag: "syr",
    direction: Direction::RightToLeft,
    numbering: "latn",
    first_day_of_week: Weekday::Saturday,
    casing: CasingStyle::Standard,
    capitalises_month_names: false,
    weekdays: ContextualNames::same(weekday_widths(
        &[
            "ܬܪܝܢܒܫܒܐ",
            "ܬܠܬܒܫܒܐ",
            "ܐܪܒܥܒܫܒܐ",
            "ܚܡܫܒܫܒܐ",
            "ܥܪܘܒܬܐ",
            "ܫܒܬܐ",
            "ܚܕܒܫܒܐ",
        ],
        &["ܬܪܝܢ", "ܬܠܬ", "ܐܪܒܥ", "ܚܡܫ", "ܥܪܘ", "ܫܒܬܐ", "ܚܕ"],
        &[],
        &["ܬ", "ܬ", "ܐ", "ܚ", "ܥ", "ܫ", "ܚ"],
    )),
    day_periods: ContextualNames::same(widths(
        &["\u{70f}ܩܛ\u{200c}", "\u{70f}ܒܛ\u{200c}"],
        &[],
        &["\u{70f}ܩ\u{200c}", "\u{70f}ܒ\u{200c}"],
    )),
    cycle: SexagenaryNames::EMPTY,
    calendars: SYR_CALENDARS,
};

// --- Tamil ----------------------------------------------------------------
//
// The language of the Tamil solar calendar. Gregorian vocabulary from
// CLDR 48 `common/main/ta.xml`: the wide months, the abbreviated ones
// (May, June and July resolving to the wide form), the stand-alone narrow
// ones, the weekdays (Saturday's abbreviation resolving to the wide form),
// the quarters and the eras; CLDR's `ta` has no day periods of its own.
// The twelve months of its `calendar type="indian"` are the Tamil months
// சித்திரை to பங்குனி, Chithirai first, in the same order
// `hc_seasons::zodiac::rashi::TAMIL` counts them from the Meṣa saṅkrānti,
// so one list serves both the national calendar, with CLDR's era
// abbreviation சாகா, and `hindu-solar-tamil`. The week begins on Sunday
// (CLDR 48 `weekData/firstDay`, IN) and the numbering system is `latn`.

const TA_SOLAR_MONTHS: &[&str] = &[
    "சித்திரை",
    "வைகாசி",
    "ஆனி",
    "ஆடி",
    "ஆவணி",
    "புரட்டாசி",
    "ஐப்பசி",
    "கார்த்திகை",
    "மார்கழி",
    "தை",
    "மாசி",
    "பங்குனி",
];

const TA_CALENDARS: &[CalendarNames] = &[
    gregorian(
        &[month_cycle(ContextualNames::same(widths(
            &[
                "ஜனவரி",
                "பிப்ரவரி",
                "மார்ச்",
                "ஏப்ரல்",
                "மே",
                "ஜூன்",
                "ஜூலை",
                "ஆகஸ்ட்",
                "செப்டம்பர்",
                "அக்டோபர்",
                "நவம்பர்",
                "டிசம்பர்",
            ],
            &[
                "ஜன.",
                "பிப்.",
                "மார்.",
                "ஏப்.",
                "மே",
                "ஜூன்",
                "ஜூலை",
                "ஆக.",
                "செப்.",
                "அக்.",
                "நவ.",
                "டிச.",
            ],
            &[
                "ஜ", "பி", "மா", "ஏ", "மே", "ஜூ", "ஜூ", "ஆ", "செ", "அ", "ந", "டி",
            ],
        )))],
        gregorian_eras(
            &["கிறிஸ்துவுக்கு முன்", "அன்னோ டோமினி"],
            &["கி.மு.", "கி.பி."],
            &[],
        ),
        ContextualNames::same(widths(
            &["முதல் காலாண்டு", "இரண்டாம் காலாண்டு", "மூன்றாம் காலாண்டு", "நான்காம் காலாண்டு"],
            &["கா.1", "கா.2", "கா.3", "கா.4"],
            &[],
        )),
    ),
    dated(
        &[CalendarId("indian")],
        &[months(TA_SOLAR_MONTHS)],
        &["saka"],
        &["சாகா"],
    ),
    dated(
        &[CalendarId("hindu-solar-tamil")],
        &[months(TA_SOLAR_MONTHS)],
        &[],
        &[],
    ),
];

const TA: LocaleData = LocaleData {
    tag: "ta",
    direction: Direction::LeftToRight,
    numbering: "latn",
    first_day_of_week: Weekday::Sunday,
    casing: CasingStyle::Standard,
    capitalises_month_names: false,
    weekdays: ContextualNames::same(weekday_widths(
        &["திங்கள்", "செவ்வாய்", "புதன்", "வியாழன்", "வெள்ளி", "சனி", "ஞாயிறு"],
        &["திங்.", "செவ்.", "புத.", "வியா.", "வெள்.", "சனி", "ஞாயி."],
        &["தி", "செ", "பு", "வி", "வெ", "ச", "ஞா"],
        &["தி", "செ", "பு", "வி", "வெ", "ச", "ஞா"],
    )),
    day_periods: ContextualNames::EMPTY,
    cycle: SexagenaryNames::EMPTY,
    calendars: TA_CALENDARS,
};

// --- Thai -----------------------------------------------------------------

const TH: LocaleData = LocaleData {
    tag: "th",
    direction: Direction::LeftToRight,
    numbering: "latn",
    first_day_of_week: Weekday::Sunday,
    casing: CasingStyle::Standard,
    capitalises_month_names: false,
    weekdays: ContextualNames::same(weekday_widths(
        &[
            "วันจันทร์",
            "วันอังคาร",
            "วันพุธ",
            "วันพฤหัสบดี",
            "วันศุกร์",
            "วันเสาร์",
            "วันอาทิตย์",
        ],
        &["จ.", "อ.", "พ.", "พฤ.", "ศ.", "ส.", "อา."],
        &[],
        &["จ", "อ", "พ", "พฤ", "ศ", "ส", "อา"],
    )),
    day_periods: ContextualNames::same(widths(&["ก่อนเที่ยง", "หลังเที่ยง"], &[], &[])),
    cycle: SexagenaryNames::EMPTY,
    calendars: &[
        gregorian(
            &[month_cycle(ContextualNames::same(widths(
                &[
                    "มกราคม",
                    "กุมภาพันธ์",
                    "มีนาคม",
                    "เมษายน",
                    "พฤษภาคม",
                    "มิถุนายน",
                    "กรกฎาคม",
                    "สิงหาคม",
                    "กันยายน",
                    "ตุลาคม",
                    "พฤศจิกายน",
                    "ธันวาคม",
                ],
                &[
                    "ม.ค.",
                    "ก.พ.",
                    "มี.ค.",
                    "เม.ย.",
                    "พ.ค.",
                    "มิ.ย.",
                    "ก.ค.",
                    "ส.ค.",
                    "ก.ย.",
                    "ต.ค.",
                    "พ.ย.",
                    "ธ.ค.",
                ],
                &[],
            )))],
            gregorian_eras(&["ปีก่อนคริสต์ศักราช", "คริสต์ศักราช"], &["ก่อน ค.ศ.", "ค.ศ."], &[]),
            ContextualNames::EMPTY,
        ),
        CalendarNames {
            calendars: BUDDHIST_CALENDARS,
            cycles: &[],
            leap_month_prefix: "",
            eras: EraNames {
                codes: &["be"],
                names: widths(&["พุทธศักราช"], &["พ.ศ."], &[]),
                calendars: &[],
            },
            quarters: ContextualNames::EMPTY,
        },
    ],
};

// --- Turkish --------------------------------------------------------------

const TR: LocaleData = LocaleData {
    tag: "tr",
    direction: Direction::LeftToRight,
    numbering: "latn",
    first_day_of_week: Weekday::Monday,
    // Turkish is the reason `casing` exists: İ and ı are separate letters.
    casing: CasingStyle::Turkic,
    capitalises_month_names: true,
    weekdays: ContextualNames::same(weekday_widths(
        &[
            "Pazartesi",
            "Salı",
            "Çarşamba",
            "Perşembe",
            "Cuma",
            "Cumartesi",
            "Pazar",
        ],
        &["Pzt", "Sal", "Çar", "Per", "Cum", "Cmt", "Paz"],
        &[],
        &["P", "S", "Ç", "P", "C", "C", "P"],
    )),
    day_periods: ContextualNames::same(widths(&["ÖÖ", "ÖS"], &[], &[])),
    cycle: SexagenaryNames::EMPTY,
    calendars: &[gregorian(
        &[month_cycle(ContextualNames::same(widths(
            &[
                "Ocak", "Şubat", "Mart", "Nisan", "Mayıs", "Haziran", "Temmuz", "Ağustos", "Eylül",
                "Ekim", "Kasım", "Aralık",
            ],
            &[
                "Oca", "Şub", "Mar", "Nis", "May", "Haz", "Tem", "Ağu", "Eyl", "Eki", "Kas", "Ara",
            ],
            &["O", "Ş", "M", "N", "M", "H", "T", "A", "E", "E", "K", "A"],
        )))],
        gregorian_eras(&["Milattan Önce", "Milattan Sonra"], &["MÖ", "MS"], &[]),
        ContextualNames::EMPTY,
    )],
};

// --- Vietnamese -----------------------------------------------------------

const VI: LocaleData = LocaleData {
    tag: "vi",
    direction: Direction::LeftToRight,
    numbering: "latn",
    first_day_of_week: Weekday::Monday,
    casing: CasingStyle::Standard,
    capitalises_month_names: false,
    weekdays: ContextualNames::same(weekday_widths(
        &[
            "Thứ Hai",
            "Thứ Ba",
            "Thứ Tư",
            "Thứ Năm",
            "Thứ Sáu",
            "Thứ Bảy",
            "Chủ Nhật",
        ],
        &["Th 2", "Th 3", "Th 4", "Th 5", "Th 6", "Th 7", "CN"],
        &[],
        &["T2", "T3", "T4", "T5", "T6", "T7", "CN"],
    )),
    day_periods: ContextualNames::same(widths(&["SA", "CH"], &[], &[])),
    cycle: SexagenaryNames {
        reading: Some(&readings::VIETNAMESE),
        // Not the Chinese animals: 丑 is the buffalo and 卯 the cat.
        zodiac: Some(&[
            "Chuột", "Trâu", "Hổ", "Mèo", "Rồng", "Rắn", "Ngựa", "Dê", "Khỉ", "Gà", "Chó", "Lợn",
        ]),
    },
    calendars: &[gregorian(
        &[month_cycle(ContextualNames::same(widths(
            &[
                "tháng 1",
                "tháng 2",
                "tháng 3",
                "tháng 4",
                "tháng 5",
                "tháng 6",
                "tháng 7",
                "tháng 8",
                "tháng 9",
                "tháng 10",
                "tháng 11",
                "tháng 12",
            ],
            &[
                "thg 1", "thg 2", "thg 3", "thg 4", "thg 5", "thg 6", "thg 7", "thg 8", "thg 9",
                "thg 10", "thg 11", "thg 12",
            ],
            &[],
        )))],
        gregorian_eras(
            &["Trước Công Nguyên", "Sau Công Nguyên"],
            &["TCN", "SCN"],
            &[],
        ),
        ContextualNames::EMPTY,
    )],
};

// --- Yucatec Maya ---------------------------------------------------------
//
// The language of the Maya counts. CLDR has no `yua` locale, so this entry
// has no Gregorian vocabulary, weekdays, day periods or eras. The twenty
// day-signs and the nineteen haabʼ months are the forms
// `hc_calendars_regional::maya` declares with its shape: the
// sixteenth-century Yucatec spelling, as Reingold and Dershowitz print it
// (*Calendrical Calculations*, 4th ed., 2018, chapter 11) and as
// Wikipedia, "Tzolkʼin" and "Maya calendar", tabulate it beside the revised
// orthography, which is not carried; docs/systems/mesoamerican-counts.md
// sets the sources out. The locale restates them so that the language,
// and not only the calendar, claims them, keyed to the calendars of both
// correlation constants alike, since a name does not depend on the
// constant. The week begins on Sunday (CLDR 48 `weekData/firstDay`, MX and
// GT alike) and numbering is `latn`.

const YUA_DAY_SIGNS: &[&str] = &[
    "Imix", "Ik", "Akbal", "Kan", "Chicchan", "Cimi", "Manik", "Lamat", "Muluc", "Oc", "Chuen",
    "Eb", "Ben", "Ix", "Men", "Cib", "Caban", "Etznab", "Cauac", "Ahau",
];

const YUA_HAAB_MONTHS: &[&str] = &[
    "Pop", "Uo", "Zip", "Zotz", "Tzec", "Xul", "Yaxkin", "Mol", "Chen", "Yax", "Zac", "Ceh", "Mac",
    "Kankin", "Muan", "Pax", "Kayab", "Cumku", "Uayeb",
];

const YUA_CALENDARS: &[CalendarNames] = &[
    gregorian(&[], EraNames::EMPTY, ContextualNames::EMPTY),
    CalendarNames {
        calendars: &[CalendarId("maya-tzolkin"), CalendarId("maya-tzolkin-gmt2")],
        cycles: &[cycle("day-sign", YUA_DAY_SIGNS)],
        leap_month_prefix: "",
        eras: EraNames::EMPTY,
        quarters: ContextualNames::EMPTY,
    },
    dated(
        &[CalendarId("maya-haab"), CalendarId("maya-haab-gmt2")],
        &[months(YUA_HAAB_MONTHS)],
        &[],
        &[],
    ),
    CalendarNames {
        calendars: &[CalendarId("maya-round"), CalendarId("maya-round-gmt2")],
        cycles: &[cycle("day-sign", YUA_DAY_SIGNS), months(YUA_HAAB_MONTHS)],
        leap_month_prefix: "",
        eras: EraNames::EMPTY,
        quarters: ContextualNames::EMPTY,
    },
];

const YUA: LocaleData = LocaleData {
    tag: "yua",
    direction: Direction::LeftToRight,
    numbering: "latn",
    first_day_of_week: Weekday::Sunday,
    casing: CasingStyle::Standard,
    capitalises_month_names: true,
    weekdays: ContextualNames::EMPTY,
    day_periods: ContextualNames::EMPTY,
    cycle: SexagenaryNames::EMPTY,
    calendars: YUA_CALENDARS,
};

// --- Standard Moroccan Tamazight ------------------------------------------
//
// A language of the Berber agrarian calendar, in Tifinagh. Gregorian
// vocabulary from CLDR 48 `common/main/zgh.xml`: the wide, abbreviated and
// stand-alone narrow months, the wide and abbreviated weekdays, the day
// periods (whose wide form resolves to the abbreviation), the eras and the
// quarters. The months are the Latin-derived names — ⵉⵏⵏⴰⵢⵔ from Januarius,
// as Yennayer is — and the agrarian calendar is the Julian year under
// exactly those names (the Encyclopédie berbère's "Calendrier", 1992,
// which `hc_calendars_solar::berber` cites), so the same list is keyed to
// `berber`. Wikipedia's "Berber calendar" prints no Tifinagh forms; these
// are CLDR's, in the standard Moroccan orthography rather than the Kabyle
// one the calendar declares, and the locale's forms are the override. The
// week begins on Monday (CLDR 48 `weekData/firstDay` lists MA under no
// exception) and the numbering system is `latn`.

const ZGH_MONTHS: &[&str] = &[
    "ⵉⵏⵏⴰⵢⵔ",
    "ⴱⵕⴰⵢⵕ",
    "ⵎⴰⵕⵚ",
    "ⵉⴱⵔⵉⵔ",
    "ⵎⴰⵢⵢⵓ",
    "ⵢⵓⵏⵢⵓ",
    "ⵢⵓⵍⵢⵓⵣ",
    "ⵖⵓⵛⵜ",
    "ⵛⵓⵜⴰⵏⴱⵉⵔ",
    "ⴽⵜⵓⴱⵔ",
    "ⵏⵓⵡⴰⵏⴱⵉⵔ",
    "ⴷⵓⵊⴰⵏⴱⵉⵔ",
];

const ZGH_MONTHS_ABBREVIATED: &[&str] = &[
    "ⵉⵏⵏ",
    "ⴱⵕⴰ",
    "ⵎⴰⵕ",
    "ⵉⴱⵔ",
    "ⵎⴰⵢ",
    "ⵢⵓⵏ",
    "ⵢⵓⵍ",
    "ⵖⵓⵛ",
    "ⵛⵓⵜ",
    "ⴽⵜⵓ",
    "ⵏⵓⵡ",
    "ⴷⵓⵊ",
];

const ZGH_MONTHS_NARROW: &[&str] = &["ⵉ", "ⴱ", "ⵎ", "ⵉ", "ⵎ", "ⵢ", "ⵢ", "ⵖ", "ⵛ", "ⴽ", "ⵏ", "ⴷ"];

const ZGH_CALENDARS: &[CalendarNames] = &[
    gregorian(
        &[month_cycle(ContextualNames::same(widths(
            ZGH_MONTHS,
            ZGH_MONTHS_ABBREVIATED,
            ZGH_MONTHS_NARROW,
        )))],
        gregorian_eras(&["ⴷⴰⵜ ⵏ ⵄⵉⵙⴰ", "ⴷⴼⴼⵉⵔ ⵏ ⵄⵉⵙⴰ"], &["ⴷⴰⵄ", "ⴷⴼⵄ"], &[]),
        ContextualNames::same(widths(
            &["ⴰⴽⵕⴰⴹⵢⵓⵔ 1", "ⴰⴽⵕⴰⴹⵢⵓⵔ 2", "ⴰⴽⵕⴰⴹⵢⵓⵔ 3", "ⴰⴽⵕⴰⴹⵢⵓⵔ 4"],
            &["ⴰⴽ 1", "ⴰⴽ 2", "ⴰⴽ 3", "ⴰⴽ 4"],
            &[],
        )),
    ),
    dated(
        &[CalendarId("berber")],
        &[month_cycle(ContextualNames::same(widths(
            ZGH_MONTHS,
            ZGH_MONTHS_ABBREVIATED,
            ZGH_MONTHS_NARROW,
        )))],
        &[],
        &[],
    ),
];

const ZGH: LocaleData = LocaleData {
    tag: "zgh",
    direction: Direction::LeftToRight,
    numbering: "latn",
    first_day_of_week: Weekday::Monday,
    casing: CasingStyle::Standard,
    capitalises_month_names: false,
    weekdays: ContextualNames::same(weekday_widths(
        &[
            "ⴰⵢⵏⴰⵙ",
            "ⴰⵙⵉⵏⴰⵙ",
            "ⴰⴽⵕⴰⵙ",
            "ⴰⴽⵡⴰⵙ",
            "ⴰⵙⵉⵎⵡⴰⵙ",
            "ⴰⵙⵉⴹⵢⴰⵙ",
            "ⴰⵙⴰⵎⴰⵙ",
        ],
        &["ⴰⵢⵏ", "ⴰⵙⵉ", "ⴰⴽⵕ", "ⴰⴽⵡ", "ⴰⵙⵉⵎ", "ⴰⵙⵉⴹ", "ⴰⵙⴰ"],
        &[],
        &[],
    )),
    day_periods: ContextualNames::same(widths(&["ⵜⵉⴼⴰⵡⵜ", "ⵜⴰⴷⴳⴳⵯⴰⵜ"], &[], &[])),
    cycle: SexagenaryNames::EMPTY,
    calendars: ZGH_CALENDARS,
};

// --- Chinese, simplified --------------------------------------------------

const ZH_HANS_CALENDARS: &[CalendarNames] = &[
    gregorian(
        &[month_cycle(ContextualNames::same(widths(
            &[
                "一月",
                "二月",
                "三月",
                "四月",
                "五月",
                "六月",
                "七月",
                "八月",
                "九月",
                "十月",
                "十一月",
                "十二月",
            ],
            &[
                "1月", "2月", "3月", "4月", "5月", "6月", "7月", "8月", "9月", "10月", "11月",
                "12月",
            ],
            &[
                "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12",
            ],
        )))],
        gregorian_eras(&["公元前", "公元"], &[], &[]),
        ContextualNames::same(widths(
            &["第一季度", "第二季度", "第三季度", "第四季度"],
            &["1季度", "2季度", "3季度", "4季度"],
            &[],
        )),
    ),
    lunisolar(
        NUMBERED_LUNISOLAR_CALENDARS,
        &[months(&[
            "正月",
            "二月",
            "三月",
            "四月",
            "五月",
            "六月",
            "七月",
            "八月",
            "九月",
            "十月",
            "十一月",
            "腊月",
        ])],
        "闰",
    ),
];

const ZH_HANS: LocaleData = LocaleData {
    tag: "zh-Hans",
    direction: Direction::LeftToRight,
    numbering: "latn",
    first_day_of_week: Weekday::Sunday,
    casing: CasingStyle::Standard,
    capitalises_month_names: false,
    weekdays: ContextualNames::same(weekday_widths(
        &[
            "星期一",
            "星期二",
            "星期三",
            "星期四",
            "星期五",
            "星期六",
            "星期日",
        ],
        &["周一", "周二", "周三", "周四", "周五", "周六", "周日"],
        &[],
        &["一", "二", "三", "四", "五", "六", "日"],
    )),
    day_periods: ContextualNames::same(widths(&["上午", "下午"], &[], &[])),
    cycle: SexagenaryNames {
        reading: Some(&readings::HAN),
        zodiac: Some(&[
            "鼠", "牛", "虎", "兔", "龙", "蛇", "马", "羊", "猴", "鸡", "狗", "猪",
        ]),
    },
    calendars: ZH_HANS_CALENDARS,
};

// --- Chinese, traditional -------------------------------------------------

const ZH_HANT_CALENDARS: &[CalendarNames] = &[
    gregorian(
        &[month_cycle(ContextualNames::same(widths(
            &[
                "1月", "2月", "3月", "4月", "5月", "6月", "7月", "8月", "9月", "10月", "11月",
                "12月",
            ],
            &[],
            &[
                "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12",
            ],
        )))],
        gregorian_eras(&["西元前", "西元"], &[], &[]),
        ContextualNames::same(widths(&["第1季", "第2季", "第3季", "第4季"], &[], &[])),
    ),
    lunisolar(
        NUMBERED_LUNISOLAR_CALENDARS,
        &[months(&[
            "正月",
            "二月",
            "三月",
            "四月",
            "五月",
            "六月",
            "七月",
            "八月",
            "九月",
            "十月",
            "十一月",
            "臘月",
        ])],
        "閏",
    ),
];

const ZH_HANT: LocaleData = LocaleData {
    tag: "zh-Hant",
    direction: Direction::LeftToRight,
    numbering: "latn",
    first_day_of_week: Weekday::Sunday,
    casing: CasingStyle::Standard,
    capitalises_month_names: false,
    weekdays: ContextualNames::same(weekday_widths(
        &[
            "星期一",
            "星期二",
            "星期三",
            "星期四",
            "星期五",
            "星期六",
            "星期日",
        ],
        &["週一", "週二", "週三", "週四", "週五", "週六", "週日"],
        &[],
        &["一", "二", "三", "四", "五", "六", "日"],
    )),
    day_periods: ContextualNames::same(widths(&["上午", "下午"], &[], &[])),
    cycle: SexagenaryNames {
        reading: Some(&readings::HAN),
        zodiac: Some(&[
            "鼠", "牛", "虎", "兔", "龍", "蛇", "馬", "羊", "猴", "雞", "狗", "豬",
        ]),
    },
    calendars: ZH_HANT_CALENDARS,
};

/// Every locale this crate ships, in tag order.
///
/// The root entry is not in this table: it is [`ROOT`], the floor that the
/// lookup falls to when nothing here claims the locale.
pub static LOCALES: &[LocaleData] = &[
    AM, AR, BAN, BN, BO, COP, CS, DE, EN, ES, FA, FR, HE, HI, ID, IT, JA, JV, KAB, KO, MID, ML, MY,
    NAH, NE, NL, PL, PT, RU, SA, SYR, TA, TH, TR, VI, YUA, ZGH, ZH_HANS, ZH_HANT,
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::locale::Locale;
    use crate::names::{NameContext, NameWidth};
    use alloc::string::ToString as _;
    use alloc::vec::Vec;

    fn every_entry() -> Vec<&'static LocaleData> {
        let mut entries: Vec<&'static LocaleData> = LOCALES.iter().collect();
        entries.push(&ROOT);
        entries
    }

    fn every_width_set(data: &'static LocaleData) -> Vec<(&'static str, &'static WidthSet)> {
        let mut sets: Vec<(&'static str, &'static WidthSet)> = alloc::vec![
            ("weekdays/format", &data.weekdays.format),
            ("weekdays/standalone", &data.weekdays.standalone),
            ("dayperiods/format", &data.day_periods.format),
            ("dayperiods/standalone", &data.day_periods.standalone),
        ];
        for calendar in data.calendars {
            for cycle in calendar.cycles {
                sets.push((cycle.kind, &cycle.names.format));
                sets.push((cycle.kind, &cycle.names.standalone));
            }
            sets.push(("quarters/format", &calendar.quarters.format));
            sets.push(("quarters/standalone", &calendar.quarters.standalone));
            sets.push(("eras", &calendar.eras.names));
        }
        sets
    }

    #[test]
    fn every_tag_is_canonical_and_the_table_is_sorted() {
        for pair in LOCALES.windows(2) {
            assert!(
                pair[0].tag < pair[1].tag,
                "{} !< {}",
                pair[0].tag,
                pair[1].tag
            );
        }
        for data in every_entry() {
            let locale =
                Locale::parse(data.tag).unwrap_or_else(|error| panic!("{}: {error}", data.tag));
            assert_eq!(
                locale.to_string(),
                data.tag,
                "{} is not canonical",
                data.tag
            );
        }
    }

    #[test]
    fn no_name_anywhere_is_empty_or_padded() {
        for data in every_entry() {
            for (label, set) in every_width_set(data) {
                for width in NameWidth::ALL {
                    for name in set.exact(width) {
                        assert!(!name.is_empty(), "{} {label}: empty name", data.tag);
                        assert_eq!(
                            *name,
                            name.trim(),
                            "{} {label}: padded name {name:?}",
                            data.tag
                        );
                    }
                }
            }
            // The stems and branches are a reading `hc-calendar` owns and
            // checks; only the animals are this crate's.
            for name in data.cycle.zodiac.into_iter().flatten() {
                assert!(!name.is_empty(), "{}: empty zodiac name", data.tag);
            }
        }
    }

    #[test]
    fn every_weekday_list_has_seven_entries() {
        for data in every_entry() {
            for width in NameWidth::ALL {
                for context in [NameContext::Format, NameContext::Standalone] {
                    let names = match context {
                        NameContext::Format => data.weekdays.format.exact(width),
                        NameContext::Standalone => data.weekdays.standalone.exact(width),
                    };
                    assert!(
                        names.is_empty() || names.len() == 7,
                        "{}: {} weekday names",
                        data.tag,
                        names.len()
                    );
                }
            }
            // A locale that states no weekday at all inherits them — Coptic,
            // whose sources name no week. One that states any must state
            // the wide form.
            if data.weekdays.is_empty() {
                continue;
            }
            assert_eq!(
                data.weekdays.format.wide.len(),
                7,
                "{}: no wide weekday names",
                data.tag
            );
        }
    }

    /// Every width of a cycle must carry the same number of names.
    ///
    /// A fixed expected length — twelve or thirteen months — would be the
    /// wrong test: it would reject the Badíʿ calendar's nineteen months and
    /// the Maya Haabʼ's nineteen, correct data included.
    ///
    /// What is actually invariant is internal consistency — a calendar that
    /// names twelve months wide must name twelve abbreviated — and
    /// agreement with the calendar's own declared shape, which
    /// `hyper-calendar`'s vocabulary test checks because only there are the
    /// registry and the locale data both in scope.
    #[test]
    fn every_width_of_a_cycle_has_the_same_number_of_names() {
        for data in every_entry() {
            for calendar in data.calendars {
                for cycle in calendar.cycles {
                    for set in [&cycle.names.format, &cycle.names.standalone] {
                        let mut expected: Option<usize> = None;
                        for width in NameWidth::ALL {
                            let names = set.exact(width);
                            if names.is_empty() {
                                continue;
                            }
                            match expected {
                                None => expected = Some(names.len()),
                                Some(length) => assert_eq!(
                                    names.len(),
                                    length,
                                    "{} {}: {:?} has {} names, another width has {length}",
                                    data.tag,
                                    cycle.kind,
                                    width,
                                    names.len()
                                ),
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn every_quarter_and_day_period_list_is_the_right_length() {
        for data in every_entry() {
            for width in NameWidth::ALL {
                for set in [&data.day_periods.format, &data.day_periods.standalone] {
                    let names = set.exact(width);
                    assert!(
                        names.is_empty() || names.len() == 2,
                        "{}: {} day periods",
                        data.tag,
                        names.len()
                    );
                }
                for calendar in data.calendars {
                    for set in [&calendar.quarters.format, &calendar.quarters.standalone] {
                        let names = set.exact(width);
                        assert!(
                            names.is_empty() || names.len() == 4,
                            "{} {:?}: {} quarters",
                            data.tag,
                            calendar.calendars,
                            names.len()
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn era_codes_and_era_names_stay_parallel() {
        for data in every_entry() {
            for calendar in data.calendars {
                let eras = &calendar.eras;
                for width in NameWidth::ALL {
                    let names = eras.names.exact(width);
                    assert!(
                        names.is_empty() || names.len() == eras.codes.len(),
                        "{} {:?}: {} era names for {} codes",
                        data.tag,
                        calendar.calendars,
                        names.len(),
                        eras.codes.len()
                    );
                }
                if !eras.codes.is_empty() {
                    assert!(
                        !eras.names.wide.is_empty(),
                        "{} {:?}: era codes with no names",
                        data.tag,
                        calendar.calendars
                    );
                }
            }
        }
    }

    /// No two entries may carry the *same cycle* for one calendar.
    ///
    /// Several entries serving one calendar is deliberate — Thai's
    /// Buddhist calendar takes its months from the shared Gregorian entry
    /// and its era from an entry of its own. What must not happen is two
    /// entries both claiming to name its months, because then the lookup
    /// silently takes whichever comes first.
    #[test]
    fn no_calendar_has_one_cycle_named_twice_in_a_locale() {
        for data in every_entry() {
            for entry in data.calendars {
                for id in entry.calendars {
                    for kind in ["month", "weekday", "decade-day"] {
                        let carriers = data
                            .entries_for(*id)
                            .filter(|candidate| !candidate.cycle(kind).is_empty())
                            .count();
                        assert!(
                            carriers <= 1,
                            "{}: {} has {carriers} entries naming its {kind}s",
                            data.tag,
                            id.0
                        );
                    }
                    let eras = data
                        .entries_for(*id)
                        .filter(|candidate| {
                            !candidate.eras.codes.is_empty()
                                && candidate.eras.belongs_to(*id, candidate.calendars)
                        })
                        .count();
                    assert!(eras <= 1, "{}: {} has {eras} era lists", data.tag, id.0);
                }
            }
        }
    }

    #[test]
    fn a_locale_that_writes_the_cycle_also_names_its_animals() {
        // Ten stems and twelve animals are the types' business now; what is
        // left to check is that no locale can say 甲辰 and not "dragon".
        for data in every_entry() {
            assert_eq!(
                data.cycle.reading.is_some(),
                data.cycle.zodiac.is_some(),
                "{}: a reading without animals, or animals without a reading",
                data.tag
            );
        }
    }

    #[test]
    fn every_locale_names_a_numbering_system_this_crate_has() {
        for data in every_entry() {
            assert!(
                crate::numbering::NumberingSystem::from_id(data.numbering).is_some(),
                "{}: unknown numbering system {}",
                data.tag,
                data.numbering
            );
        }
    }

    #[test]
    fn every_locale_states_the_gregorian_calendar() {
        for data in every_entry() {
            assert!(
                data.calendar(CalendarId("gregory")).is_some(),
                "{}: no gregorian vocabulary",
                data.tag
            );
        }
    }

    #[test]
    fn every_month_name_differs_from_its_neighbours_within_a_list() {
        // September and Září are the same word in Czech in both contexts, so
        // the check is on adjacent months, which no language conflates.
        for data in every_entry() {
            for calendar in data.calendars {
                let months = calendar.months();
                for set in [&months.format, &months.standalone] {
                    for width in NameWidth::ALL {
                        let names = set.exact(width);
                        for pair in names.windows(2) {
                            if width == NameWidth::Narrow {
                                continue;
                            }
                            assert_ne!(
                                pair[0], pair[1],
                                "{} {:?}: adjacent months share a name",
                                data.tag, calendar.calendars
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn the_right_to_left_locales_are_the_ones_with_right_to_left_scripts() {
        let rtl: Vec<&str> = every_entry()
            .iter()
            .filter(|data| data.direction == Direction::RightToLeft)
            .map(|data| data.tag)
            .collect();
        assert_eq!(rtl, ["ar", "fa", "he", "mid", "syr"]);
    }

    #[test]
    fn the_shipped_locale_set_is_the_documented_one() {
        let tags: Vec<&str> = LOCALES.iter().map(|data| data.tag).collect();
        assert_eq!(
            tags,
            [
                "am", "ar", "ban", "bn", "bo", "cop", "cs", "de", "en", "es", "fa", "fr", "he",
                "hi", "id", "it", "ja", "jv", "kab", "ko", "mid", "ml", "my", "nah", "ne", "nl",
                "pl", "pt", "ru", "sa", "syr", "ta", "th", "tr", "vi", "yua", "zgh", "zh-Hans",
                "zh-Hant"
            ]
        );
        assert_eq!(ROOT.tag.to_string(), "und");
    }
}
