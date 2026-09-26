//! The locale data itself.
//!
//! Everything in this module is `&'static` data and nothing in it is logic.
//! One language is one [`LocaleData`] value plus one line in [`LOCALES`];
//! the lookup in [`crate::names`] does not know which languages exist and
//! gains no branch when a new one arrives.
//!
//! # Provenance
//!
//! The vocabulary follows Unicode CLDR 48, the `common/main/<locale>.xml`
//! `calendars` sections at tag `release-48` (`cldr48-main`), and is
//! hand-checked, not generated: it is a subset chosen for calendar work.
//! Each entry's [`LocaleData::sources`] names the file it follows; on
//! 2026-09-26 the Gregorian months and weekdays of the 35 entries that
//! follow CLDR were compared with those files. An entry carries its
//! language's own file and CLDR's default (non-`alt`) values: regional
//! files are not carried, so Arabic has `ar.xml`'s يناير… and not the
//! كانون الثاني… of `ar_SY.xml` and the other Levantine files. Where an
//! entry departs from its file, its comment says so. Fields a locale does
//! not state are left empty on purpose, because an empty field inherits,
//! and inheriting is more correct than copying.
//!
//! What that means in practice:
//!
//! * A locale entry covers months, weekdays, day periods, eras and — for
//!   some locales — quarters. Finer CLDR day periods, date patterns,
//!   interval patterns and relative-time strings are not here.
//! * Non-Gregorian vocabulary is carried for the calendars where the names
//!   genuinely differ: Hijri months in Arabic and English, Hebrew months in
//!   Hebrew and English, Babylonian months in English, the Chinese
//!   calendar's months in both Chinese scripts, in Japanese and in Korean,
//!   the Japanese lunisolar calendars' traditional month names, and the
//!   Japanese era names. A family's names serve that family: 師走 answers
//!   for the Tenpō calendar and never for the Chinese one.
//! * Each locale states its name in English and in itself, its script, and
//!   how it writes a year with its era, a day and a whole date — the
//!   templates `hc-format` renders — and what it calls the calendars it has
//!   a word for. Every template names the CLDR pattern it was read from.
//! * Locales exist for a calendar's own language: Amharic for the Ethiopic
//!   calendar, Coptic for the Coptic, Burmese for the Burmese, Tibetan for
//!   the Tibetan, Nepali for the Bikram Sambat and Nepal Sambat, Sanskrit
//!   and Hindi for the Hindu lunisolar and Vikrami solar calendars, Tamil,
//!   Malayalam and Bengali for the regional solar calendars, Yucatec Maya
//!   and Nahuatl for the Mesoamerican counts, Zapotec for the Zapotec year,
//!   Balinese and Javanese for the Pawukon and the pasaran, Syriac for the
//!   Assyrian calendar, Kabyle and Standard Moroccan Tamazight for the
//!   Berber calendar, Mandaic for the Mandaean, and Pashto for the Solar
//!   Hijri calendar as Afghanistan kept it. Their Gregorian
//!   vocabulary is CLDR's where CLDR has the locale (CLDR 48,
//!   `common/main/<locale>.xml`, read 2026-09-25 and 2026-09-26 from the
//!   `release-48` tag of github.com/unicode-org/cldr); CLDR has no `cop`,
//!   `ban`, `yua`, `nah`, `zap` or `mid`, so those entries state only what
//!   the calendars' own sources say. Where a CLDR value is the
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
    CalendarDisplayName, CalendarNames, ContextualNames, CycleNames, DateTemplates, EraNames,
    LeapMonthNames, LocaleData, SexagenaryNames, WidthSet,
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
    CalendarId("japanese-northern-proclaimed"),
    CalendarId("japanese-southern-proclaimed"),
    CalendarId("roman-auc"),
    CalendarId("byzantine"),
    CalendarId("symmetry454"),
    CalendarId("symmetry010"),
    CalendarId("world-calendar"),
    CalendarId("julian-gregorian-catholic"),
    CalendarId("julian-gregorian-fr"),
    CalendarId("julian-gregorian-nl-states-general"),
    CalendarId("julian-gregorian-nl-holland"),
    CalendarId("julian-gregorian-de-catholic"),
    CalendarId("julian-gregorian-hu"),
    CalendarId("julian-gregorian-de-protestant"),
    CalendarId("julian-gregorian-gb"),
    CalendarId("julian-gregorian-se"),
    CalendarId("julian-gregorian-bg"),
    CalendarId("julian-gregorian-ru"),
    CalendarId("julian-gregorian-rs"),
    CalendarId("julian-gregorian-ro"),
    CalendarId("julian-gregorian-gr"),
    CalendarId("swedish-1700"),
    CalendarId("hanke-henry"),
    CalendarId("soviet-week"),
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
/// so share one set of English ordinals: First Month, Second Month.
///
/// The Mongolian calendar is here by Janson's numbers. Its months are
/// named rather than numbered, by season and by animal, month 1 being the
/// first month of spring (Janson, "Tibetan calendar mathematics", 2014,
/// Appendix A.3) — the "хаврын тэргүүн сар" of the Mongolian holiday law —
/// and those names are not carried.
///
/// English is the one locale that serves all of them from a single list,
/// because an ordinal is not anybody's word for a month. A locale with
/// words of its own states them for one family at a time —
/// [`CHINESE_FAMILY_CALENDARS`] or [`JAPANESE_LUNISOLAR_CALENDARS`] — so
/// that 師走 never answers for the Chinese calendar and 腊月 never for the
/// Tenpō.
const NUMBERED_LUNISOLAR_CALENDARS: &[CalendarId] = &[
    CalendarId("chinese"),
    CalendarId("chinese-regnal"),
    CalendarId("tibetan"),
    CalendarId("tibetan-tsurphu"),
    CalendarId("tibetan-bhutan"),
    CalendarId("mongolian"),
    CalendarId("dangi"),
    CalendarId("vietnamese"),
    CalendarId("japanese-tenpo"),
    CalendarId("japanese-kansei"),
    CalendarId("japanese-horyaku"),
    CalendarId("japanese-jokyo"),
    CalendarId("japanese-senmyo"),
];

/// The calendars of the Chinese family that write their year by its stem
/// and branch — 癸卯年 — and their months as 正月, 二月 … 腊月 in Chinese
/// and 正月, 二月 … 十二月 in Japanese: the Chinese calendar itself, the
/// Korean Dangi and the Vietnamese, which share its month structure and
/// its sexagenary count. The Chinese regnal calendar shares the months but
/// writes its year by the reign, so it is served beside these and not
/// among them.
const CHINESE_FAMILY_CALENDARS: &[CalendarId] = &[
    CalendarId("chinese"),
    CalendarId("dangi"),
    CalendarId("vietnamese"),
];

/// The Chinese regnal calendar on its own: the months of the Chinese
/// family, the year of the reign.
const CHINESE_REGNAL_CALENDARS: &[CalendarId] = &[CalendarId("chinese-regnal")];

/// The five Japanese lunisolar calendars, whose months the Japanese
/// traditional names — 睦月 … 師走 — belong to and to nothing else.
const JAPANESE_LUNISOLAR_CALENDARS: &[CalendarId] = &[
    CalendarId("japanese-tenpo"),
    CalendarId("japanese-kansei"),
    CalendarId("japanese-horyaku"),
    CalendarId("japanese-jokyo"),
    CalendarId("japanese-senmyo"),
];

/// The Qumran 364-day year, whose months the scrolls number.
const QUMRAN_CALENDARS: &[CalendarId] = &[CalendarId("qumran")];

/// Palmen's Yerm lunar calendar, whose months are numbered.
const YERM_CALENDARS: &[CalendarId] = &[CalendarId("yerm")];

/// The Solar Hijri calendar.
///
/// The registry has it four times with Iran's month names: the two
/// arithmetic calendars, `persian-arithmetic` (Birashk's cycle) and
/// `persian-arithmetic-33`, and the two astronomical ones of
/// `hc-calendars-equinox`, `persian` and `persian-apparent-noon`. Their
/// months have the same names.
const PERSIAN_CALENDARS: &[CalendarId] = &[
    CalendarId("persian-arithmetic"),
    CalendarId("persian-arithmetic-33"),
    CalendarId("persian"),
    CalendarId("persian-apparent-noon"),
];

/// The Solar Hijri calendar under every name the registry has for it: the
/// four Iranian ones and `persian-afghan`, the same days under the Arabic
/// names of the zodiac signs. They share an era, and CLDR's one `persian`
/// calendar is all of them, so a locale that follows CLDR's month names
/// for it — Pashto — serves all of them too.
const SOLAR_HIJRI_CALENDARS: &[CalendarId] = &[
    CalendarId("persian-arithmetic"),
    CalendarId("persian-arithmetic-33"),
    CalendarId("persian"),
    CalendarId("persian-apparent-noon"),
    CalendarId("persian-afghan"),
];

/// The Solar Hijri calendar as Afghanistan kept it.
const AFGHAN_SOLAR_HIJRI_CALENDARS: &[CalendarId] = &[CalendarId("persian-afghan")];

/// The Coptic calendar.
const COPTIC_CALENDARS: &[CalendarId] = &[CalendarId("coptic")];

/// The Ethiopic calendar.
const ETHIOPIC_CALENDARS: &[CalendarId] = &[CalendarId("ethiopic")];

/// The Burmese calendar.
const BURMESE_CALENDARS: &[CalendarId] = &[CalendarId("burmese")];

/// The Khmer lunar calendar.
const KHMER_CALENDARS: &[CalendarId] = &[CalendarId("khmer")];

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
    CalendarId("japanese-northern-proclaimed"),
    CalendarId("japanese-southern-proclaimed"),
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
        templates: GREGORIAN_TEMPLATES,
        leap_names: LeapMonthNames::NONE,
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
        templates: DateTemplates::NONE,
        leap_names: LeapMonthNames::NONE,
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
        templates: DateTemplates::NONE,
        leap_names: LeapMonthNames::NONE,
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

// --- date templates -------------------------------------------------------
//
// How a locale writes a year with its era, a day of the month and a whole
// date, as `hc-format` renders them. Each convention is read from CLDR 48
// `common/main/<locale>.xml`, `calendar type="gregorian"` (read 2026-09-26
// from the `release-48` tag of github.com/unicode-org/cldr): the `Gy`
// date-format item for the year with its era, `d` for the day and `yMMMMd`
// for the date, with CLDR's `G`, `y`, `MMMM` and `d` read as this crate's
// `{era}`, `{year}`, `{month}` and `{day}`; where the day carries a suffix
// or a point, that is the day template's, so that a date without a day
// loses the point with it. A locale whose patterns were not read (`am`,
// `bo`, `cop`, `my`, `ne`), whose CLDR patterns no template here matches
// yet (`bn`, `jv`, `kab`, `ml`, `syr`, `ta`, `zgh`) or which has no CLDR
// locale (`ban`, `mid`, `nah`, `yua`, `zap`) states none and takes
// `DateTemplates::DEFAULT`.
//
// The era is written wherever the date carries one and the locale has not
// declared it implied: every Gregorian-family entry declares `ad` implied,
// as CLDR's own `yMMMMd` patterns leave the era out and its `Gy` items put
// it back for the years before Christ, so 2026 is *2026* and 44 BC is
// *44 BC*.

/// What the Hebrew entries state: the era of the world is implied.
const HEBREW_TEMPLATES: DateTemplates = DateTemplates {
    implied_era: "am",
    ..DateTemplates::NONE
};

/// What every Gregorian-family entry states: the current era is implied.
const GREGORIAN_TEMPLATES: DateTemplates = DateTemplates {
    implied_era: "ad",
    ..DateTemplates::NONE
};

/// `en.xml`: `Gy` is "y G", `yMMMMd` is "MMMM d, y".
const EN_TEMPLATES: DateTemplates = DateTemplates {
    year: "{year} {era}",
    date: "{month} {day}, {year}",
    ..DateTemplates::NONE
};

/// `fr.xml`, `it.xml`, `nl.xml`, `pl.xml`, `id.xml`, `hi.xml`, `sa.xml`:
/// `Gy` is "y G" and `yMMMMd` is "d MMMM y" in each.
const DAY_MONTH_YEAR_TEMPLATES: DateTemplates = DateTemplates {
    year: "{year} {era}",
    date: "{day} {month} {year}",
    ..DateTemplates::NONE
};

/// `de.xml`: `Gy` is "y G", `d` is "d.", `yMMMMd` is "d. MMMM y".
const DE_TEMPLATES: DateTemplates = DateTemplates {
    year: "{year} {era}",
    day: "{day}.",
    date: "{day} {month} {year}",
    ..DateTemplates::NONE
};

/// `cs.xml`: `Gy` is "y G", `d` is "d.", `yMMMMd` is "d. MMMM y".
const CS_TEMPLATES: DateTemplates = DE_TEMPLATES;

/// `es.xml` and `pt.xml`: `Gy` is "y G", `yMMMMd` is "d 'de' MMMM 'de' y".
const ES_TEMPLATES: DateTemplates = DateTemplates {
    year: "{year} {era}",
    date: "{day} de {month} de {year}",
    ..DateTemplates::NONE
};

/// `ru.xml`: `Gy` is "y G", `yMMMMd` is "d MMMM y 'г'.".
const RU_TEMPLATES: DateTemplates = DateTemplates {
    year: "{year} {era}",
    date: "{day} {month} {year} г.",
    ..DateTemplates::NONE
};

/// `tr.xml`: `Gy` is "G y", `yMMMMd` is "d MMMM y".
const TR_TEMPLATES: DateTemplates = DateTemplates {
    year: "{era} {year}",
    date: "{day} {month} {year}",
    ..DateTemplates::NONE
};

/// `ja.xml`: `Gy` is "Gy年", `d` is "d日", `GyMMMd` is "Gy年M月d日". The
/// first year of an era is 元年, not 1年: CLDR's `jpanyear` numbering
/// system (`supplemental/numberingSystems.xml`, "Japanese first-year Gannen
/// numbering for Japanese calendar") exists for exactly that.
const JA_TEMPLATES: DateTemplates = DateTemplates {
    year: "{era}{year}年",
    first_year: "{era}元年",
    day: "{day}日",
    date: "{year}{month}{day}",
    ..DateTemplates::NONE
};

/// `zh.xml` and `zh_Hant.xml`: `Gy` is "Gy年", `d` is "d日", `yMMMd` is
/// "y年M月d日" — the month by its number, which is the abbreviated form
/// here, since the wide simplified names are 一月 … 十二月.
const ZH_TEMPLATES: DateTemplates = DateTemplates {
    year: "{era}{year}年",
    day: "{day}日",
    date: "{year}{month:abbreviated}{day}",
    ..DateTemplates::NONE
};

/// `ko.xml`: `Gy` is "G y년", `d` is "d일", `yMMMd` is "y년 M월 d일".
const KO_TEMPLATES: DateTemplates = DateTemplates {
    year: "{era} {year}년",
    day: "{day}일",
    date: "{year} {month} {day}",
    ..DateTemplates::NONE
};

/// `he.xml`: `Gy` is "y G", `yMMMMd` is "d בMMMM y".
const HE_TEMPLATES: DateTemplates = DateTemplates {
    year: "{year} {era}",
    date: "{day} ב{month} {year}",
    ..DateTemplates::NONE
};

/// `ar.xml` and `fa.xml`: `Gy` is "y G", `yMMMMd` is "d MMMM y".
const AR_TEMPLATES: DateTemplates = DAY_MONTH_YEAR_TEMPLATES;
const FA_TEMPLATES: DateTemplates = DAY_MONTH_YEAR_TEMPLATES;

/// `th.xml`: `Gy` is "G y", `yMMMMd` is "d MMMM y" — so the Buddhist
/// calendar writes 21 กันยายน พ.ศ. 2569, the era before its year.
const TH_TEMPLATES: DateTemplates = DateTemplates {
    year: "{era} {year}",
    date: "{day} {month} {year}",
    ..DateTemplates::NONE
};

/// `vi.xml`: `Gy` is "y G", `yMMMMd` is "d MMMM, y".
const VI_TEMPLATES: DateTemplates = DateTemplates {
    year: "{year} {era}",
    date: "{day} {month}, {year}",
    ..DateTemplates::NONE
};

/// The Chinese calendar's year and day as Chinese writes them: the year by
/// its stem and branch (CLDR 48 `zh.xml`, `calendar type="chinese"`, whose
/// `y` item is "U年", the cyclic year name) and the day of the month by the
/// `hanidays` numbering system (`supplemental/numberingSystems.xml`,
/// "Han-character day-of-month numbering for lunar/other traditional
/// calendars": 初一 … 三十), with the date "U年MMMd" — 癸卯年闰二月初一.
/// The same characters serve both scripts.
const CHINESE_TEMPLATES: DateTemplates = DateTemplates {
    year: "{sexagenary}年",
    day: "{day}",
    date: "{year}{month}{day}",
    day_names: HANIDAYS,
    ..DateTemplates::NONE
};

