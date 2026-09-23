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
//!   Hebrew and English, the Chinese lunisolar months in both Chinese
//!   scripts, their Japanese traditional names, and the Japanese era names.
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
/// so one vocabulary entry serves all of them. This replaces the alias
/// function that used to map identifiers onto `gregory` — and two of the
/// identifiers that function listed, `iso8601` and `roc`, were not
/// registry identifiers at all, so their entries were inert.
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
];

/// A month cycle from names already shaped into widths and contexts.
const fn month_cycle(names: ContextualNames) -> CycleNames {
    CycleNames::new(hc_calendar::shape::MONTH, names)
}

/// A month cycle from a plain list of names, the same in every width.
const fn months(names: &'static [&'static str]) -> CycleNames {
    month_cycle(ContextualNames::same(widths(names, &[], &[])))
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
/// The registry calls it `persian-arithmetic`. This data used to key it as
/// `persian`, a name no calendar answered to, so the twelve Persian month
/// names below were written, tested and unreachable.
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
    calendars: &[gregorian(
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
    )],
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
    AR, CS, DE, EN, ES, FA, FR, HE, HI, ID, IT, JA, KO, NL, PL, PT, RU, TH, TR, VI, ZH_HANS,
    ZH_HANT,
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
    /// This replaces a test that asserted every month list had twelve or
    /// thirteen entries. That assertion was not a safety net, it was the
    /// hole: it made the Badíʿ calendar's nineteen months and the Maya
    /// Haabʼ's nineteen *rejectable* rather than merely absent, so the data
    /// could not have been added even by someone willing to write it.
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
        assert_eq!(rtl, ["ar", "fa", "he"]);
    }

    #[test]
    fn the_shipped_locale_set_is_the_documented_one() {
        let tags: Vec<&str> = LOCALES.iter().map(|data| data.tag).collect();
        assert_eq!(
            tags,
            [
                "ar", "cs", "de", "en", "es", "fa", "fr", "he", "hi", "id", "it", "ja", "ko", "nl",
                "pl", "pt", "ru", "th", "tr", "vi", "zh-Hans", "zh-Hant"
            ]
        );
        assert_eq!(ROOT.tag.to_string(), "und");
    }
}