/// The thirty day names of CLDR's `hanidays`.
const HANIDAYS: &[&str] = &[
    "初一", "初二", "初三", "初四", "初五", "初六", "初七", "初八", "初九", "初十", "十一", "十二",
    "十三", "十四", "十五", "十六", "十七", "十八", "十九", "二十", "廿一", "廿二", "廿三", "廿四",
    "廿五", "廿六", "廿七", "廿八", "廿九", "三十",
];

/// The Chinese calendar's year in Japanese: by its stem and branch, in
/// the same characters — 癸卯年 — as the National Astronomical Observatory's
/// calendar notes write the 干支 of a year (暦Wiki, eco.mtk.nao.ac.jp/koyomi/wiki,
/// read 2026-09-26); the day stays the locale's `{day}日`.
const JA_CHINESE_TEMPLATES: DateTemplates = DateTemplates {
    year: "{sexagenary}年",
    date: "{year}{month}{day}",
    ..DateTemplates::NONE
};

/// The Dangi year in Korean: by its stem and branch in Hangul, 계묘년
/// (CLDR 48 `ko.xml`, `calendar type="dangi"`, whose `y` item is "U년").
const KO_CHINESE_TEMPLATES: DateTemplates = DateTemplates {
    year: "{sexagenary}년",
    date: "{year} {month} {day}",
    ..DateTemplates::NONE
};

// --- calendar display names ----------------------------------------------
//
// What a locale calls a calendar, from CLDR 48, the `release-48` tag of
// github.com/unicode-org/cldr, `common/main/<locale>.xml`,
// `localeDisplayNames/types/type[@key="calendar"]`, read 2026-09-26 for
// every locale below: `zh.xml` for `zh-Hans`, `zh_Hant.xml` for `zh-Hant`,
// and the file of the same tag for the rest. Keyed to the registry: CLDR's
// `gregorian` is `gregory`, its `iso8601` is `iso8601-week`, its `persian`
// serves both `persian` and `persian-arithmetic`, and its `islamic-civil`,
// `islamic-tbla`, `islamic-umalqura` and `islamic-rgsa` are the registry's
// own. CLDR's bare `islamic` names a tradition rather than one of the four
// tabular and observational conventions here, and its
// `ethiopic-amete-alem` is a calendar the registry does not carry, so
// neither is here. Only the plain value of each type is taken — not an
// `alt` or `scope="core"` variant — and only at CLDR's release levels,
// `approved` and `contributed`: a `provisional` or `unconfirmed` value is
// a proposal the release does not stand behind, and is left out. That
// leaves Coptic and Kabyle, whose every value is unconfirmed, and Tibetan,
// Balinese, Yucatec Maya, Nahuatl and Mandaic, which have none or no file,
// without a table. A calendar a locale has no CLDR name for is left
// unnamed, never filled from another language [cldr48-calendar-names].

const AM_CALENDAR_NAMES: &[CalendarDisplayName] = &[
    CalendarDisplayName::new("buddhist", "የቡዲስት ቀን አቆጣጠር"),
    CalendarDisplayName::new("chinese", "የቻይና የቀን አቆጣጠር"),
    CalendarDisplayName::new("coptic", "የኮፕቲክ የቀን አቆጣጠር"),
    CalendarDisplayName::new("dangi", "የዳንጊ የቀን አቆጣጠር"),
    CalendarDisplayName::new("ethiopic", "የኢትዮጵያ የቀን አቆጣጠር"),
    CalendarDisplayName::new("gregory", "የግሪጎሪያን የቀን አቆጣጠር"),
    CalendarDisplayName::new("hebrew", "የእብራዊያን የቀን አቆጣጠር"),
    CalendarDisplayName::new("indian", "የህንድ ብሔራዊ የቀን አቆጣጠር"),
    CalendarDisplayName::new("islamic-civil", "የሂጅራ የቀን አቆጣጠር (ታቡላር፣ ሲቪል አፖች)"),
    CalendarDisplayName::new("islamic-umalqura", "የሂጅራ የቀን አቆጣጠር (ኡም አል-ቁራ)"),
    CalendarDisplayName::new("iso8601-week", "ISO-8601 የቀን አቆጣጠር"),
    CalendarDisplayName::new("japanese", "የጃፓን የቀን አቆጣጠር"),
    CalendarDisplayName::new("persian", "የፐርሽያ የቀን አቆጣጠር"),
    CalendarDisplayName::new("persian-arithmetic", "የፐርሽያ የቀን አቆጣጠር"),
    CalendarDisplayName::new("roc", "የሚንጉ የቀን አቆጣጠር"),
];

const AR_CALENDAR_NAMES: &[CalendarDisplayName] = &[
    CalendarDisplayName::new("buddhist", "التقويم البوذي"),
    CalendarDisplayName::new("chinese", "التقويم الصيني"),
    CalendarDisplayName::new("coptic", "التقويم القبطي"),
    CalendarDisplayName::new("dangi", "تقويم دانجي"),
    CalendarDisplayName::new("ethiopic", "التقويم الإثيوبي"),
    CalendarDisplayName::new("gregory", "التقويم الميلادي"),
    CalendarDisplayName::new("hebrew", "التقويم العبري"),
    CalendarDisplayName::new("indian", "التقويم القومي الهندي"),
    CalendarDisplayName::new("islamic-civil", "التقويم الهجري المدني"),
    CalendarDisplayName::new("islamic-tbla", "التقويم الهجري (الحسابات الفلكية)"),
    CalendarDisplayName::new("islamic-umalqura", "التقويم الهجري (أم القرى)"),
    CalendarDisplayName::new("islamic-rgsa", "التقويم الهجري (السعودية - الرؤية)"),
    CalendarDisplayName::new("iso8601-week", "تقويم ISO-8601"),
    CalendarDisplayName::new("japanese", "التقويم الياباني"),
    CalendarDisplayName::new("persian", "التقويم الفارسي"),
    CalendarDisplayName::new("persian-arithmetic", "التقويم الفارسي"),
    CalendarDisplayName::new("roc", "تقويم مينجو"),
];

const BN_CALENDAR_NAMES: &[CalendarDisplayName] = &[
    CalendarDisplayName::new("buddhist", "বৌদ্ধ ক্যালেন্ডার"),
    CalendarDisplayName::new("chinese", "চীনা ক্যালেন্ডার"),
    CalendarDisplayName::new("coptic", "কপটিক ক্যালেন্ডার"),
    CalendarDisplayName::new("dangi", "দাঙ্গি ক্যালেন্ডার"),
    CalendarDisplayName::new("ethiopic", "ইথিওপিক ক্যালেন্ডার"),
    CalendarDisplayName::new("gregory", "গ্রিগোরিয়ান ক্যালেন্ডার"),
    CalendarDisplayName::new("hebrew", "হিব্রু ক্যালেন্ডার"),
    CalendarDisplayName::new("indian", "ভারতীয় জাতীয় বর্ষপঞ্জী"),
    CalendarDisplayName::new("islamic-civil", "ইসলামিক ক্যালেন্ডার (ছকবদ্ধ, নাগরিক বর্ষপঞ্জি)"),
    CalendarDisplayName::new("islamic-tbla", "ইসলামিক বর্ষপঞ্জী (ছকবদ্ধ, জ্যোতির্বিদ্যীয় যুগ)"),
    CalendarDisplayName::new("islamic-umalqura", "ইসলামিক বর্ষপঞ্জী (উম্মা আল-কুরআ)"),
    CalendarDisplayName::new("islamic-rgsa", "ইসলামিক বর্ষপঞ্জী (সৌদি আরব, দৃশ্যমান)"),
    CalendarDisplayName::new("iso8601-week", "গ্রেগরীয় ক্যালেন্ডার (প্রথম বর্ষ)"),
    CalendarDisplayName::new("japanese", "জাপানি ক্যালেন্ডার"),
    CalendarDisplayName::new("persian", "ফারসি ক্যালেন্ডার"),
    CalendarDisplayName::new("persian-arithmetic", "ফারসি ক্যালেন্ডার"),
    CalendarDisplayName::new("roc", "মিঙ্গুও ক্যালেন্ডার"),
];

const CS_CALENDAR_NAMES: &[CalendarDisplayName] = &[
    CalendarDisplayName::new("buddhist", "Buddhistický kalendář"),
    CalendarDisplayName::new("chinese", "Čínský kalendář"),
    CalendarDisplayName::new("coptic", "Koptský kalendář"),
    CalendarDisplayName::new("dangi", "Korejský kalendář Dangi"),
    CalendarDisplayName::new("ethiopic", "Etiopský kalendář"),
    CalendarDisplayName::new("gregory", "Gregoriánský kalendář"),
    CalendarDisplayName::new("hebrew", "Hebrejský kalendář"),
    CalendarDisplayName::new("indian", "Indický národní kalendář"),
    CalendarDisplayName::new("islamic-civil", "Kalendář podle hidžry (občanský)"),
    CalendarDisplayName::new("islamic-umalqura", "Kalendář podle hidžry (Umm al-Qura)"),
    CalendarDisplayName::new("iso8601-week", "Kalendář ISO-8601"),
    CalendarDisplayName::new("japanese", "Japonský kalendář"),
    CalendarDisplayName::new("persian", "Perský kalendář"),
    CalendarDisplayName::new("persian-arithmetic", "Perský kalendář"),
    CalendarDisplayName::new("roc", "Kalendář Čínské republiky"),
];

const DE_CALENDAR_NAMES: &[CalendarDisplayName] = &[
    CalendarDisplayName::new("buddhist", "Buddhistischer Kalender"),
    CalendarDisplayName::new("chinese", "Chinesischer Kalender"),
    CalendarDisplayName::new("coptic", "Koptischer Kalender"),
    CalendarDisplayName::new("dangi", "Dangi-Kalender"),
    CalendarDisplayName::new("ethiopic", "Äthiopischer Kalender"),
    CalendarDisplayName::new("gregory", "Gregorianischer Kalender"),
    CalendarDisplayName::new("hebrew", "Jüdischer Kalender"),
    CalendarDisplayName::new("indian", "Indischer Nationalkalender"),
    CalendarDisplayName::new(
        "islamic-civil",
        "Hidschra-Kalender (tabellarisch, nicht-astronomisch)",
    ),
    CalendarDisplayName::new("islamic-umalqura", "Hidschra-Kalender (Umm al-Qura)"),
    CalendarDisplayName::new("iso8601-week", "ISO-8601-Kalender"),
    CalendarDisplayName::new("japanese", "Japanischer Kalender"),
    CalendarDisplayName::new("persian", "Persischer Kalender"),
    CalendarDisplayName::new("persian-arithmetic", "Persischer Kalender"),
    CalendarDisplayName::new("roc", "Minguo-Kalender"),
];

const EN_CALENDAR_NAMES: &[CalendarDisplayName] = &[
    CalendarDisplayName::new("buddhist", "Buddhist Calendar"),
    CalendarDisplayName::new("chinese", "Chinese Calendar"),
    CalendarDisplayName::new("coptic", "Coptic Calendar"),
    CalendarDisplayName::new("dangi", "Dangi Calendar"),
    CalendarDisplayName::new("ethiopic", "Ethiopic Calendar"),
    CalendarDisplayName::new("gregory", "Gregorian Calendar"),
    CalendarDisplayName::new("hebrew", "Hebrew Calendar"),
    CalendarDisplayName::new("indian", "Indian National Calendar"),
    CalendarDisplayName::new("islamic-civil", "Hijri Calendar (tabular, civil epoch)"),
    CalendarDisplayName::new(
        "islamic-tbla",
        "Hijri Calendar (tabular, astronomical epoch)",
    ),
    CalendarDisplayName::new("islamic-umalqura", "Hijri Calendar (Umm al-Qura)"),
    CalendarDisplayName::new("islamic-rgsa", "Hijri Calendar (Saudi Arabia, sighting)"),
    CalendarDisplayName::new("iso8601-week", "Gregorian Calendar (ISO 8601 Weeks)"),
    CalendarDisplayName::new("japanese", "Japanese Calendar"),
    CalendarDisplayName::new("persian", "Persian Calendar"),
    CalendarDisplayName::new("persian-arithmetic", "Persian Calendar"),
    CalendarDisplayName::new("roc", "Minguo Calendar"),
];

const ES_CALENDAR_NAMES: &[CalendarDisplayName] = &[
    CalendarDisplayName::new("buddhist", "calendario budista"),
    CalendarDisplayName::new("chinese", "calendario chino"),
    CalendarDisplayName::new("coptic", "calendario cóptico"),
    CalendarDisplayName::new("dangi", "calendario dangi"),
    CalendarDisplayName::new("ethiopic", "calendario etíope"),
    CalendarDisplayName::new("gregory", "calendario gregoriano"),
    CalendarDisplayName::new("hebrew", "calendario hebreo"),
    CalendarDisplayName::new("indian", "calendario nacional hindú"),
    CalendarDisplayName::new("islamic-civil", "calendario hijri tabular"),
    CalendarDisplayName::new("islamic-umalqura", "calendario hijri Umm al-Qura"),
    CalendarDisplayName::new("iso8601-week", "calendario ISO-8601"),
    CalendarDisplayName::new("japanese", "calendario japonés"),
    CalendarDisplayName::new("persian", "calendario persa"),
    CalendarDisplayName::new("persian-arithmetic", "calendario persa"),
    CalendarDisplayName::new("roc", "calendario de la República de China"),
];

const FA_CALENDAR_NAMES: &[CalendarDisplayName] = &[
    CalendarDisplayName::new("buddhist", "تقویم بودایی"),
    CalendarDisplayName::new("chinese", "تقویم چینی"),
    CalendarDisplayName::new("coptic", "تقویم قبطی"),
    CalendarDisplayName::new("dangi", "تقویم دانگی"),
    CalendarDisplayName::new("ethiopic", "تقویم اتیوپیایی"),
    CalendarDisplayName::new("gregory", "تقویم میلادی"),
    CalendarDisplayName::new("hebrew", "تقویم عبری"),
    CalendarDisplayName::new("indian", "تقویم ملی هند"),
    CalendarDisplayName::new("islamic-civil", "تقویم هجری قمری جدولی مدنی"),
    CalendarDisplayName::new("islamic-tbla", "تقویم هجری قمری جدولی نجومی"),
    CalendarDisplayName::new("islamic-umalqura", "تقویم هجری قمری ام‌القری"),
    CalendarDisplayName::new("islamic-rgsa", "قویم هجری قمری هلالی عربستان سعودی"),
    CalendarDisplayName::new("iso8601-week", "تقویم ایزو ۸۶۰۱"),
    CalendarDisplayName::new("japanese", "تقویم ژاپنی"),
    CalendarDisplayName::new("persian", "تقویم هجری شمسی"),
    CalendarDisplayName::new("persian-arithmetic", "تقویم هجری شمسی"),
    CalendarDisplayName::new("roc", "تقویم جمهوری چین (تایوان)"),
];

const FR_CALENDAR_NAMES: &[CalendarDisplayName] = &[
    CalendarDisplayName::new("buddhist", "calendrier bouddhiste"),
    CalendarDisplayName::new("chinese", "calendrier chinois"),
    CalendarDisplayName::new("coptic", "calendrier copte"),
    CalendarDisplayName::new("dangi", "calendrier dangi"),
    CalendarDisplayName::new("ethiopic", "calendrier éthiopien"),
    CalendarDisplayName::new("gregory", "calendrier grégorien"),
    CalendarDisplayName::new("hebrew", "calendrier hébraïque"),
    CalendarDisplayName::new("indian", "calendrier indien"),
    CalendarDisplayName::new(
        "islamic-civil",
        "calendrier hégirien (tabulaire, époque civile)",
    ),
    CalendarDisplayName::new(
        "islamic-tbla",
        "calendrier hégirien (tabulaire, époque astronomique)",
    ),
    CalendarDisplayName::new("islamic-umalqura", "calendrier hégirien (Umm al-Qura)"),
    CalendarDisplayName::new("iso8601-week", "calendrier ISO 8601"),
    CalendarDisplayName::new("japanese", "calendrier japonais"),
    CalendarDisplayName::new("persian", "calendrier persan"),
    CalendarDisplayName::new("persian-arithmetic", "calendrier persan"),
    CalendarDisplayName::new("roc", "calendrier républicain chinois"),
];

const HE_CALENDAR_NAMES: &[CalendarDisplayName] = &[
    CalendarDisplayName::new("buddhist", "לוח השנה הבודהיסטי"),
    CalendarDisplayName::new("chinese", "לוח השנה הסיני"),
    CalendarDisplayName::new("coptic", "לוח השנה הקופטי"),
    CalendarDisplayName::new("dangi", "לוח השנה הקוריאני"),
    CalendarDisplayName::new("ethiopic", "לוח השנה האתיופי"),
    CalendarDisplayName::new("gregory", "לוח השנה הגרגוריאני"),
    CalendarDisplayName::new("hebrew", "לוח השנה העברי"),
    CalendarDisplayName::new("indian", "לוח השנה ההודי הלאומי"),
    CalendarDisplayName::new("islamic-civil", "לוח השנה המוסלמי האזרחי"),
    CalendarDisplayName::new("islamic-tbla", "לוח השנה המוסלמי האסטרונומי"),
    CalendarDisplayName::new("islamic-umalqura", "לוח השנה המוסלמי אום אל-קורא"),
    CalendarDisplayName::new("islamic-rgsa", "לוח השנה המוסלמי (ערב הסעודית)"),
    CalendarDisplayName::new("iso8601-week", "לוח שנה ISO-8601"),
    CalendarDisplayName::new("japanese", "לוח השנה היפני"),
    CalendarDisplayName::new("persian", "לוח השנה הפרסי"),
    CalendarDisplayName::new("persian-arithmetic", "לוח השנה הפרסי"),
    CalendarDisplayName::new("roc", "לוח השנה הטייוואני"),
];

const HI_CALENDAR_NAMES: &[CalendarDisplayName] = &[
    CalendarDisplayName::new("buddhist", "बौद्ध पंचांग"),
    CalendarDisplayName::new("chinese", "चीनी पंचांग"),
    CalendarDisplayName::new("coptic", "कॉप्टिक कैलेंडर"),
    CalendarDisplayName::new("dangi", "दांगी कैलेंडर"),
    CalendarDisplayName::new("ethiopic", "इथियोपिक कैलेंडर"),
    CalendarDisplayName::new("gregory", "ग्रेगोरियन कैलेंडर"),
    CalendarDisplayName::new("hebrew", "हिब्रू पंचांग"),
    CalendarDisplayName::new("indian", "भारतीय राष्ट्रीय कैलेंडर"),
    CalendarDisplayName::new("islamic-civil", "हिजरी नागरिक कैलेंडर"),
    CalendarDisplayName::new("islamic-umalqura", "हिजरी कैलेंडर (उम्म अल-क़ुरा)"),
    CalendarDisplayName::new("iso8601-week", "आईएसओ-8601 कैलेंडर"),
    CalendarDisplayName::new("japanese", "जापानी पंचांग"),
    CalendarDisplayName::new("persian", "फ़ारसी कैलेंडर"),
    CalendarDisplayName::new("persian-arithmetic", "फ़ारसी कैलेंडर"),
    CalendarDisplayName::new("roc", "चीनी गणतंत्र पंचांग"),
];

const ID_CALENDAR_NAMES: &[CalendarDisplayName] = &[
    CalendarDisplayName::new("buddhist", "Kalender Buddha"),
    CalendarDisplayName::new("chinese", "Kalender Tionghoa"),
    CalendarDisplayName::new("coptic", "Kalender Koptik"),
    CalendarDisplayName::new("dangi", "Kalender Dangi"),
    CalendarDisplayName::new("ethiopic", "Kalender Etiopia"),
    CalendarDisplayName::new("gregory", "Kalender Gregorian"),
    CalendarDisplayName::new("hebrew", "Kalender Ibrani"),
    CalendarDisplayName::new("indian", "Kalender Nasional India"),
    CalendarDisplayName::new("islamic-civil", "Kalender Sipil Islam"),
    CalendarDisplayName::new("islamic-tbla", "Kalender Astronomi Islam"),
    CalendarDisplayName::new("islamic-umalqura", "Kalender Islam (Umm al-Qura)"),
    CalendarDisplayName::new("islamic-rgsa", "Kalender Islam (Arab Saudi, penglihatan)"),
    CalendarDisplayName::new("iso8601-week", "Kalender ISO-8601"),
    CalendarDisplayName::new("japanese", "Kalender Jepang"),
    CalendarDisplayName::new("persian", "Kalender Persia"),
    CalendarDisplayName::new("persian-arithmetic", "Kalender Persia"),
    CalendarDisplayName::new("roc", "Kalender Min-guo"),
];

const IT_CALENDAR_NAMES: &[CalendarDisplayName] = &[
    CalendarDisplayName::new("buddhist", "Calendario buddista"),
    CalendarDisplayName::new("chinese", "Calendario cinese"),
    CalendarDisplayName::new("coptic", "Calendario copto"),
    CalendarDisplayName::new("dangi", "Calendario dangi"),
    CalendarDisplayName::new("ethiopic", "Calendario etiope"),
    CalendarDisplayName::new("gregory", "Calendario gregoriano"),
    CalendarDisplayName::new("hebrew", "Calendario ebraico"),
    CalendarDisplayName::new("indian", "calendario nazionale indiano"),
    CalendarDisplayName::new("islamic-civil", "Calendario Hijri (tabulare, epoca civile)"),
    CalendarDisplayName::new("islamic-umalqura", "Calendario Hijri (Umm al-Qura)"),
    CalendarDisplayName::new("iso8601-week", "Calendario ISO-8601"),
    CalendarDisplayName::new("japanese", "Calendario giapponese"),
    CalendarDisplayName::new("persian", "Calendario persiano"),
    CalendarDisplayName::new("persian-arithmetic", "Calendario persiano"),
    CalendarDisplayName::new("roc", "Calendario minguo"),
];

const JA_CALENDAR_NAMES: &[CalendarDisplayName] = &[
    CalendarDisplayName::new("buddhist", "仏暦"),
    CalendarDisplayName::new("chinese", "中国暦"),
    CalendarDisplayName::new("coptic", "コプト暦"),
    CalendarDisplayName::new("dangi", "ダンギ暦"),
    CalendarDisplayName::new("ethiopic", "エチオピア暦"),
    CalendarDisplayName::new("gregory", "西暦(グレゴリオ暦)"),
    CalendarDisplayName::new("hebrew", "ユダヤ暦"),
    CalendarDisplayName::new("indian", "インド国定暦"),
    CalendarDisplayName::new("islamic-civil", "イスラム暦(定周期、公民紀元)"),
    CalendarDisplayName::new("islamic-umalqura", "イスラム暦(ウンム・アルクラー)"),
    CalendarDisplayName::new("iso8601-week", "ISO-8601"),
    CalendarDisplayName::new("japanese", "和暦"),
    CalendarDisplayName::new("persian", "ペルシア暦"),
    CalendarDisplayName::new("persian-arithmetic", "ペルシア暦"),
    CalendarDisplayName::new("roc", "中華民国暦"),
];

const JV_CALENDAR_NAMES: &[CalendarDisplayName] = &[
    CalendarDisplayName::new("buddhist", "Tanggalan Buddha"),
    CalendarDisplayName::new("chinese", "Tanggalan Cina"),
    CalendarDisplayName::new("coptic", "Tanggalan Koptik"),
    CalendarDisplayName::new("dangi", "Tanggalan Dangi"),
    CalendarDisplayName::new("ethiopic", "Tanggalan Etiopia"),
    CalendarDisplayName::new("gregory", "Tanggalan Gregorian"),
    CalendarDisplayName::new("hebrew", "Tanggalan Ibrani"),
    CalendarDisplayName::new("islamic-civil", "Tanggalan Hijriah (tabel, jaman sipil)"),
    CalendarDisplayName::new(
        "islamic-tbla",
        "Tanggalan Hijriah (tabel, jaman astronomis)",
    ),
    CalendarDisplayName::new("islamic-umalqura", "Tanggalan Hijriah (Ummul Qura)"),
    CalendarDisplayName::new("iso8601-week", "Tanggalan ISO-8601"),
    CalendarDisplayName::new("japanese", "Tanggalan Jepang"),
    CalendarDisplayName::new("persian", "Tanggalan Persia"),
    CalendarDisplayName::new("persian-arithmetic", "Tanggalan Persia"),
    CalendarDisplayName::new("roc", "Tanggalan Minguo"),
];

const KO_CALENDAR_NAMES: &[CalendarDisplayName] = &[
    CalendarDisplayName::new("buddhist", "불교력"),
    CalendarDisplayName::new("chinese", "음력"),
    CalendarDisplayName::new("coptic", "콥트력"),
    CalendarDisplayName::new("dangi", "단기력"),
    CalendarDisplayName::new("ethiopic", "에티오피아력"),
    CalendarDisplayName::new("gregory", "양력"),
    CalendarDisplayName::new("hebrew", "히브리력"),
    CalendarDisplayName::new("indian", "인도력"),
    CalendarDisplayName::new("islamic-civil", "히즈라 상용력"),
    CalendarDisplayName::new("islamic-umalqura", "히즈라력(움 알 쿠라)"),
    CalendarDisplayName::new("iso8601-week", "ISO-8601 달력"),
    CalendarDisplayName::new("japanese", "일본력"),
    CalendarDisplayName::new("persian", "페르시안력"),
    CalendarDisplayName::new("persian-arithmetic", "페르시안력"),
    CalendarDisplayName::new("roc", "대만력"),
];

const ML_CALENDAR_NAMES: &[CalendarDisplayName] = &[
    CalendarDisplayName::new("buddhist", "ബുദ്ധിസ്റ്റ് കലണ്ടർ"),
    CalendarDisplayName::new("chinese", "ചൈനീസ് കലണ്ടർ"),
    CalendarDisplayName::new("coptic", "കോപ്റ്റിക് കലണ്ടർ"),
    CalendarDisplayName::new("dangi", "ഡാംഗി കലണ്ടർ"),
    CalendarDisplayName::new("ethiopic", "എത്യോപിക് കലണ്ടർ"),
    CalendarDisplayName::new("gregory", "ഇംഗ്ലീഷ് കലണ്ടർ"),
    CalendarDisplayName::new("hebrew", "ഹീബ്രൂ കലണ്ടർ"),
    CalendarDisplayName::new("indian", "ശകവർഷ കലണ്ടർ"),
    CalendarDisplayName::new("islamic-civil", "ഇസ്ലാമിക് കലണ്ടർ (ടാബുലർ, സിവിൽ ഇപോക്ക്)"),
    CalendarDisplayName::new("islamic-tbla", "ഇസ്ലാം-ജ്യോതിഷ കലണ്ടർ"),
    CalendarDisplayName::new("islamic-umalqura", "ഇസ്ലാമിക് കലണ്ടർ (ഉം അൽ ഖുറ)"),
    CalendarDisplayName::new("islamic-rgsa", "ഇസ്ലാം-അറബിക് കലണ്ടർ"),
    CalendarDisplayName::new("iso8601-week", "ഐഎസ്ഓ 8601 കലണ്ടർ"),
    CalendarDisplayName::new("japanese", "ജാപ്പനീസ് കലണ്ടർ"),
    CalendarDisplayName::new("persian", "പേർഷ്യൻ കലണ്ടർ"),
    CalendarDisplayName::new("persian-arithmetic", "പേർഷ്യൻ കലണ്ടർ"),
    CalendarDisplayName::new("roc", "മിംഗ്വോ കലണ്ടർ"),
];

const MY_CALENDAR_NAMES: &[CalendarDisplayName] = &[
    CalendarDisplayName::new("buddhist", "ဗုဒ္ဓ ပြက္ခဒိန်"),
    CalendarDisplayName::new("chinese", "တရုတ် ပြက္ခဒိန်"),
    CalendarDisplayName::new("coptic", "ကို့ပ်တစ် ပြက္ခဒိန်"),
    CalendarDisplayName::new("dangi", "ဒန်းဂိ ပြက္ခဒိန်"),
    CalendarDisplayName::new("ethiopic", "အီသီယိုးပီးယား ပြက္ခဒိန်"),
    CalendarDisplayName::new("gregory", "နိုင်ငံတကာသုံး ပြက္ခဒိန်"),
    CalendarDisplayName::new("hebrew", "ဟီဘရူး ပြက္ခဒိန်"),
    CalendarDisplayName::new("indian", "အိန္ဒြိယ အမျိုးသား ပြက္ခဒိန်"),
    CalendarDisplayName::new("islamic-civil", "အစ္စလာမ်မစ် ပြက္ခဒိန်"),
    CalendarDisplayName::new("islamic-umalqura", "အယ်လ်ကူရာ အစ္စလာမ်မစ် ပြက္ခဒိန်"),
    CalendarDisplayName::new("iso8601-week", "ISO-8601 ပြက္ခဒိန်"),
    CalendarDisplayName::new("japanese", "ဂျပန် ပြက္ခဒိန်"),
    CalendarDisplayName::new("persian", "ပါရှား ပြက္ခဒိန်"),
    CalendarDisplayName::new("persian-arithmetic", "ပါရှား ပြက္ခဒိန်"),
    CalendarDisplayName::new("roc", "မင်ဂုအို ပြက္ခဒိန်"),
];

const NE_CALENDAR_NAMES: &[CalendarDisplayName] = &[
    CalendarDisplayName::new("buddhist", "बुद्धिष्ट पात्रो"),
    CalendarDisplayName::new("chinese", "चिनियाँ पात्रो"),
    CalendarDisplayName::new("coptic", "कोप्टिक पात्र"),
    CalendarDisplayName::new("dangi", "डाङ्गी पात्रो"),
    CalendarDisplayName::new("ethiopic", "इथिओपिक पात्रो"),
    CalendarDisplayName::new("gregory", "ग्रेगोरियन पात्रो"),
    CalendarDisplayName::new("hebrew", "हिब्रु पात्रो"),
    CalendarDisplayName::new("indian", "भारतीय राष्ट्रिय पात्रो"),
    CalendarDisplayName::new("islamic-civil", "हिजरी पात्रो (टेबुलर, नागरिक युग)"),
    CalendarDisplayName::new("islamic-umalqura", "इस्लामी पात्रो"),
    CalendarDisplayName::new("iso8601-week", "ISO-8601 पात्रो"),
    CalendarDisplayName::new("japanese", "जापानी पात्रो"),
    CalendarDisplayName::new("persian", "फारसी पात्रो"),
    CalendarDisplayName::new("persian-arithmetic", "फारसी पात्रो"),
    CalendarDisplayName::new("roc", "चिनियाँ गणतन्त्रको पात्रो"),
];

const NL_CALENDAR_NAMES: &[CalendarDisplayName] = &[
    CalendarDisplayName::new("buddhist", "Boeddhistische kalender"),
    CalendarDisplayName::new("chinese", "Chinese kalender"),
    CalendarDisplayName::new("coptic", "Koptische kalender"),
    CalendarDisplayName::new("dangi", "Dangi-kalender"),
    CalendarDisplayName::new("ethiopic", "Ethiopische kalender"),
    CalendarDisplayName::new("gregory", "Gregoriaanse kalender"),
    CalendarDisplayName::new("hebrew", "Hebreeuwse kalender"),
    CalendarDisplayName::new("indian", "Indiase nationale kalender"),
    CalendarDisplayName::new("islamic-civil", "Islamitische kalender (cyclisch)"),
    CalendarDisplayName::new("islamic-tbla", "Islamitische kalender (epoche)"),
    CalendarDisplayName::new("islamic-umalqura", "Islamitische kalender (Umm al-Qura)"),
    CalendarDisplayName::new("islamic-rgsa", "Islamitische kalender (Saudi–Arabië)"),
    CalendarDisplayName::new("iso8601-week", "ISO-8601-kalender"),
    CalendarDisplayName::new("japanese", "Japanse kalender"),
    CalendarDisplayName::new("persian", "Perzische kalender"),
    CalendarDisplayName::new("persian-arithmetic", "Perzische kalender"),
    CalendarDisplayName::new("roc", "Kalender van de Chinese Republiek"),
];

const PL_CALENDAR_NAMES: &[CalendarDisplayName] = &[
    CalendarDisplayName::new("buddhist", "kalendarz buddyjski"),
    CalendarDisplayName::new("chinese", "kalendarz chiński"),
    CalendarDisplayName::new("coptic", "kalendarz koptyjski"),
    CalendarDisplayName::new("dangi", "kalendarz koreański"),
    CalendarDisplayName::new("ethiopic", "kalendarz etiopski"),
    CalendarDisplayName::new("gregory", "kalendarz gregoriański"),
    CalendarDisplayName::new("hebrew", "kalendarz hebrajski"),
    CalendarDisplayName::new("indian", "narodowy kalendarz hinduski"),
    CalendarDisplayName::new(
        "islamic-civil",
        "kalendarz muzułmański (metoda obliczeniowa)",
    ),
    CalendarDisplayName::new(
        "islamic-tbla",
        "kalendarz islamski (metoda obliczeniowa, epoka astronomiczna)",
    ),
    CalendarDisplayName::new("islamic-umalqura", "kalendarz muzułmański (Umm al-Kura)"),
    CalendarDisplayName::new(
        "islamic-rgsa",
        "kalendarz islamski (Arabia Saudyjska, metoda wzrokowa)",
    ),
    CalendarDisplayName::new("iso8601-week", "kalendarz ISO-8601"),
    CalendarDisplayName::new("japanese", "kalendarz japoński"),
    CalendarDisplayName::new("persian", "kalendarz perski"),
    CalendarDisplayName::new("persian-arithmetic", "kalendarz perski"),
    CalendarDisplayName::new("roc", "kalendarz Republiki Chińskiej"),
];

const PT_CALENDAR_NAMES: &[CalendarDisplayName] = &[
    CalendarDisplayName::new("buddhist", "Calendário Budista"),
    CalendarDisplayName::new("chinese", "Calendário Chinês"),
    CalendarDisplayName::new("coptic", "Calendário Copta"),
    CalendarDisplayName::new("dangi", "Calendário Dangi"),
    CalendarDisplayName::new("ethiopic", "Calendário Etíope"),
    CalendarDisplayName::new("gregory", "Calendário Gregoriano"),
    CalendarDisplayName::new("hebrew", "Calendário Hebraico"),
    CalendarDisplayName::new("indian", "Calendário Nacional Indiano"),
    CalendarDisplayName::new(
        "islamic-civil",
        "Calendário Hegírico (tabular, época civil)",
    ),
    CalendarDisplayName::new("islamic-umalqura", "Calendário Hegírico (Umm al‑Qura)"),
    CalendarDisplayName::new("iso8601-week", "Calendário ISO-8601"),
    CalendarDisplayName::new("japanese", "Calendário Japonês"),
    CalendarDisplayName::new("persian", "Calendário Persa"),
    CalendarDisplayName::new("persian-arithmetic", "Calendário Persa"),
    CalendarDisplayName::new("roc", "Calendário da República da China"),
];

const RU_CALENDAR_NAMES: &[CalendarDisplayName] = &[
    CalendarDisplayName::new("buddhist", "буддийский календарь"),
    CalendarDisplayName::new("chinese", "китайский календарь"),
    CalendarDisplayName::new("coptic", "коптский календарь"),
    CalendarDisplayName::new("dangi", "календарь данги"),
    CalendarDisplayName::new("ethiopic", "эфиопский календарь"),
    CalendarDisplayName::new("gregory", "григорианский календарь"),
    CalendarDisplayName::new("hebrew", "еврейский календарь"),
    CalendarDisplayName::new("indian", "Национальный календарь Индии"),
    CalendarDisplayName::new("islamic-civil", "гражданский календарь хиджры (табличный)"),
    CalendarDisplayName::new(
        "islamic-tbla",
        "исламский календарь (табличный, астрономическая эпоха)",
    ),
    CalendarDisplayName::new("islamic-umalqura", "календарь хиджры (Умм аль-Кура)"),
    CalendarDisplayName::new("islamic-rgsa", "исламский календарь (Саудовская Аравия)"),
    CalendarDisplayName::new("iso8601-week", "григорианский (сначала год)"),
    CalendarDisplayName::new("japanese", "японский календарь"),
    CalendarDisplayName::new("persian", "персидский календарь"),
    CalendarDisplayName::new("persian-arithmetic", "персидский календарь"),
    CalendarDisplayName::new("roc", "календарь Миньго"),
];

const SA_CALENDAR_NAMES: &[CalendarDisplayName] =
    &[CalendarDisplayName::new("gregory", "ग्रेगोरियन पञ्चाङ्ग")];

const SYR_CALENDAR_NAMES: &[CalendarDisplayName] = &[
    CalendarDisplayName::new("buddhist", "ܣܘܪܓܕܐ ܒܘܕܗܝܝܐ"),
    CalendarDisplayName::new("chinese", "ܣܘܪܓܕܐ ܨܝܢܝܐ"),
    CalendarDisplayName::new("coptic", "ܣܘܪܓܕܐ ܐܓܒܛܝܐ"),
    CalendarDisplayName::new("dangi", "ܣܘܪܓܕܐ ܕܢܓܝ"),
    CalendarDisplayName::new("ethiopic", "ܣܘܪܓܕܐ ܟܘܫܝܐ"),
    CalendarDisplayName::new("gregory", "ܣܘܪܓܕܐ ܓܪܝܓܘܪܝܐ"),
    CalendarDisplayName::new("hebrew", "ܣܘܪܓܕܐ ܝܗܘܕܝܐ"),
    CalendarDisplayName::new("indian", "ܣܘܪܓܕܐ ܐܘܡܬܢܝܐ ܗܢܕܘܝܐ"),
    CalendarDisplayName::new("islamic-civil", "ܣܘܪܓܕܐ ܡܫܠܡܢܐ ܡܕܝܢܝܐ"),
    CalendarDisplayName::new("iso8601-week", "ܣܘܪܓܕܐ ISO-8601"),
    CalendarDisplayName::new("japanese", "ܣܘܪܓܕܐ ܝܦܢܝܐ"),
    CalendarDisplayName::new("persian", "ܣܘܪܓܕܐ ܦܪܣܝܐ"),
    CalendarDisplayName::new("persian-arithmetic", "ܣܘܪܓܕܐ ܦܪܣܝܐ"),
    CalendarDisplayName::new("roc", "ܣܘܪܓܕܐ ܡܝܢܓܘ"),
];

const TA_CALENDAR_NAMES: &[CalendarDisplayName] = &[
    CalendarDisplayName::new("buddhist", "புத்த நாள்காட்டி"),
    CalendarDisplayName::new("chinese", "சீன நாள்காட்டி"),
    CalendarDisplayName::new("coptic", "காப்டிக் நாள்காட்டி"),
    CalendarDisplayName::new("dangi", "டேங்கி நாள்காட்டி"),
    CalendarDisplayName::new("ethiopic", "எத்தியோப்பிய நாள்காட்டி"),
    CalendarDisplayName::new("gregory", "கிரிகோரியன் நாள்காட்டி"),
    CalendarDisplayName::new("hebrew", "ஹீப்ரு நாள்காட்டி"),
    CalendarDisplayName::new("indian", "இந்திய தேசிய நாள்காட்டி"),
    CalendarDisplayName::new("islamic-civil", "இஸ்லாமிய சிவில் நாள்காட்டி"),
    CalendarDisplayName::new("islamic-tbla", "இஸ்லாமிய வானியல் நாள்காட்டி"),
    CalendarDisplayName::new("islamic-umalqura", "இஸ்லாமிய நாள்காட்டி (உம்-அல்-குரா)"),
    CalendarDisplayName::new("iso8601-week", "கிரிகோரிய நாள்காட்டி"),
    CalendarDisplayName::new("japanese", "ஜப்பானிய நாள்காட்டி"),
    CalendarDisplayName::new("persian", "பாரசீக நாள்காட்டி"),
    CalendarDisplayName::new("persian-arithmetic", "பாரசீக நாள்காட்டி"),
    CalendarDisplayName::new("roc", "மின்கோ நாள்காட்டி"),
];

const TH_CALENDAR_NAMES: &[CalendarDisplayName] = &[
    CalendarDisplayName::new("buddhist", "ปฏิทินพุทธ"),
    CalendarDisplayName::new("chinese", "ปฏิทินจีน"),
    CalendarDisplayName::new("coptic", "ปฏิทินคอปติก"),
    CalendarDisplayName::new("dangi", "ปฏิทินเกาหลี"),
    CalendarDisplayName::new("ethiopic", "ปฏิทินเอธิโอเปีย"),
    CalendarDisplayName::new("gregory", "ปฏิทินเกรกอเรียน"),
    CalendarDisplayName::new("hebrew", "ปฏิทินฮิบรู"),
    CalendarDisplayName::new("indian", "ปฏิทินแห่งชาติอินเดีย"),
    CalendarDisplayName::new("islamic-civil", "ปฏิทินอิสลามซีวิล"),
    CalendarDisplayName::new("islamic-umalqura", "ปฏิทินอิสลาม (อุมม์อัลกุรา)"),
    CalendarDisplayName::new("iso8601-week", "ปฏิทิน ISO-8601"),
    CalendarDisplayName::new("japanese", "ปฏิทินญี่ปุ่น"),
    CalendarDisplayName::new("persian", "ปฏิทินเปอร์เชีย"),
    CalendarDisplayName::new("persian-arithmetic", "ปฏิทินเปอร์เชีย"),
    CalendarDisplayName::new("roc", "ปฏิทินไต้หวัน"),
];

const TR_CALENDAR_NAMES: &[CalendarDisplayName] = &[
    CalendarDisplayName::new("buddhist", "Budist Takvimi"),
    CalendarDisplayName::new("chinese", "Çin Takvimi"),
    CalendarDisplayName::new("coptic", "Kıpti Takvim"),
    CalendarDisplayName::new("dangi", "Dangi Takvimi"),
    CalendarDisplayName::new("ethiopic", "Etiyopik Takvim"),
    CalendarDisplayName::new("gregory", "Miladi Takvim"),
    CalendarDisplayName::new("hebrew", "İbrani Takvimi"),
    CalendarDisplayName::new("indian", "Ulusal Hint Takvimi"),
    CalendarDisplayName::new("islamic-civil", "Hicri Takvim (16 Temmuz 622)"),
    CalendarDisplayName::new("islamic-tbla", "Hicri Takvim (15 Temmuz 622)"),
    CalendarDisplayName::new("islamic-umalqura", "Hicri Takvim (Ümmü-l Kurra Takvimi)"),
    CalendarDisplayName::new("islamic-rgsa", "Hicri Takvim (Suudi)"),
    CalendarDisplayName::new("iso8601-week", "ISO-8601 Takvimi"),
    CalendarDisplayName::new("japanese", "Japon Takvimi"),
    CalendarDisplayName::new("persian", "İran Takvimi"),
    CalendarDisplayName::new("persian-arithmetic", "İran Takvimi"),
    CalendarDisplayName::new("roc", "Çin Cumhuriyeti Takvimi"),
];

const VI_CALENDAR_NAMES: &[CalendarDisplayName] = &[
    CalendarDisplayName::new("buddhist", "Lịch Phật Giáo"),
    CalendarDisplayName::new("chinese", "Lịch Trung Quốc"),
    CalendarDisplayName::new("coptic", "Lịch Copts"),
    CalendarDisplayName::new("dangi", "Lịch Dangi"),
    CalendarDisplayName::new("ethiopic", "Lịch Ethiopia"),
    CalendarDisplayName::new("gregory", "Lịch Gregory"),
    CalendarDisplayName::new("hebrew", "Lịch Do Thái"),
    CalendarDisplayName::new("indian", "Lịch Quốc gia Ấn Độ"),
    CalendarDisplayName::new(
        "islamic-civil",
        "Lịch Hồi Giáo (dạng bảng, kỷ nguyên dân sự)",
    ),
    CalendarDisplayName::new("islamic-umalqura", "Lịch Hồi Giáo (Umm al-Qura)"),
    CalendarDisplayName::new("iso8601-week", "Lịch ISO-8601"),
    CalendarDisplayName::new("japanese", "Lịch Nhật Bản"),
    CalendarDisplayName::new("persian", "Lịch Ba Tư"),
    CalendarDisplayName::new("persian-arithmetic", "Lịch Ba Tư"),
    CalendarDisplayName::new("roc", "Lịch Trung Hoa Dân Quốc"),
];

const ZGH_CALENDAR_NAMES: &[CalendarDisplayName] = &[
    CalendarDisplayName::new("coptic", "ⴰⵙⵎⵍⵓⵙⵙⴰⵏ ⴰⵇⴱⵟⵉ"),
    CalendarDisplayName::new("ethiopic", "ⴰⵙⵎⵍⵓⵙⵙⴰⵏ ⵏ ⵉⵜⵢⵓⴱⵢⴰ"),
    CalendarDisplayName::new("gregory", "ⴰⵙⵎⵍⵓⵙⵙⴰⵏ ⴰⴳⵔⵉⴳⵓⵔ"),
];

const ZH_HANS_CALENDAR_NAMES: &[CalendarDisplayName] = &[
    CalendarDisplayName::new("buddhist", "佛历"),
    CalendarDisplayName::new("chinese", "农历"),
    CalendarDisplayName::new("coptic", "科普特历"),
    CalendarDisplayName::new("dangi", "檀纪历"),
    CalendarDisplayName::new("ethiopic", "埃塞俄比亚历"),
    CalendarDisplayName::new("gregory", "公历"),
    CalendarDisplayName::new("hebrew", "希伯来历"),
    CalendarDisplayName::new("indian", "印度国定历"),
    CalendarDisplayName::new("islamic-civil", "表格式伊斯兰历（民用纪元）"),
    CalendarDisplayName::new("islamic-tbla", "表格式伊斯兰历（天文纪元）"),
    CalendarDisplayName::new("islamic-umalqura", "伊斯兰历（乌姆库拉）"),
    CalendarDisplayName::new("iso8601-week", "国际标准历法"),
    CalendarDisplayName::new("japanese", "和历"),
    CalendarDisplayName::new("persian", "波斯历"),
    CalendarDisplayName::new("persian-arithmetic", "波斯历"),
    CalendarDisplayName::new("roc", "民国纪年"),
];

const ZH_HANT_CALENDAR_NAMES: &[CalendarDisplayName] = &[
    CalendarDisplayName::new("buddhist", "佛曆"),
    CalendarDisplayName::new("chinese", "農曆"),
    CalendarDisplayName::new("coptic", "科普特曆"),
    CalendarDisplayName::new("dangi", "檀紀曆"),
    CalendarDisplayName::new("ethiopic", "衣索比亞曆"),
    CalendarDisplayName::new("gregory", "公曆"),
    CalendarDisplayName::new("hebrew", "希伯來曆"),
    CalendarDisplayName::new("indian", "印度國曆"),
    CalendarDisplayName::new("islamic-civil", "伊斯蘭民用曆"),
    CalendarDisplayName::new("islamic-tbla", "伊斯蘭天文曆"),
    CalendarDisplayName::new("islamic-umalqura", "伊斯蘭曆（烏姆庫拉）"),
    CalendarDisplayName::new("islamic-rgsa", "伊斯蘭新月曆"),
    CalendarDisplayName::new("iso8601-week", "ISO 8601 國際曆法"),
    CalendarDisplayName::new("japanese", "和曆"),
    CalendarDisplayName::new("persian", "波斯曆"),
    CalendarDisplayName::new("persian-arithmetic", "波斯曆"),
    CalendarDisplayName::new("roc", "國曆"),
];

// --- root -----------------------------------------------------------------

/// The floor of every lookup.
///
/// CLDR's own root locale names months `M01`…`M12` rather than inventing
/// English ones, and this follows it: a caller that reaches root is told
/// plainly that no language claimed the field.
///
/// Its week begins on Monday, the day CLDR 48 `weekData/firstDay` gives the
/// world, `001` (`cldr48-supplemental`): `und` names no language and no
/// region, and this entry is also where a tag with an unknown language
/// lands, so it takes the world's default rather than the region CLDR's
/// likely subtags would guess for `und`.
pub static ROOT: LocaleData = LocaleData {
    tag: "und",
    sources: "Unicode CLDR 48, common/main/root.xml (cldr48-main)",
    english_name: "Root",
    native_name: "",
    script: "Latn",
    templates: DateTemplates::NONE,
    calendar_names: &[],
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
    sources: "Unicode CLDR 48, common/main/am.xml (cldr48-main), its Gregorian months and weekdays compared with it 2026-09-26; the other calendars' vocabulary as the entry's comment states",
    english_name: "Amharic",
    native_name: "አማርኛ",
    script: "Ethi",
    templates: DateTemplates::NONE,
    calendar_names: AM_CALENDAR_NAMES,
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
    // CLDR 48 `ar.xml`, `calendar type="islamic"`, compared 2026-09-26.
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
    sources: "Unicode CLDR 48, common/main/ar.xml (cldr48-main), its Gregorian months and weekdays compared with it 2026-09-26; the other calendars' vocabulary as the entry's comment states",
    english_name: "Arabic",
    native_name: "العربية",
    script: "Arab",
    templates: AR_TEMPLATES,
    calendar_names: AR_CALENDAR_NAMES,
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
// The names: Balinese is CLDR 48 `en.xml` `localeDisplayNames/languages`
// (`ban` is named there though CLDR has no `ban` locale); Basa Bali is the
// Latin form the infobox of Wikipedia, "Balinese language", read
// 2026-09-26, prints, its Balinese-script ᬩᬲᬩᬮᬶ not carried for the same
// reason the Pawukon names are not. No CLDR locale, so no templates.

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
        leap_names: LeapMonthNames::NONE,
        eras: EraNames::EMPTY,
        quarters: ContextualNames::EMPTY,
        templates: DateTemplates::NONE,
    },
];

const BAN: LocaleData = LocaleData {
    tag: "ban",
    sources: "none from CLDR, which has no ban locale: the Pawukon vocabulary from Reingold and Dershowitz (reingold2018) and wikipedia-pawukon, as the entry states",
    english_name: "Balinese",
    native_name: "Basa Bali",
    script: "Latn",
    templates: DateTemplates::NONE,
    calendar_names: &[],
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
// form), the stand-alone abbreviated and narrow ones, the weekdays with
// their short forms (Saturday's resolving to the abbreviation), the
// format-wide and
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
// The names are CLDR 48 `localeDisplayNames/languages`: Bangla in `en.xml`
// (the calendar prose here says Bengali; CLDR's English name is Bangla),
// বাংলা in `bn.xml`. No templates: `bn.xml` writes the long date "d MMMM,
// y", which no template here has the shape of.

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

const BN_MONTHS: &[&str] = &[
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
];

const BN_NARROW_MONTHS: &[&str] = &[
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
];

const BN_CALENDARS: &[CalendarNames] = &[
    gregorian(
        // Five of the stand-alone abbreviations are longer than the format
        // ones, April, July, August and November the whole word.
        &[month_cycle(ContextualNames {
            format: widths(
                BN_MONTHS,
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
                BN_NARROW_MONTHS,
            ),
            standalone: widths(
                BN_MONTHS,
                &[
                    "জানু",
                    "ফেব",
                    "মার্চ",
                    "এপ্রিল",
                    "মে",
                    "জুন",
                    "জুলাই",
                    "আগস্ট",
                    "সেপ্ট",
                    "অক্টো",
                    "নভেম্বর",
                    "ডিসে",
                ],
                BN_NARROW_MONTHS,
            ),
        })],
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
    sources: "Unicode CLDR 48, common/main/bn.xml (cldr48-main), its Gregorian months and weekdays compared with it 2026-09-26; the other calendars' vocabulary as the entry's comment states",
    english_name: "Bangla",
    native_name: "বাংলা",
    script: "Beng",
    templates: DateTemplates::NONE,
    calendar_names: BN_CALENDAR_NAMES,
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
        &["সোঃ", "মঃ", "বুঃ", "বৃঃ", "শুঃ", "শনি", "রঃ"],
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
// the wide names too. The week begins on Monday: CLDR 48's likely subtags
// give `bo` the region CN, which `weekData/firstDay` lists under Monday
// (a caller in India asks for `bo-IN` and gets Sunday from the region). The
// default numbering system inherits root's `latn`.
//
// The Tibetan calendar numbers its months (Janson, "Tibetan calendar
// mathematics", 2014, Section 5, which `hc_calendars_lunar::tibetan`
// follows), so the CLDR ordinal month names are keyed to it as English's
// "First Month" is keyed to the numbered lunisolar calendars — the same
// twelve words for the same twelve numbers, and not a translation. The
// Tsurphu version numbers them the same way (Janson, Appendix A.2) and
// shares them; the Bhutanese is written in Dzongkha, which is not this
// locale, and is not keyed here. CLDR has no `tibetan` calendar and no word
// for the doubled month, so the leap-month prefix is empty. The sixty-year names are not here: the
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
        &[CalendarId("tibetan"), CalendarId("tibetan-tsurphu")],
        &[month_cycle(ContextualNames {
            format: widths(BO_MONTHS, BO_MONTHS_ABBREVIATED, &[]),
            standalone: widths(BO_MONTHS_STANDALONE, &[], &[]),
        })],
        "",
    ),
];

const BO: LocaleData = LocaleData {
    tag: "bo",
    sources: "Unicode CLDR 48, common/main/bo.xml (cldr48-main), its Gregorian months and weekdays compared with it 2026-09-26; the other calendars' vocabulary as the entry's comment states",
    english_name: "Tibetan",
    native_name: "བོད་སྐད་",
    script: "Tibt",
    templates: DateTemplates::NONE,
    calendar_names: &[],
    direction: Direction::LeftToRight,
    numbering: "latn",
    first_day_of_week: Weekday::Monday,
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
    sources: "none from CLDR, which has no cop locale: the Bohairic months from Wikipedia, \"Coptic calendar\", as the entry states",
    english_name: "Coptic",
    native_name: "Ϯⲙⲉⲧⲣⲉⲙⲛ̀ⲭⲏⲙⲓ",
    script: "Copt",
    templates: DateTemplates::NONE,
    calendar_names: &[],
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
    sources: "Unicode CLDR 48, common/main/cs.xml (cldr48-main), its Gregorian months and weekdays compared with it 2026-09-26; the other calendars' vocabulary as the entry's comment states",
    english_name: "Czech",
    native_name: "čeština",
    script: "Latn",
    templates: CS_TEMPLATES,
    calendar_names: CS_CALENDAR_NAMES,
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

const DE_WEEKDAYS: &[&str] = &[
    "Montag",
    "Dienstag",
    "Mittwoch",
    "Donnerstag",
    "Freitag",
    "Samstag",
    "Sonntag",
];

const DE_MONTHS: &[&str] = &[
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
];

const DE: LocaleData = LocaleData {
    tag: "de",
    sources: "Unicode CLDR 48, common/main/de.xml (cldr48-main), its Gregorian months and weekdays compared with it 2026-09-26; the other calendars' vocabulary as the entry's comment states",
    english_name: "German",
    native_name: "Deutsch",
    script: "Latn",
    templates: DE_TEMPLATES,
    calendar_names: DE_CALENDAR_NAMES,
    direction: Direction::LeftToRight,
    numbering: "latn",
    first_day_of_week: Weekday::Monday,
    casing: CasingStyle::Standard,
    // German month names are nouns, so they are capitalised everywhere.
    capitalises_month_names: true,
    // The stand-alone abbreviations drop the full stop; the short forms
    // resolve to the format abbreviations in both contexts.
    weekdays: ContextualNames {
        format: weekday_widths(
            DE_WEEKDAYS,
            &["Mo.", "Di.", "Mi.", "Do.", "Fr.", "Sa.", "So."],
            &["Mo.", "Di.", "Mi.", "Do.", "Fr.", "Sa.", "So."],
            &["M", "D", "M", "D", "F", "S", "S"],
        ),
        standalone: weekday_widths(
            DE_WEEKDAYS,
            &["Mo", "Di", "Mi", "Do", "Fr", "Sa", "So"],
            &["Mo.", "Di.", "Mi.", "Do.", "Fr.", "Sa.", "So."],
            &["M", "D", "M", "D", "F", "S", "S"],
        ),
    },
    day_periods: ContextualNames::same(widths(&["AM", "PM"], &[], &[])),
    cycle: SexagenaryNames::EMPTY,
    calendars: &[gregorian(
        &[month_cycle(ContextualNames {
            format: widths(
                DE_MONTHS,
                &[
                    "Jan.", "Feb.", "März", "Apr.", "Mai", "Juni", "Juli", "Aug.", "Sept.", "Okt.",
                    "Nov.", "Dez.",
                ],
                &["J", "F", "M", "A", "M", "J", "J", "A", "S", "O", "N", "D"],
            ),
            standalone: widths(
                DE_MONTHS,
                &[
                    "Jan", "Feb", "Mär", "Apr", "Mai", "Jun", "Jul", "Aug", "Sep", "Okt", "Nov",
                    "Dez",
                ],
                &["J", "F", "M", "A", "M", "J", "J", "A", "S", "O", "N", "D"],
            ),
        })],
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
    // The Buddhist era as CLDR 48 abbreviates it (`common/main/en.xml`,
    // `calendar[@type="buddhist"]/eras/eraAbbr`, read 2026-09-26); CLDR states
    // no wide form, so the abbreviation serves for both.
    CalendarNames {
        calendars: BUDDHIST_CALENDARS,
        cycles: &[],
        leap_month_prefix: "",
        eras: EraNames {
            codes: &["be"],
            names: widths(&["BE"], &["BE"], &[]),
            calendars: &[],
        },
        quarters: ContextualNames::EMPTY,
        templates: DateTemplates::NONE,
        leap_names: LeapMonthNames::NONE,
    },
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
                // CLDR 48 `root.xml`, `calendar type="japanese"`, `eraAbbr`,
                // which `en.xml` inherits: the macron spellings, as the
                // English `japanese-imperial` era writes Kōki.
                &["Meiji", "Taishō", "Shōwa", "Heisei", "Reiwa"],
                &[],
                JAPANESE_ERA_NARROW,
            ),
            calendars: &[],
        },
        quarters: ContextualNames::EMPTY,
        templates: DateTemplates::NONE,
        leap_names: LeapMonthNames::NONE,
    },
    // CLDR 48 `en.xml`, `calendar type="islamic"`, compared 2026-09-26,
    // with its ʻ (U+02BB) where an ASCII apostrophe is often printed.
    dated(
        ISLAMIC_CALENDARS,
        &[months(&[
            "Muharram",
            "Safar",
            "Rabiʻ I",
            "Rabiʻ II",
            "Jumada I",
            "Jumada II",
            "Rajab",
            "Shaʻban",
            "Ramadan",
            "Shawwal",
            "Dhuʻl-Qiʻdah",
            "Dhuʻl-Hijjah",
        ])],
        &["ah"],
        &["AH"],
    ),
    // The months as `hc_calendars_lunar::hebrew` numbers them, Tishri 1 to
    // Elul 12, with Adar I the intercalary repetition of month 5 and the
    // Adar of a leap year Adar II — CLDR 48 `en.xml`, `calendar
    // type="hebrew"`, spells the same thirteen names in its own thirteen
    // slots. The year is written bare — 5784 — as English-language Jewish
    // calendars print it and as Reingold and Dershowitz, *Calendrical
    // Calculations* (4th ed., 2018, chapter 8), write it; *AM* is the era's
    // name when one is asked for.
    dated(
        HEBREW_CALENDARS,
        &[months(&[
            "Tishri", "Heshvan", "Kislev", "Tevet", "Shevat", "Adar", "Nisan", "Iyar", "Sivan",
            "Tamuz", "Av", "Elul",
        ])],
        &["am"],
        &["AM"],
    )
    .with_leap_names(LeapMonthNames {
        intercalary: &[(5, "Adar I")],
        in_leap_years: &[(6, "Adar II")],
    })
    .with_templates(HEBREW_TEMPLATES),
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
        templates: DateTemplates::NONE,
        leap_names: LeapMonthNames::NONE,
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
    // The Samaritans number their months, "the first month", "the sixth
    // month", as the community's calendars print them
    // (the-samaritans.net, israelite-samaritans.com, read 2026-09-26);
    // `hc_calendars_lunar::samaritan` numbers them from the Sixth Month,
    // where the year number changes, with the Thirteenth Month of an
    // intercalary year as the intercalary month after the Twelfth.
    CalendarNames {
        calendars: &[CalendarId("samaritan")],
        cycles: &[months(&[
            "Sixth Month",
            "Seventh Month",
            "Eighth Month",
            "Ninth Month",
            "Tenth Month",
            "Eleventh Month",
            "Twelfth Month",
            "First Month",
            "Second Month",
            "Third Month",
            "Fourth Month",
            "Fifth Month",
        ])],
        leap_month_prefix: "",
        eras: EraNames::EMPTY,
        quarters: ContextualNames::EMPTY,
        templates: DateTemplates::NONE,
        leap_names: LeapMonthNames {
            intercalary: &[(7, "Thirteenth Month")],
            in_leap_years: &[],
        },
    },
    // The scrolls number the months of the 364-day year, "the first
    // month", "the seventh month", as Talmon translates them in the
    // Encyclopedia of the Dead Sea Scrolls (2000), p. 110.
    lunisolar(
        QUMRAN_CALENDARS,
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
        "",
    ),
    // Palmen numbers the months of a yerm and writes them "Month 5",
    // "Night 26 Month 2 Yerm 3" (hermetic.ch/cal_stud/palmen/yerm1.htm,
    // read 2026-09-26); a yerm has fifteen or seventeen.
    lunisolar(
        YERM_CALENDARS,
        &[months(&[
            "Month 1", "Month 2", "Month 3", "Month 4", "Month 5", "Month 6", "Month 7", "Month 8",
            "Month 9", "Month 10", "Month 11", "Month 12", "Month 13", "Month 14", "Month 15",
            "Month 16", "Month 17",
        ])],
        "",
    ),
    // Calendars whose own names are in another script, romanised as
    // English-language sources print them; each overrides the names the
    // calendar declares for itself. The Coptic months are the Coptic-derived
    // forms Wikipedia, "Coptic calendar" (`wikipedia-coptic-calendar`),
    // prints, read 2026-09-26, and not the Arabic-derived Tout, Baba… of
    // CLDR 48 `root.xml`. The Ethiopic follow the transliteration system of
    // the *Encyclopaedia Aethiopica* and the Armenian the
    // Hübschmann–Meillet–Benveniste one; no source that prints these two
    // lists is recorded. The Persian are CLDR 48 `root.xml`'s, which
    // `en.xml` inherits, compared 2026-09-26.
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
    // Wikipedia, "Burmese calendar" (`wikipedia-burmese-calendar`),
    // compared 2026-09-26.
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
    // The Khmer months in the UNGEGN romanisation of Wikipedia, "Month",
    // section "Khmer calendar" (revision of 24 September 2026, read
    // 2026-09-26), month 6 as Pĭsakh of the page's "Vĭsakh/Pĭsakh", the
    // form ពិសាខ that Khmer Wikipedia and the New Year announcements use,
    // and the extra Asath of a leap-month year as the page's Bâthâmôsath.
    // The page names the second Tŭtĕyéasath, which is not given as a
    // leap-year name: a leap year here is any year that `is_leap_year`
    // answers for, a year with a 30th day of Jesth among them, which has one
    // Asath. The Khmer script is declared with the calendar's shape
    // (`hc_calendars_regional::khmer::MONTHS`); no `km` locale is shipped.
    CalendarNames {
        calendars: KHMER_CALENDARS,
        cycles: &[months(&[
            "Mĭkôsĕr",
            "Bŏss",
            "Méakh",
            "Phâlkŭn",
            "Chétr",
            "Pĭsakh",
            "Chésth",
            "Asath",
            "Srapôn",
            "Phôtrôbât",
            "Âssŏch",
            "Kâtdĕk",
        ])],
        leap_month_prefix: "",
        eras: EraNames::EMPTY,
        quarters: ContextualNames::EMPTY,
        templates: DateTemplates::NONE,
        leap_names: LeapMonthNames {
            intercalary: &[(8, "Bâthâmôsath")],
            in_leap_years: &[],
        },
    },
    // Wikipedia, "Rumi calendar" (`wikipedia-rumi-calendar`), compared
    // 2026-09-26.
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
    // Wikipedia, "Nanakshahi calendar" (`wikipedia-nanakshahi-calendar`),
    // compared 2026-09-26.
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
    // The Dari months of `persian-afghan` as Wikipedia's "Solar Hijri
    // calendar" romanises them (read 2026-09-26); the calendar declares
    // them in Persian script with its shape.
    dated(
        AFGHAN_SOLAR_HIJRI_CALENDARS,
        &[months(&[
            "Hamal", "Sawr", "Jawzā", "Saratān", "Asad", "Sunbula", "Mīzān", "ʿAqrab", "Qaws",
            "Jadī", "Dalwa", "Hūt",
        ])],
        &[],
        &[],
    ),
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
        templates: DateTemplates::NONE,
        leap_names: LeapMonthNames::NONE,
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
        templates: DateTemplates::NONE,
        leap_names: LeapMonthNames::NONE,
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
        templates: DateTemplates::NONE,
        leap_names: LeapMonthNames::NONE,
    },
    // The Vira Nirvana Samvat runs the same amānta months as `hindu-lunar`,
    // Kārtika first, so they carry the Rashtriya Panchang's English
    // transliteration above in the Jain year's order. The Oshwal
    // Association's calendar of 2025 prints the months in Gujarati forms —
    // Kartik, Posh, Maha, Fagan, Chaitra, Vaishakh, Jeth, Ashadh, Shravan,
    // Bhadarvo, Aaso — but not Mārgaśīrṣa, so those are not used; it heads
    // its pages "Vir Samvat 2551", the abbreviation here. The Devanagari
    // names are the calendar's own, in `hc-calendars-indic`.
    CalendarNames {
        calendars: &[CalendarId("vira-nirvana-samvat")],
        cycles: &[months(&[
            "Kartika",
            "Agrahayana",
            "Pausha",
            "Magha",
            "Phalguna",
            "Chaitra",
            "Vaisakha",
            "Jyaishtha",
            "Ashadha",
            "Sravana",
            "Bhadra",
            "Asvina",
        ])],
        leap_month_prefix: "Adhika ",
        eras: EraNames {
            codes: &["vira-nirvana"],
            names: widths(&["Vira Nirvana Samvat"], &["Vir Samvat"], &[]),
            calendars: &[],
        },
        quarters: ContextualNames::EMPTY,
        templates: DateTemplates::NONE,
        leap_names: LeapMonthNames::NONE,
    },
    // The Odia Anka runs the pūrṇimānta months of `hindu-lunar-purnimanta`,
    // numbered from Chaitra as that calendar numbers them, so they carry the
    // same transliteration; the year opens in Bhadra. The era is the reign
    // the Anka counts, Dibyasingha Deb's, as the Puri press names him on
    // Suniā ("Gajapati Dibyasingha Deb entered his 69th Anka", OdishaTV,
    // 4 September 2025).
    CalendarNames {
        calendars: &[CalendarId("odia-anka")],
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
            codes: &["dibyasingha-deb"],
            names: widths(&["Anka of Dibyasingha Deb"], &["Anka"], &[]),
            calendars: &[],
        },
        quarters: ContextualNames::EMPTY,
        templates: DateTemplates::NONE,
        leap_names: LeapMonthNames::NONE,
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
        templates: DateTemplates::NONE,
        leap_names: LeapMonthNames::NONE,
    },
    // The nineteen Badíʿ months, checked against the length the calendar
    // declares, in the Bahá'í transliteration English-language texts use,
    // which is why they are a locale's and not the calendar's own. No
    // source is recorded for these spellings: `bwc-badi-dates` and
    // `uhj-2014-07-10`, which the calendar follows, were read for its dates
    // and not for its month names.
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
        templates: DateTemplates::NONE,
        leap_names: LeapMonthNames::NONE,
    },
    // Eras only, for calendars whose months English names elsewhere or not at
    // all, placed after every other entry so that each calendar's first entry,
    // and with it its templates, is unchanged; `tests/vocabulary.rs` holds every
    // era a calendar writes to an English name.
    // CLDR 48 `en.xml` names the Śaka, Solar Hijri and Minguo eras; the rest
    // are the names each calendar's module documentation gives its era, from
    // the sources its system document or module cites.
    dated(&[CalendarId("indian")], &[], &["saka"], &["Saka"]),
    dated(SOLAR_HIJRI_CALENDARS, &[], &["ap"], &["AP"]),
    dated(
        &[CalendarId("roc")],
        &[],
        &["broc", "roc"],
        &["Before R.O.C.", "Minguo"],
    ),
    dated(
        &[CalendarId("armenian"), CalendarId("armenian-fixed")],
        &[],
        &["armenian"],
        &["Armenian era"],
    ),
    dated(&[CalendarId("assyrian")], &[], &["ay"], &["A.Y."]),
    dated(&[CalendarId("byzantine")], &[], &["am"], &["Anno Mundi"]),
    dated(&[CalendarId("coptic")], &[], &["am"], &["Anno Martyrum"]),
    dated(&[CalendarId("discordian")], &[], &["yold"], &["YOLD"]),
    dated(
        &[CalendarId("egyptian")],
        &[],
        &["nabonassar"],
        &["Era of Nabonassar"],
    ),
    dated(
        &[CalendarId("ethiopic")],
        &[],
        &["am", "aa"],
        &["Amätä Məḥrät", "Amätä Aläm"],
    ),
    dated(
        &[
            CalendarId("french-republican-arithmetic"),
            CalendarId("french-republican-equinox"),
        ],
        &[],
        &["re"],
        &["Republican era"],
    ),
    dated(
        &[CalendarId("hindu-old-solar")],
        &[],
        &["kali-yuga"],
        &["Kali Yuga"],
    ),
    dated(
        &[CalendarId("hindu-solar-tamil")],
        &[],
        &["saka"],
        &["Saka"],
    ),
    dated(
        &[CalendarId("hindu-solar-malayalam")],
        &[],
        &["kollam"],
        &["Kollam era"],
    ),
    dated(
        &[CalendarId("hindu-solar-bengali")],
        &[],
        &["bangabda"],
        &["Bengali San"],
    ),
    dated(
        &[CalendarId("hindu-solar-vikrami")],
        &[],
        &["vs"],
        &["Vikrama Samvat"],
    ),
    dated(&[CalendarId("holocene")], &[], &["he"], &["Human Era"]),
    dated(
        &[CalendarId("japanese-imperial")],
        &[],
        &["koki"],
        &["Kōki"],
    ),
    dated(&[CalendarId("juche")], &[], &["juche"], &["Juche"]),
    dated(&[CalendarId("mandaean")], &[], &["aa"], &["AA"]),
    dated(&[CalendarId("nanakshahi")], &[], &["ns"], &["Nanakshahi"]),
    dated(&[CalendarId("roman-auc")], &[], &["auc"], &["AUC"]),
    dated(&[CalendarId("rumi")], &[], &["rumi"], &["Rumi"]),
    dated(&[CalendarId("samaritan")], &[], &["entry"], &["Entry Era"]),
    dated(
        &[
            CalendarId("zoroastrian-qadimi"),
            CalendarId("zoroastrian-shahanshahi"),
            CalendarId("zoroastrian-fasli"),
        ],
        &[],
        &["yz"],
        &["Y.Z."],
    ),
    dated(
        &[
            CalendarId("julian-gregorian-catholic"),
            CalendarId("julian-gregorian-fr"),
            CalendarId("julian-gregorian-nl-states-general"),
            CalendarId("julian-gregorian-nl-holland"),
            CalendarId("julian-gregorian-de-catholic"),
            CalendarId("julian-gregorian-hu"),
            CalendarId("julian-gregorian-de-protestant"),
            CalendarId("julian-gregorian-gb"),
            CalendarId("julian-gregorian-se"),
            CalendarId("julian-gregorian-bg"),
            CalendarId("julian-gregorian-ru"),
            CalendarId("julian-gregorian-rs"),
            CalendarId("julian-gregorian-ro"),
            CalendarId("julian-gregorian-gr"),
        ],
        &[],
        &["os", "ns"],
        &["Old Style", "New Style"],
    ),
];

// The week begins on Sunday: CLDR 48's likely subtags give `en` the region
// US (`likelySubtags.xml`, `en` → `en_Latn_US`), which `weekData/firstDay`
// lists under Sunday (`cldr48-supplemental`). `en-GB` and the other
// Monday regions take their region's day first.
const EN: LocaleData = LocaleData {
    tag: "en",
    sources: "Unicode CLDR 48, common/main/en.xml (cldr48-main), its Gregorian months and weekdays compared with it 2026-09-26; the other calendars' vocabulary as the entry's comment states",
    english_name: "English",
    native_name: "English",
    script: "Latn",
    templates: EN_TEMPLATES,
    calendar_names: EN_CALENDAR_NAMES,
    direction: Direction::LeftToRight,
    numbering: "latn",
    first_day_of_week: Weekday::Sunday,
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
        // Pinyin writes the two syllables apart: jia zi.
        joiner: " ",
    },
    calendars: EN_CALENDARS,
};

// --- Spanish --------------------------------------------------------------

const ES: LocaleData = LocaleData {
    tag: "es",
    sources: "Unicode CLDR 48, common/main/es.xml (cldr48-main), its Gregorian months and weekdays compared with it 2026-09-26; the other calendars' vocabulary as the entry's comment states",
    english_name: "Spanish",
    native_name: "español",
    script: "Latn",
    templates: ES_TEMPLATES,
    calendar_names: ES_CALENDAR_NAMES,
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
    sources: "Unicode CLDR 48, common/main/fa.xml (cldr48-main), its Gregorian months and weekdays compared with it 2026-09-26; the other calendars' vocabulary as the entry's comment states",
    english_name: "Persian",
    native_name: "فارسی",
    script: "Arab",
    templates: FA_TEMPLATES,
    calendar_names: FA_CALENDAR_NAMES,
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
            // CLDR 48 `fa.xml`: the format months carry the ezafe (ژانویهٔ, as
            // in «۵ ژانویهٔ ۲۰۲۴»), the stand-alone ones do not.
            &[month_cycle(ContextualNames {
                format: widths(
                    &[
                        "ژانویهٔ",
                        "فوریهٔ",
                        "مارس",
                        "آوریل",
                        "مهٔ",
                        "ژوئن",
                        "ژوئیهٔ",
                        "اوت",
                        "سپتامبر",
                        "اکتبر",
                        "نوامبر",
                        "دسامبر",
                    ],
                    &[],
                    &[],
                ),
                standalone: widths(
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
                ),
            })],
            gregorian_eras(&["قبل از میلاد", "میلادی"], &["ق.م.", "م."], &[]),
            ContextualNames::EMPTY,
        ),
        // The month names are the calendar's own, declared with its shape in
        // `hc-calendars-solar` for Iran's and in `hc-calendars-equinox` for
        // Afghanistan's Dari ones; only the era is a Persian word, and CLDR
        // 48 `fa_AF.xml` states no era of its own for the calendar, so
        // `fa-AF` takes this one.
        dated(SOLAR_HIJRI_CALENDARS, &[], &["ap"], &["ه.ش."]),
    ],
};

// --- French ---------------------------------------------------------------

const FR: LocaleData = LocaleData {
    tag: "fr",
    sources: "Unicode CLDR 48, common/main/fr.xml (cldr48-main), its Gregorian months and weekdays compared with it 2026-09-26; the other calendars' vocabulary as the entry's comment states",
    english_name: "French",
    native_name: "français",
    script: "Latn",
    templates: DAY_MONTH_YEAR_TEMPLATES,
    calendar_names: FR_CALENDAR_NAMES,
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
            &[
                "ינו׳", "פבר׳", "מרץ", "אפר׳", "מאי", "יוני", "יולי", "אוג׳", "ספט׳", "אוק׳",
                "נוב׳", "דצמ׳",
            ],
            &[],
        )))],
        gregorian_eras(&["לפני הספירה", "לספירה"], &[], &[]),
        ContextualNames::EMPTY,
    ),
    // Keyed as the English entry is, Tishri 1 to Elul 12 with Adar I and
    // Adar II beside them; the names are CLDR 48 `he.xml`, `calendar
    // type="hebrew"`.
    dated(
        HEBREW_CALENDARS,
        &[months(&[
            "תשרי",
            "חשוון",
            "כסלו",
            "טבת",
            "שבט",
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
    )
    .with_leap_names(LeapMonthNames {
        intercalary: &[(5, "אדר א׳")],
        in_leap_years: &[(6, "אדר ב׳")],
    })
    // The year alone, as in English: CLDR 48 `he.xml`, `calendar
    // type="hebrew"`, formats a date as "d בMMMM y" with no era.
    .with_templates(HEBREW_TEMPLATES),
];

const HE: LocaleData = LocaleData {
    tag: "he",
    sources: "Unicode CLDR 48, common/main/he.xml (cldr48-main), its Gregorian months and weekdays compared with it 2026-09-26; the other calendars' vocabulary as the entry's comment states",
    english_name: "Hebrew",
    native_name: "עברית",
    script: "Hebr",
    templates: HE_TEMPLATES,
    calendar_names: HE_CALENDAR_NAMES,
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
        &["ב׳", "ג׳", "ד׳", "ה׳", "ו׳", "ש׳", "א׳"],
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
                "अक्टूबर",
                "नवंबर",
                "दिसंबर",
            ],
            &[
                "जन॰",
                "फ़र॰",
                "मार्च",
                "अप्रैल",
                "मई",
                "जून",
                "जुल॰",
                "अग॰",
                "सित॰",
                "अक्टू॰",
                "नव॰",
                "दिस॰",
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
    sources: "Unicode CLDR 48, common/main/hi.xml (cldr48-main), its Gregorian months and weekdays compared with it 2026-09-26; the other calendars' vocabulary as the entry's comment states",
    english_name: "Hindi",
    native_name: "हिन्दी",
    script: "Deva",
    templates: DAY_MONTH_YEAR_TEMPLATES,
    calendar_names: HI_CALENDAR_NAMES,
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
    sources: "Unicode CLDR 48, common/main/id.xml (cldr48-main), its Gregorian months and weekdays compared with it 2026-09-26; the other calendars' vocabulary as the entry's comment states",
    english_name: "Indonesian",
    native_name: "Indonesia",
    script: "Latn",
    templates: DAY_MONTH_YEAR_TEMPLATES,
    calendar_names: ID_CALENDAR_NAMES,
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
    sources: "Unicode CLDR 48, common/main/it.xml (cldr48-main), its Gregorian months and weekdays compared with it 2026-09-26; the other calendars' vocabulary as the entry's comment states",
    english_name: "Italian",
    native_name: "italiano",
    script: "Latn",
    templates: DAY_MONTH_YEAR_TEMPLATES,
    calendar_names: IT_CALENDAR_NAMES,
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
    // The Buddhist era as CLDR 48 names it (`common/main/ja.xml`,
    // `calendar[@type="buddhist"]/eras/eraNames`, read 2026-09-26); its
    // abbreviated and narrow forms inherit the name.
    CalendarNames {
        calendars: BUDDHIST_CALENDARS,
        cycles: &[],
        leap_month_prefix: "",
        eras: EraNames {
            codes: &["be"],
            names: widths(&["仏暦"], &[], &[]),
            calendars: &[],
        },
        quarters: ContextualNames::EMPTY,
        templates: DateTemplates::NONE,
        leap_names: LeapMonthNames::NONE,
    },
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
            // CLDR 48 `ja.xml`, `calendar type="japanese"`, `eraAbbr`.
            names: widths(
                &["明治", "大正", "昭和", "平成", "令和"],
                &[],
                JAPANESE_ERA_NARROW,
            ),
            calendars: &[],
        },
        quarters: ContextualNames::EMPTY,
        templates: DateTemplates::NONE,
        leap_names: LeapMonthNames::NONE,
    },
    // The traditional month names of the Japanese lunisolar calendars,
    // still used for seasonal and literary dates: 師走 is December in
    // feeling, the twelfth lunar month in fact. The twelve as NAOJ's 暦Wiki,
    // 「月の和名」 (`nao-rekiwiki-tsuki-no-wamei`, read 2026-09-26), prints
    // them for the months of the lunisolar calendar. They are Japan's words for
    // Japan's months and serve nothing else — the Chinese calendar is
    // below, by number.
    lunisolar(
        JAPANESE_LUNISOLAR_CALENDARS,
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
    // The Chinese calendar's months in Japanese: the first is 正月 and the
    // rest are numbered, an intercalary month is 閏二月, and the year is
    // written by its stem and branch — the forms the National Astronomical
    // Observatory's calendar notes use for the old calendar (暦Wiki,
    // eco.mtk.nao.ac.jp/koyomi/wiki, "旧暦", read 2026-09-26). The regnal
    // calendar takes the same months with the year of the reign.
    lunisolar(CHINESE_FAMILY_CALENDARS, JA_LUNAR_MONTHS, "閏").with_templates(JA_CHINESE_TEMPLATES),
    lunisolar(CHINESE_REGNAL_CALENDARS, JA_LUNAR_MONTHS, "閏"),
];

const JA_LUNAR_MONTHS: &[CycleNames] = &[months(&[
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
    "十二月",
])];

const JA: LocaleData = LocaleData {
    tag: "ja",
    sources: "Unicode CLDR 48, common/main/ja.xml (cldr48-main), its Gregorian months and weekdays compared with it 2026-09-26; the other calendars' vocabulary as the entry's comment states",
    english_name: "Japanese",
    native_name: "日本語",
    script: "Jpan",
    templates: JA_TEMPLATES,
    calendar_names: JA_CALENDAR_NAMES,
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
        joiner: "",
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
// with Monday in the module's Javanese form Senen (Javanese Wikipedia,
// "Senèn"; Wiktionary, "Senèn", citing Sastra Jawa, Surakarta, 2023) where
// that table prints the Indonesian Senin; it gives Minggu beside Ahad for
// Sunday. That page also prints every one of these names
// in Javanese script (ꦊꦒꦶ, ꦱꦼꦤꦶꦤ꧀ …); they are not carried, because `jv` is
// CLDR's Latin-script locale and a `jv-Java` entry would have no Gregorian
// vocabulary of its own to stand beside them. The week begins on Sunday
// (CLDR 48 `weekData/firstDay`, ID) and the numbering system is `latn`.
// The names are CLDR 48 `localeDisplayNames/languages`: Javanese in
// `en.xml`, Jawa in `jv.xml`. No templates: `jv.xml`'s `Gy` is the
// inheritance marker, resolving to root's "G y", which no template here has
// the shape of.

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
                    "Ahad", "Senen", "Selasa", "Rebo", "Kemis", "Jemuwah", "Setu",
                ],
            ),
        ],
        leap_month_prefix: "",
        leap_names: LeapMonthNames::NONE,
        eras: EraNames::EMPTY,
        quarters: ContextualNames::EMPTY,
        templates: DateTemplates::NONE,
    },
];

const JV: LocaleData = LocaleData {
    tag: "jv",
    sources: "Unicode CLDR 48, common/main/jv.xml (cldr48-main), its Gregorian months and weekdays compared with it 2026-09-26; the other calendars' vocabulary as the entry's comment states",
    english_name: "Javanese",
    native_name: "Jawa",
    script: "Latn",
    templates: DateTemplates::NONE,
    calendar_names: JV_CALENDAR_NAMES,
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
// The names are CLDR 48 `localeDisplayNames/languages`: Kabyle in `en.xml`,
// Taqbaylit in `kab.xml`. No templates: `kab.xml`'s `Gy` is "y G" only as
// `draft="unconfirmed"`, so the convention waits until CLDR confirms it.

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
    sources: "Unicode CLDR 48, common/main/kab.xml (cldr48-main), its Gregorian months and weekdays compared with it 2026-09-26; the other calendars' vocabulary as the entry's comment states",
    english_name: "Kabyle",
    native_name: "Taqbaylit",
    script: "Latn",
    templates: DateTemplates::NONE,
    calendar_names: &[],
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
    sources: "Unicode CLDR 48, common/main/ko.xml (cldr48-main), its Gregorian months and weekdays compared with it 2026-09-26; the other calendars' vocabulary as the entry's comment states",
    english_name: "Korean",
    native_name: "한국어",
    script: "Kore",
    templates: KO_TEMPLATES,
    calendar_names: KO_CALENDAR_NAMES,
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
        joiner: "",
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
    calendars: &[
        gregorian(
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
        ),
        // The Dangi calendar's months are numbered, 1월 … 12월, as CLDR 48
        // `ko.xml` `calendar type="dangi"` numbers them; an intercalary month
        // is 윤 before the number (its `monthPatterns`, leap "윤{0}"); the year
        // is written by its stem and branch in Hangul, as `KO_CHINESE_TEMPLATES`
        // states. The Chinese and Vietnamese calendars share the forms.
        lunisolar(
            CHINESE_FAMILY_CALENDARS,
            &[months(&[
                "1월", "2월", "3월", "4월", "5월", "6월", "7월", "8월", "9월", "10월", "11월",
                "12월",
            ])],
            "윤",
        )
        .with_templates(KO_CHINESE_TEMPLATES),
    ],
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
// The names: Mandaic, as the calendar's sources call it, CLDR's `en.xml`
// having no `mid`; ࡋࡉࡔࡀࡍࡀ ࡖ ࡌࡀࡍࡃࡀࡉࡉࡀ is the Mandaic-script form the
// infobox of Wikipedia, "Mandaic language", read 2026-09-26, prints. No
// CLDR locale, so no templates.

const MID: LocaleData = LocaleData {
    tag: "mid",
    sources: "none from CLDR, which has no mid locale: the weekdays from wikipedia-mandaean-calendar",
    english_name: "Mandaic",
    native_name: "ࡋࡉࡔࡀࡍࡀ ࡖ ࡌࡀࡍࡃࡀࡉࡉࡀ",
    script: "Mand",
    templates: DateTemplates::NONE,
    calendar_names: &[],
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
// The names are CLDR 48 `localeDisplayNames/languages`: Malayalam in
// `en.xml`, മലയാളം in `ml.xml`. No templates: `ml.xml`'s `Gy` and long date
// pattern are both the inheritance marker, resolving to root's.

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
    sources: "Unicode CLDR 48, common/main/ml.xml (cldr48-main), its Gregorian months and weekdays compared with it 2026-09-26; the other calendars' vocabulary as the entry's comment states",
    english_name: "Malayalam",
    native_name: "മലയാളം",
    script: "Mlym",
    templates: DateTemplates::NONE,
    calendar_names: ML_CALENDAR_NAMES,
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
    sources: "Unicode CLDR 48, common/main/my.xml (cldr48-main), its Gregorian months and weekdays compared with it 2026-09-26; the other calendars' vocabulary as the entry's comment states",
    english_name: "Burmese",
    native_name: "မြန်မာ",
    script: "Mymr",
    templates: DateTemplates::NONE,
    calendar_names: MY_CALENDAR_NAMES,
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
// The names: Nahuatl, as the calendar's sources call it, CLDR's `en.xml`
// having no `nah`; the own-language forms the infobox of Wikipedia,
// "Nahuatl", read 2026-09-26, prints (Nawatlahtolli, Mexihkatlahtolli …)
// are in the modern orthography this entry's names are not written in, so
// the English name, which is the word in the classical spelling the entry
// uses, stands for the native one. No CLDR locale, so no templates.

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
        leap_names: LeapMonthNames::NONE,
        eras: EraNames::EMPTY,
        quarters: ContextualNames::EMPTY,
        templates: DateTemplates::NONE,
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
    sources: "none from CLDR, which has no nah locale: the day-signs and months from wikipedia-tonalpohualli and Reingold and Dershowitz (reingold2018)",
    english_name: "Nahuatl",
    native_name: "Nahuatl",
    script: "Latn",
    templates: DateTemplates::NONE,
    calendar_names: &[],
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

// --- Zapotec --------------------------------------------------------------
//
// The language of the Zapotec yza. CLDR has no `zap` locale (CLDR 48 has no
// `common/main/zap.xml`, checked 2026-09-26), so this entry has no
// Gregorian vocabulary, weekdays, day periods or eras. The nineteen months
// and the four year bearers are the forms `hc_calendars_regional::zapotec`
// declares with its shape: the months of Manuscript 85 of the Villa Alta
// corpus in the lower-case spelling Urcid, *Zapotec Hieroglyphic Writing*
// (2001), Table 3.5, prints from Alcina Franch's transcription, and the
// bearers as the colonial Northern Zapotec roots Tavárez and Justeson,
// *Ancient Mesoamerica* 19 (2008), Table 1, give, as
// docs/systems/mesoamerican-years.md sets out; the locale restates them so
// that the language, and not only the calendar, claims them. The week
// begins on Sunday (CLDR 48 `weekData/firstDay`, MX) and numbering is
// `latn`, as for `nah`.
// The names: Zapotec, as the calendar's sources call it. Zapotec has no one
// name for itself — "the name of the language in Zapotec itself varies
// according to the geographical variant" (Wikipedia, "Zapotec languages",
// read 2026-09-26) — and the town Manuscript 85 comes from is not known, so
// the English name stands for the native one. No CLDR locale, so no
// templates.

const ZAP_CALENDARS: &[CalendarNames] = &[
    gregorian(&[], EraNames::EMPTY, ContextualNames::EMPTY),
    dated(
        &[CalendarId("zapotec-yza")],
        &[
            months(&[
                "toohua",
                "huistao",
                "begag",
                "lohuec",
                "yagqueo",
                "gabena",
                "golagoo",
                "cheag",
                "gogaa",
                "gonaa",
                "gaha",
                "tina",
                "zaha",
                "zadii",
                "zohuao",
                "yetilla",
                "yeche",
                "gohui",
                "quicholla",
            ]),
            cycle("year-bearer", &["xoo", "ee", "china", "biaa"]),
        ],
        &[],
        &[],
    ),
];

const ZAP: LocaleData = LocaleData {
    tag: "zap",
    sources: "none from CLDR, which has no zap locale: the months and year bearers from urcid2001 (Table 3.5) and tavarez2008 (Table 1)",
    english_name: "Zapotec",
    native_name: "Zapotec",
    script: "Latn",
    templates: DateTemplates::NONE,
    calendar_names: &[],
    direction: Direction::LeftToRight,
    numbering: "latn",
    first_day_of_week: Weekday::Sunday,
    casing: CasingStyle::Standard,
    capitalises_month_names: false,
    weekdays: ContextualNames::EMPTY,
    day_periods: ContextualNames::EMPTY,
    cycle: SexagenaryNames::EMPTY,
    calendars: ZAP_CALENDARS,
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
    sources: "Unicode CLDR 48, common/main/ne.xml (cldr48-main), its Gregorian months and weekdays compared with it 2026-09-26; the other calendars' vocabulary as the entry's comment states",
    english_name: "Nepali",
    native_name: "नेपाली",
    script: "Deva",
    templates: DateTemplates::NONE,
    calendar_names: NE_CALENDAR_NAMES,
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
    sources: "Unicode CLDR 48, common/main/nl.xml (cldr48-main), its Gregorian months and weekdays compared with it 2026-09-26; the other calendars' vocabulary as the entry's comment states",
    english_name: "Dutch",
    native_name: "Nederlands",
    script: "Latn",
    templates: DAY_MONTH_YEAR_TEMPLATES,
    calendar_names: NL_CALENDAR_NAMES,
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
    sources: "Unicode CLDR 48, common/main/pl.xml (cldr48-main), its Gregorian months and weekdays compared with it 2026-09-26; the other calendars' vocabulary as the entry's comment states",
    english_name: "Polish",
    native_name: "polski",
    script: "Latn",
    templates: DAY_MONTH_YEAR_TEMPLATES,
    calendar_names: PL_CALENDAR_NAMES,
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

// --- Pashto ---------------------------------------------------------------
//
// The second language of Afghanistan's calendar. Gregorian vocabulary from
// CLDR 48 `ps.xml` (read 2026-09-26 from the `release-48` tag of
// github.com/unicode-org/cldr): the months, whose stand-alone forms spell
// February and September differently and whose stand-alone abbreviated
// September is its own; the weekdays, stated only in the wide width; the
// day periods; the eras; the wide quarters, with the narrow ones CLDR
// resolves to root's digits. CLDR's narrow weekdays and its stand-alone
// narrow months resolve, past the markers, to root's Latin letters and
// numerals, which are not a Pashto spelling, so those widths are left to
// fall back.
//
// The Solar Hijri months are the Pashto names of the zodiac signs CLDR's
// `ps.xml` gives under `calendar type="persian"`: وری … کب. CLDR keys them
// to its one `persian` calendar, so they serve `persian`,
// `persian-arithmetic` and `persian-afghan` alike. Wikipedia's "Solar
// Hijri calendar" (read 2026-09-26) prints the same twelve in the Pashto
// letters ګ, ي and ك where CLDR has the Persian گ, ی and ک in four of them
// (غبرګولی, چنګاښ, ليندۍ, كب); the CLDR spellings are the ones carried.
// CLDR gives no Pashto name for the Solar Hijri era, so it inherits.
//
// The week begins on Saturday (CLDR 48 `weekData/firstDay`, AF) and the
// numbering system is `arabext`, `ps.xml`'s `defaultNumberingSystem`.
// The names are CLDR 48 `localeDisplayNames/languages`: Pashto in `en.xml`,
// پښتو in `ps.xml`. No templates: `ps.xml` states none of the patterns a
// template is read from and inherits root's.

const PS_MONTHS: &[&str] = &[
    "جنوري",
    "فبروري",
    "مارچ",
    "اپریل",
    "مۍ",
    "جون",
    "جولای",
    "اګست",
    "سېپتمبر",
    "اکتوبر",
    "نومبر",
    "دسمبر",
];

const PS_MONTHS_STANDALONE: &[&str] = &[
    "جنوري",
    "فېبروري",
    "مارچ",
    "اپریل",
    "مۍ",
    "جون",
    "جولای",
    "اګست",
    "سپتمبر",
    "اکتوبر",
    "نومبر",
    "دسمبر",
];

const PS_MONTHS_STANDALONE_ABBREVIATED: &[&str] = &[
    "جنوري",
    "فبروري",
    "مارچ",
    "اپریل",
    "مۍ",
    "جون",
    "جولای",
    "اګست",
    "سپتمبر",
    "اکتوبر",
    "نومبر",
    "دسمبر",
];

const PS_MONTHS_NARROW: &[&str] = &["ج", "ف", "م", "ا", "م", "ج", "ج", "ا", "س", "ا", "ن", "د"];

/// The Solar Hijri months in Pashto, CLDR 48 `ps.xml`, `calendar
/// type="persian"`.
const PS_SOLAR_HIJRI_MONTHS: &[&str] = &[
    "وری",
    "غویی",
    "غبرگولی",
    "چنگاښ",
    "زمری",
    "وږی",
    "تله",
    "لړم",
    "لیندۍ",
    "مرغومی",
    "سلواغه",
    "کب",
];

const PS_CALENDARS: &[CalendarNames] = &[
    gregorian(
        &[month_cycle(ContextualNames {
            format: widths(PS_MONTHS, &[], PS_MONTHS_NARROW),
            standalone: widths(PS_MONTHS_STANDALONE, PS_MONTHS_STANDALONE_ABBREVIATED, &[]),
        })],
        gregorian_eras(
            &["له میلاد څخه وړاندې", "له میلاد څخه وروسته"],
            &["له میلاد وړاندې", "م."],
            &[],
        ),
        ContextualNames::same(widths(
            &["لومړۍ ربعه", "۲مه ربعه", "۳مه ربعه", "۴مه ربعه"],
            &[],
            &["1", "2", "3", "4"],
        )),
    ),
    dated(
        SOLAR_HIJRI_CALENDARS,
        &[months(PS_SOLAR_HIJRI_MONTHS)],
        &[],
        &[],
    ),
];

const PS: LocaleData = LocaleData {
    tag: "ps",
    sources: "Unicode CLDR 48, common/main/ps.xml (cldr48-main), its Gregorian months and weekdays compared with it 2026-09-26; the other calendars' vocabulary as the entry's comment states",
    english_name: "Pashto",
    native_name: "پښتو",
    script: "Arab",
    templates: DateTemplates::NONE,
    calendar_names: &[],
    direction: Direction::RightToLeft,
    numbering: "arabext",
    first_day_of_week: Weekday::Saturday,
    casing: CasingStyle::Standard,
    capitalises_month_names: false,
    weekdays: ContextualNames::same(weekday_widths(
        &["دونۍ", "درېنۍ", "څلرنۍ", "پينځنۍ", "جمعه", "اونۍ", "يونۍ"],
        &[],
        &[],
        &[],
    )),
    day_periods: ContextualNames::same(widths(&["غ.م.", "غ.و."], &[], &[])),
    cycle: SexagenaryNames::EMPTY,
    calendars: PS_CALENDARS,
};

// --- Portuguese -----------------------------------------------------------

// The week begins on Sunday: CLDR 48's likely subtags give `pt` the region
// BR (`likelySubtags.xml`, `pt` → `pt_Latn_BR`), which `weekData/firstDay`
// lists under Sunday (`cldr48-supplemental`), as it does PT; `pt-AO` and
// `pt-MZ` take their regions' days, Monday and Sunday.
const PT: LocaleData = LocaleData {
    tag: "pt",
    sources: "Unicode CLDR 48, common/main/pt.xml (cldr48-main), its Gregorian months and weekdays compared with it 2026-09-26; the other calendars' vocabulary as the entry's comment states",
    english_name: "Portuguese",
    native_name: "português",
    script: "Latn",
    templates: ES_TEMPLATES,
    calendar_names: PT_CALENDAR_NAMES,
    direction: Direction::LeftToRight,
    numbering: "latn",
    first_day_of_week: Weekday::Sunday,
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
    sources: "Unicode CLDR 48, common/main/ru.xml (cldr48-main), its Gregorian months and weekdays compared with it 2026-09-26; the other calendars' vocabulary as the entry's comment states",
    english_name: "Russian",
    native_name: "русский",
    script: "Cyrl",
    templates: RU_TEMPLATES,
    calendar_names: RU_CALENDAR_NAMES,
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
// months, which end in मासः, the abbreviated and narrow months, the
// weekdays in every width, the wide day periods and the quarters. CLDR 48
// prints an ASCII colon where a visarga (ः) belongs in the abbreviated
// months (जनवरी: …) and in Thursday's wide weekday (गुरुवासर:, beside
// सोमवासरः and the rest); they are transcribed as printed, so that the
// entry says what CLDR 48 says, and the comments at each mark the colon.
// The eras are not written: CLDR's
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
// The names are CLDR 48 `localeDisplayNames/languages`: Sanskrit in
// `en.xml`, संस्कृत भाषा in `sa.xml`. The templates are `sa.xml`'s: `Gy` is
// "y G" and the long date pattern "d MMMM y", the same as `hi.xml`'s.

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
            // CLDR 48 prints the visarga of these abbreviations as an ASCII
            // colon, and they are transcribed as printed.
            &[
                "जनवरी:",
                "फरवरी:",
                "मार्च:",
                "अप्रैल:",
                "मई",
                "जून:",
                "जुलाई:",
                "अगस्त:",
                "सितंबर:",
                "अक्तूबर:",
                "नवंबर:",
                "दिसंबर:",
            ],
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
    sources: "Unicode CLDR 48, common/main/sa.xml (cldr48-main), its Gregorian months and weekdays compared with it 2026-09-26; the other calendars' vocabulary as the entry's comment states",
    english_name: "Sanskrit",
    native_name: "संस्कृत भाषा",
    script: "Deva",
    templates: DAY_MONTH_YEAR_TEMPLATES,
    calendar_names: SA_CALENDAR_NAMES,
    direction: Direction::LeftToRight,
    numbering: "deva",
    first_day_of_week: Weekday::Sunday,
    casing: CasingStyle::Standard,
    capitalises_month_names: false,
    // CLDR 48 writes Thursday's visarga as an ASCII colon in the wide
    // form, transcribed as printed; the short forms resolve to the
    // abbreviations and the narrow ones to the stand-alone narrow.
    weekdays: ContextualNames::same(weekday_widths(
        &[
            "सोमवासरः",
            "मंगलवासरः",
            "बुधवासरः",
            "गुरुवासर:",
            "शुक्रवासरः",
            "शनिवासरः",
            "रविवासरः",
        ],
        &["सोम", "मंगल", "बुध", "गुरु", "शुक्र", "शनि", "रवि"],
        &["सोम", "मंगल", "बुध", "गुरु", "शुक्र", "शनि", "रवि"],
        &["सो", "मं", "बु", "गु", "शु", "श", "र"],
    )),
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
// The names are CLDR 48 `localeDisplayNames/languages`: Syriac in `en.xml`,
// ܣܘܪܝܝܐ in `syr.xml`. No templates: `syr.xml` writes the long date
// "d ܒMMMM y", with a prefixed ܒ no template here has.

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
    sources: "Unicode CLDR 48, common/main/syr.xml (cldr48-main), its Gregorian months and weekdays compared with it 2026-09-26; the other calendars' vocabulary as the entry's comment states",
    english_name: "Syriac",
    native_name: "ܣܘܪܝܝܐ",
    script: "Syrc",
    templates: DateTemplates::NONE,
    calendar_names: SYR_CALENDAR_NAMES,
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
// so one list serves both the national calendar and `hindu-solar-tamil`,
// which counts its years in the same Śaka era, and CLDR's era
// abbreviation சாகா names that era for both. The week begins on Sunday
// (CLDR 48 `weekData/firstDay`, IN) and the numbering system is `latn`.
// The names are CLDR 48 `localeDisplayNames/languages`: Tamil in `en.xml`,
// தமிழ் in `ta.xml`. No templates: `ta.xml` writes the long date "d MMMM,
// y", which no template here has the shape of, and its `Gy` inherits root's.

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

// The sixty year names of the southern cycle, Prabhava first, which the
// Tamil solar year carries (`hc_calendars_indic::samvatsara`, keyed to
// `hindu-solar-tamil` as its shape declares them): the University of
// Madras's *Tamil Lexicon* (1924–1936), entry வருஷம், sense 2, p. 3524,
// which lists all sixty in their three twenties, read 2026-09-26 in the
// Digital Dictionaries of South Asia edition; every one is also a headword
// there glossed as that year of the Jupiter cycle. The Lexicon's spellings
// are kept, ஶ்ரீமுக and தாருண among them, where some almanacs now print
// ஸ்ரீமுக and தாரண.
const TA_SAMVATSARA: &[&str] = &[
    "பிரபவ",
    "விபவ",
    "சுக்கில",
    "பிரமோதூத",
    "பிரசோற்பத்தி",
    "ஆங்கீரச",
    "ஶ்ரீமுக",
    "பவ",
    "யுவ",
    "தாது",
    "ஈசுவர",
    "வெகுதானிய",
    "பிரமாதி",
    "விக்கிரம",
    "விஷு",
    "சித்திரபானு",
    "சுபானு",
    "தாருண",
    "பார்த்திவ",
    "வியய",
    "சர்வசித்து",
    "சர்வதாரி",
    "விரோதி",
    "விகிருதி",
    "கர",
    "நந்தன",
    "விசய",
    "சய",
    "மன்மத",
    "துன்முகி",
    "ஏவிளம்பி",
    "விளம்பி",
    "விகாரி",
    "சார்வரி",
    "பிலவ",
    "சுபகிருது",
    "சோபகிருது",
    "குரோதி",
    "விசுவாவசு",
    "பராபவ",
    "பிலவங்க",
    "கீலக",
    "சௌமிய",
    "சாதாரண",
    "விரோதிகிருது",
    "பரீதாபி",
    "பிரமாதீச",
    "ஆனந்த",
    "இராட்சச",
    "நள",
    "பிங்கள",
    "காளயுக்தி",
    "சித்தார்த்தி",
    "இரௌத்திரி",
    "துன்மதி",
    "துந்துபி",
    "ருதிரோற்காரி",
    "இரத்தாட்சி",
    "குரோதன",
    "அட்சய",
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
        &[months(TA_SOLAR_MONTHS), cycle("samvatsara", TA_SAMVATSARA)],
        &["saka"],
        &["சாகா"],
    ),
];

const TA: LocaleData = LocaleData {
    tag: "ta",
    sources: "Unicode CLDR 48, common/main/ta.xml (cldr48-main), its Gregorian months and weekdays compared with it 2026-09-26; the other calendars' vocabulary as the entry's comment states",
    english_name: "Tamil",
    native_name: "தமிழ்",
    script: "Taml",
    templates: DateTemplates::NONE,
    calendar_names: TA_CALENDAR_NAMES,
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
    sources: "Unicode CLDR 48, common/main/th.xml (cldr48-main), its Gregorian months and weekdays compared with it 2026-09-26; the other calendars' vocabulary as the entry's comment states",
    english_name: "Thai",
    native_name: "ไทย",
    script: "Thai",
    templates: TH_TEMPLATES,
    calendar_names: TH_CALENDAR_NAMES,
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
        &["จันทร์", "อังคาร", "พุธ", "พฤหัส", "ศุกร์", "เสาร์", "อาทิตย์"],
        &["จ.", "อ.", "พ.", "พฤ.", "ศ.", "ส.", "อา."],
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
            templates: DateTemplates::NONE,
            leap_names: LeapMonthNames::NONE,
        },
    ],
};

// --- Turkish --------------------------------------------------------------

const TR: LocaleData = LocaleData {
    tag: "tr",
    sources: "Unicode CLDR 48, common/main/tr.xml (cldr48-main), its Gregorian months and weekdays compared with it 2026-09-26; the other calendars' vocabulary as the entry's comment states",
    english_name: "Turkish",
    native_name: "Türkçe",
    script: "Latn",
    templates: TR_TEMPLATES,
    calendar_names: TR_CALENDAR_NAMES,
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

const VI_STANDALONE_MONTHS: &[&str] = &[
    "Tháng 1",
    "Tháng 2",
    "Tháng 3",
    "Tháng 4",
    "Tháng 5",
    "Tháng 6",
    "Tháng 7",
    "Tháng 8",
    "Tháng 9",
    "Tháng 10",
    "Tháng 11",
    "Tháng 12",
];

const VI: LocaleData = LocaleData {
    tag: "vi",
    sources: "Unicode CLDR 48, common/main/vi.xml (cldr48-main), its Gregorian months and weekdays compared with it 2026-09-26; the other calendars' vocabulary as the entry's comment states",
    english_name: "Vietnamese",
    native_name: "Tiếng Việt",
    script: "Latn",
    templates: VI_TEMPLATES,
    calendar_names: VI_CALENDAR_NAMES,
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
        &["Thứ 2", "Thứ 3", "Thứ 4", "Thứ 5", "Thứ 6", "Thứ 7", "CN"],
        &["T2", "T3", "T4", "T5", "T6", "T7", "CN"],
        &["T2", "T3", "T4", "T5", "T6", "T7", "CN"],
    )),
    day_periods: ContextualNames::same(widths(&["SA", "CH"], &[], &[])),
    cycle: SexagenaryNames {
        reading: Some(&readings::VIETNAMESE),
        joiner: " ",
        // Not the Chinese animals: 丑 is the buffalo and 卯 the cat.
        zodiac: Some(&[
            "Chuột", "Trâu", "Hổ", "Mèo", "Rồng", "Rắn", "Ngựa", "Dê", "Khỉ", "Gà", "Chó", "Lợn",
        ]),
    },
    calendars: &[gregorian(
        // Capitalised when the month stands alone, and not abbreviated.
        &[month_cycle(ContextualNames {
            format: widths(
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
                    "thg 1", "thg 2", "thg 3", "thg 4", "thg 5", "thg 6", "thg 7", "thg 8",
                    "thg 9", "thg 10", "thg 11", "thg 12",
                ],
                &[],
            ),
            standalone: widths(VI_STANDALONE_MONTHS, VI_STANDALONE_MONTHS, &[]),
        })],
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
// The names: Yucatec Maya, as the calendar's sources call it, CLDR's
// `en.xml` having no `yua`; mayaʼ tʼaan is the first form the infobox of
// Wikipedia, "Yucatec Maya language", read 2026-09-26, prints. No CLDR
// locale, so no templates.

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
        leap_names: LeapMonthNames::NONE,
        eras: EraNames::EMPTY,
        quarters: ContextualNames::EMPTY,
        templates: DateTemplates::NONE,
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
        leap_names: LeapMonthNames::NONE,
        eras: EraNames::EMPTY,
        quarters: ContextualNames::EMPTY,
        templates: DateTemplates::NONE,
    },
];

const YUA: LocaleData = LocaleData {
    tag: "yua",
    sources: "none from CLDR, which has no yua locale: the day-signs and months from Reingold and Dershowitz (reingold2018), wikipedia-tzolkin and wikipedia-maya-calendar",
    english_name: "Yucatec Maya",
    native_name: "mayaʼ tʼaan",
    script: "Latn",
    templates: DateTemplates::NONE,
    calendar_names: &[],
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
// The names are CLDR 48 `localeDisplayNames/languages`: Standard Moroccan
// Tamazight in `en.xml`, ⵜⴰⵎⴰⵣⵉⵖⵜ in `zgh.xml`. No templates: `zgh.xml`'s
// `Gy` is the inheritance marker, resolving to root's "G y", which no
// template here has the shape of.

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
    sources: "Unicode CLDR 48, common/main/zgh.xml (cldr48-main), its Gregorian months and weekdays compared with it 2026-09-26; the other calendars' vocabulary as the entry's comment states",
    english_name: "Standard Moroccan Tamazight",
    native_name: "ⵜⴰⵎⴰⵣⵉⵖⵜ",
    script: "Tfng",
    templates: DateTemplates::NONE,
    calendar_names: ZGH_CALENDAR_NAMES,
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
    // The Chinese calendar's own months (CLDR 48 `zh.xml`, `calendar
    // type="chinese"`): 正月 first, 腊月 last, 闰 before a repeated one; the
    // year by its stem and branch and the day by its Han name, as
    // `CHINESE_TEMPLATES` states. The regnal calendar takes the same months
    // with the year of the reign.
    lunisolar(CHINESE_FAMILY_CALENDARS, ZH_HANS_LUNAR_MONTHS, "闰")
        .with_templates(CHINESE_TEMPLATES),
    lunisolar(CHINESE_REGNAL_CALENDARS, ZH_HANS_LUNAR_MONTHS, "闰"),
];

const ZH_HANS_LUNAR_MONTHS: &[CycleNames] = &[months(&[
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
])];

const ZH_HANS: LocaleData = LocaleData {
    tag: "zh-Hans",
    sources: "Unicode CLDR 48, common/main/zh.xml (cldr48-main), its Gregorian months and weekdays compared with it 2026-09-26; the other calendars' vocabulary as the entry's comment states",
    english_name: "Chinese (Simplified)",
    native_name: "简体中文",
    script: "Hans",
    templates: ZH_TEMPLATES,
    calendar_names: ZH_HANS_CALENDAR_NAMES,
    direction: Direction::LeftToRight,
    numbering: "latn",
    // CLDR 48's likely subtags give `zh-Hans` the region CN, which
    // `weekData/firstDay` lists under Monday.
    first_day_of_week: Weekday::Monday,
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
        joiner: "",
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
    // As the simplified entry, in traditional characters (CLDR 48
    // `zh_Hant.xml`, `calendar type="chinese"`).
    lunisolar(CHINESE_FAMILY_CALENDARS, ZH_HANT_LUNAR_MONTHS, "閏")
        .with_templates(CHINESE_TEMPLATES),
    lunisolar(CHINESE_REGNAL_CALENDARS, ZH_HANT_LUNAR_MONTHS, "閏"),
];

const ZH_HANT_LUNAR_MONTHS: &[CycleNames] = &[months(&[
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
])];

const ZH_HANT: LocaleData = LocaleData {
    tag: "zh-Hant",
    sources: "Unicode CLDR 48, common/main/zh_Hant.xml (cldr48-main), its Gregorian months and weekdays compared with it 2026-09-26; the other calendars' vocabulary as the entry's comment states",
    english_name: "Chinese (Traditional)",
    native_name: "繁體中文",
    script: "Hant",
    templates: ZH_TEMPLATES,
    calendar_names: ZH_HANT_CALENDAR_NAMES,
    direction: Direction::LeftToRight,
    numbering: "latn",
    // CLDR 48's likely subtags give `zh-Hant` the region TW, a Sunday
    // region in `weekData/firstDay`.
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
        joiner: "",
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
    NAH, NE, NL, PL, PS, PT, RU, SA, SYR, TA, TH, TR, VI, YUA, ZAP, ZGH, ZH_HANS, ZH_HANT,
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
    fn every_entry_names_its_sources() {
        for data in every_entry() {
            assert!(!data.sources.is_empty(), "{}", data.tag);
            assert!(
                data.sources.contains("CLDR"),
                "{} should say whether CLDR is its source",
                data.tag
            );
        }
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
        assert_eq!(rtl, ["ar", "fa", "he", "mid", "ps", "syr"]);
    }

    #[test]
    fn every_locale_states_its_names_and_its_script() {
        const SCRIPTS: &[&str] = &[
            "Arab", "Beng", "Copt", "Cyrl", "Deva", "Ethi", "Hans", "Hant", "Hebr", "Jpan", "Kore",
            "Latn", "Mand", "Mlym", "Mymr", "Syrc", "Taml", "Tfng", "Thai", "Tibt",
        ];
        for data in LOCALES {
            assert!(!data.english_name.is_empty(), "{}", data.tag);
            assert!(!data.native_name.is_empty(), "{}", data.tag);
            assert!(
                SCRIPTS.contains(&data.script),
                "{}: {}",
                data.tag,
                data.script
            );
        }
        assert_eq!(ROOT.script, "Latn");
        assert!(
            LOCALES
                .iter()
                .filter(|data| data.direction == Direction::RightToLeft)
                .all(|data| matches!(data.script, "Arab" | "Hebr" | "Mand" | "Syrc"))
        );
    }

    /// Every placeholder a template writes is one the renderer fills, and
    /// the day names and implied eras are well formed.
    #[test]
    fn every_template_uses_known_placeholders() {
        const PLACEHOLDERS: &[&str] = &["era", "year", "month", "day", "sexagenary", "extras"];
        const WIDTHS: &[&str] = &["wide", "abbreviated", "narrow", "short"];
        fn check(tag: &str, templates: &DateTemplates) {
            for (field, template) in [
                ("era", templates.era),
                ("year", templates.year),
                ("first_year", templates.first_year),
                ("month", templates.month),
                ("day", templates.day),
                ("date", templates.date),
            ] {
                let mut rest = template;
                while let Some(start) = rest.find('{') {
                    let after = &rest[start + 1..];
                    let end = after
                        .find('}')
                        .unwrap_or_else(|| panic!("{tag} {field}: unclosed placeholder"));
                    let inside = &after[..end];
                    let (name, width) = inside.split_once(':').unwrap_or((inside, "wide"));
                    assert!(
                        PLACEHOLDERS.contains(&name),
                        "{tag} {field}: unknown placeholder {name}"
                    );
                    assert!(
                        WIDTHS.contains(&width),
                        "{tag} {field}: unknown width {width}"
                    );
                    rest = &after[end + 1..];
                }
                assert!(!rest.contains('}'), "{tag} {field}: stray brace");
            }
            assert!(templates.day_names.len() <= 31, "{tag}: too many day names");
            for name in templates.day_names {
                assert!(
                    !name.is_empty() && *name == name.trim(),
                    "{tag}: day name {name:?}"
                );
            }
            assert_eq!(
                templates.implied_era,
                templates.implied_era.to_ascii_lowercase(),
                "{tag}: era codes are written in lower case in the data"
            );
        }
        for data in every_entry() {
            check(data.tag, &data.templates);
            for entry in data.calendars {
                check(data.tag, &entry.templates);
            }
            for entry in data.calendars {
                for (ordinal, name) in entry
                    .leap_names
                    .intercalary
                    .iter()
                    .chain(entry.leap_names.in_leap_years)
                {
                    assert!(*ordinal >= 1, "{}: leap name for month 0", data.tag);
                    assert!(
                        !name.is_empty() && *name == name.trim(),
                        "{}: {name:?}",
                        data.tag
                    );
                    assert!(
                        usize::from(*ordinal)
                            <= entry
                                .months()
                                .get(NameWidth::Wide, NameContext::Format)
                                .len(),
                        "{}: leap name for a month the entry does not name",
                        data.tag
                    );
                }
            }
            for name in data.calendar_names {
                assert!(!name.name.is_empty(), "{}: {}", data.tag, name.calendar.0);
                assert_eq!(
                    data.calendar_names
                        .iter()
                        .filter(|other| other.calendar == name.calendar)
                        .count(),
                    1,
                    "{}: {} named twice",
                    data.tag,
                    name.calendar.0
                );
            }
        }
    }

    #[test]
    fn the_shipped_locale_set_is_the_documented_one() {
        let tags: Vec<&str> = LOCALES.iter().map(|data| data.tag).collect();
        assert_eq!(
            tags,
            [
                "am", "ar", "ban", "bn", "bo", "cop", "cs", "de", "en", "es", "fa", "fr", "he",
                "hi", "id", "it", "ja", "jv", "kab", "ko", "mid", "ml", "my", "nah", "ne", "nl",
                "pl", "ps", "pt", "ru", "sa", "syr", "ta", "th", "tr", "vi", "yua", "zap", "zgh",
                "zh-Hans", "zh-Hant"
            ]
        );
        assert_eq!(ROOT.tag.to_string(), "und");
    }
}
