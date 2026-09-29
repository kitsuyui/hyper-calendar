//! The locale entries read out of Unicode CLDR 48, and its parent locales,
//! generated.
//!
//! **Do not edit.** `scripts/locales-cldr.py` writes this file from CLDR 48
//! (`release-48`, `common/main/<file>.xml` over `root.xml`, and
//! `common/supplemental/supplementalData.xml`; `cldr48-regional-locales`),
//! as the script's documentation says: a language entry carries every
//! group of names its file states, and a regional entry the groups its
//! files resolve to other values than its parent does, so that everything
//! else inherits its parent's, as CLDR's inheritance has it. `super` lists
//! the entries in `LOCALES`.

use hc_calendar::{CalendarId, Weekday};

use super::{
    CHINESE_AND_VIETNAMESE_CALENDARS, CHINESE_TEMPLATES, COPTIC_CALENDARS, HEBREW_CALENDARS,
    ISLAMIC_CALENDARS, calendar_entry, era_names, gregorian, gregorian_eras, lunisolar,
    month_cycle, weekday_widths, widths,
};
use crate::casing::CasingStyle;
use crate::direction::Direction;
use crate::names::{
    CalendarDisplayName, CalendarNames, ContextualNames, DateTemplates, EraNames, LeapMonthNames,
    LocaleData, SexagenaryNames,
};

// --- ar-EG: Arabic (Egypt) ---------------------------------------------------
//
// CLDR 48 `ar_EG.xml`, `ar.xml`. Its files state no group of names apart from
// its parent's. Every other group is the parent entry's, `ar`, as CLDR's
// inheritance gives it. Templates from `Gy` "y G", `d` "d" and the whole date
// "d MMMM y"; digits `arab`.

const AR_EG_TEMPLATES: DateTemplates = DateTemplates {
    year: "{year} {era}",
    date: "{day} {month} {year}",
    ..DateTemplates::NONE
};

const AR_EG_CALENDARS: &[CalendarNames] = &[];

/// The `ar-EG` entry.
pub(super) const AR_EG: LocaleData = LocaleData {
    tag: "ar-EG",
    sources: "Unicode CLDR 48, common/main/ar_EG.xml, common/main/ar.xml (cldr48-regional-locales), read 2026-09-29: every group of names the files resolve apart from the parent",
    english_name: "Arabic (Egypt)",
    native_name: "العربية (مصر)",
    script: super::AR.script,
    direction: super::AR.direction,
    casing: super::AR.casing,
    capitalises_month_names: super::AR.capitalises_month_names,
    templates: AR_EG_TEMPLATES,
    calendar_names: &[],
    numbering: "arab",
    first_day_of_week: Weekday::Saturday,
    weekdays: ContextualNames::EMPTY,
    day_periods: ContextualNames::EMPTY,
    cycle: SexagenaryNames::EMPTY,
    calendars: AR_EG_CALENDARS,
};

// --- en-001: English (world) -------------------------------------------------
//
// CLDR 48 `en_001.xml`, `en.xml`. It carries the day periods and the Gregorian
// months. Every other group is the parent entry's, `en`, as CLDR's inheritance
// gives it. Templates from `Gy` "y G", `d` "d" and the whole date "d MMMM y";
// digits `latn`.

const EN_001_TEMPLATES: DateTemplates = DateTemplates {
    year: "{year} {era}",
    date: "{day} {month} {year}",
    ..DateTemplates::NONE
};

const EN_001_CALENDARS: &[CalendarNames] = &[gregorian(
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
            "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sept", "Oct", "Nov", "Dec",
        ],
        &["J", "F", "M", "A", "M", "J", "J", "A", "S", "O", "N", "D"],
    )))],
    EraNames::EMPTY,
    ContextualNames::EMPTY,
)];

/// The `en-001` entry.
pub(super) const EN_001: LocaleData = LocaleData {
    tag: "en-001",
    sources: "Unicode CLDR 48, common/main/en_001.xml, common/main/en.xml (cldr48-regional-locales), read 2026-09-29: every group of names the files resolve apart from the parent",
    english_name: "English (world)",
    native_name: "English (world)",
    script: super::EN.script,
    direction: super::EN.direction,
    casing: super::EN.casing,
    capitalises_month_names: super::EN.capitalises_month_names,
    templates: EN_001_TEMPLATES,
    calendar_names: &[],
    numbering: "latn",
    first_day_of_week: Weekday::Monday,
    weekdays: ContextualNames::EMPTY,
    day_periods: ContextualNames {
        format: widths(&["am", "pm"], &[], &["a", "p"]),
        standalone: widths(&[], &[], &["am", "pm"]),
    },
    cycle: SexagenaryNames::EMPTY,
    calendars: EN_001_CALENDARS,
};

// --- en-GB: British English --------------------------------------------------
//
// CLDR 48 `en_GB.xml`, `en_001.xml`, `en.xml`. Its files state no group of
// names apart from its parent's. Every other group is the parent entry's,
// `en-001`, as CLDR's inheritance gives it. Templates from `Gy` "y G", `d` "d"
// and the whole date "d MMMM y"; digits `latn`.

const EN_GB_TEMPLATES: DateTemplates = DateTemplates {
    year: "{year} {era}",
    date: "{day} {month} {year}",
    ..DateTemplates::NONE
};

const EN_GB_CALENDARS: &[CalendarNames] = &[];

/// The `en-GB` entry.
pub(super) const EN_GB: LocaleData = LocaleData {
    tag: "en-GB",
    sources: "Unicode CLDR 48, common/main/en_GB.xml, common/main/en_001.xml, common/main/en.xml (cldr48-regional-locales), read 2026-09-29: every group of names the files resolve apart from the parent",
    english_name: "British English",
    native_name: "British English",
    script: EN_001.script,
    direction: EN_001.direction,
    casing: EN_001.casing,
    capitalises_month_names: EN_001.capitalises_month_names,
    templates: EN_GB_TEMPLATES,
    calendar_names: &[],
    numbering: "latn",
    first_day_of_week: Weekday::Monday,
    weekdays: ContextualNames::EMPTY,
    day_periods: ContextualNames::EMPTY,
    cycle: SexagenaryNames::EMPTY,
    calendars: EN_GB_CALENDARS,
};

// --- es-419: Latin American Spanish ------------------------------------------
//
// CLDR 48 `es_419.xml`, `es.xml`. It carries the weekdays, the day periods,
// the quarters, the Gregorian eras, the Minguo eras, the Hijri months, the
// Hebrew months, the Coptic months and the Indian national months and eras.
// Every other group is the parent entry's, `es`, as CLDR's inheritance gives
// it. Templates from `Gy` "y G", `d` "d" and the whole date "d 'de' MMMM 'de'
// y"; digits `latn`.

const ES_419_TEMPLATES: DateTemplates = DateTemplates {
    year: "{year} {era}",
    date: "{day} de {month} de {year}",
    ..DateTemplates::NONE
};

const ES_419_CALENDAR_NAMES: &[CalendarDisplayName] = &[
    CalendarDisplayName::new("islamic-tbla", "calendario islámico tabular"),
    CalendarDisplayName::new("islamic-rgsa", "calendario islámico (Arabia Saudita)"),
];

const ES_419_CALENDARS: &[CalendarNames] = &[
    gregorian(
        &[],
        gregorian_eras(
            &["antes de Cristo", "después de Cristo"],
            &["a.C.", "d.C."],
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
    ),
    calendar_entry(
        &[CalendarId("roc")],
        &[],
        era_names(&["broc", "roc"], &["antes de R.O.C.", "R.O.C."], &[], &[]),
    ),
    calendar_entry(
        ISLAMIC_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
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
            ],
            &[
                "Muh.",
                "Saf.",
                "Rab. I",
                "Rab. II",
                "Jum. I",
                "Jum. II",
                "Raj.",
                "Sha.",
                "Ram.",
                "Shaw.",
                "Dhuʻl-Q.",
                "Dhuʻl-H.",
            ],
            &[],
        )))],
        EraNames::EMPTY,
    ),
    calendar_entry(
        HEBREW_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "Tishri", "Heshvan", "Kislev", "Tevet", "Shevat", "Adar", "Nisan", "Iyar", "Sivan",
                "Tamuz", "Av", "Elul",
            ],
            &[],
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_leap_names(LeapMonthNames {
        intercalary: &[(5, "Adar I")],
        in_leap_years: &[(6, "Adar II")],
        leap_day: None,
    }),
    calendar_entry(
        COPTIC_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "Tout",
                "Baba",
                "Hator",
                "Kiahk",
                "Toba",
                "Amshir",
                "Baramhat",
                "Baramouda",
                "Bashans",
                "Paona",
                "Epep",
                "Mesra",
                "Nasie",
            ],
            &[],
            &[],
        )))],
        EraNames::EMPTY,
    ),
    calendar_entry(
        &[CalendarId("indian")],
        &[month_cycle(ContextualNames::same(widths(
            &[
                "Chaitra",
                "Vaisakha",
                "Jyaistha",
                "Asadha",
                "Sravana",
                "Bhadra",
                "Asvina",
                "Kartika",
                "Agrahayana",
                "Pausa",
                "Magha",
                "Phalguna",
            ],
            &[],
            &[],
        )))],
        era_names(&["saka"], &["Saka"], &[], &[]),
    ),
];

/// The `es-419` entry.
pub(super) const ES_419: LocaleData = LocaleData {
    tag: "es-419",
    sources: "Unicode CLDR 48, common/main/es_419.xml, common/main/es.xml (cldr48-regional-locales), read 2026-09-29: every group of names the files resolve apart from the parent",
    english_name: "Latin American Spanish",
    native_name: "español latinoamericano",
    script: super::ES.script,
    direction: super::ES.direction,
    casing: super::ES.casing,
    capitalises_month_names: super::ES.capitalises_month_names,
    templates: ES_419_TEMPLATES,
    calendar_names: ES_419_CALENDAR_NAMES,
    numbering: "latn",
    first_day_of_week: Weekday::Monday,
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
        &["L", "M", "M", "J", "V", "S", "D"],
    )),
    day_periods: ContextualNames::same(widths(&["a.m.", "p.m."], &[], &[])),
    cycle: SexagenaryNames::EMPTY,
    calendars: ES_419_CALENDARS,
};

// --- mn: Mongolian -----------------------------------------------------------
//
// CLDR 48 `mn.xml`. It carries the weekdays, the day periods, the Gregorian
// months, the quarters and the Gregorian eras. Templates from `Gy` "G y", `d`
// "d" and the whole date "y 'оны' MMMM'ын' d"; digits `latn`.

const MN_TEMPLATES: DateTemplates = DateTemplates {
    year: "{era} {year}",
    date: "{year} оны {month}ын {day}",
    ..DateTemplates::NONE
};

const MN_CALENDAR_NAMES: &[CalendarDisplayName] = &[
    CalendarDisplayName::new("buddhist", "буддын цаглавар"),
    CalendarDisplayName::new("chinese", "хятад цаглавар"),
    CalendarDisplayName::new("coptic", "коптик цаглавар"),
    CalendarDisplayName::new("dangi", "данги цаглавар"),
    CalendarDisplayName::new("ethiopic", "этиоп цаглавар"),
    CalendarDisplayName::new("gregory", "грегорийн цаглавар"),
    CalendarDisplayName::new("hebrew", "еврей цаглавар"),
    CalendarDisplayName::new("indian", "энэтхэгийн үндэсний цаглавар"),
    CalendarDisplayName::new(
        "islamic-civil",
        "исламын цаглавар (хүснэгт, иргэний эрин үе)",
    ),
    CalendarDisplayName::new(
        "islamic-tbla",
        "исламийн цаглавар (хүснэгтэн, одон орны эрин)",
    ),
    CalendarDisplayName::new("islamic-umalqura", "исламын цаглавар (Umm al-Qura)"),
    CalendarDisplayName::new("islamic-rgsa", "исламийн цаглавар (саудын араб, газарзүйн)"),
    CalendarDisplayName::new("iso8601", "ISO-8601 цаглавар"),
    CalendarDisplayName::new("iso8601-week", "ISO-8601 цаглавар (W)"),
    CalendarDisplayName::new("japanese", "япон цаглавар"),
    CalendarDisplayName::new("persian", "перс цаглавар"),
    CalendarDisplayName::new("persian-arithmetic", "перс цаглавар (2820)"),
    CalendarDisplayName::new("roc", "минго цаглавар"),
];

const MN_CALENDARS: &[CalendarNames] = &[gregorian(
    &[month_cycle(ContextualNames {
        format: widths(
            &[
                "нэгдүгээр сар",
                "хоёрдугаар сар",
                "гуравдугаар сар",
                "дөрөвдүгээр сар",
                "тавдугаар сар",
                "зургаадугаар сар",
                "долоодугаар сар",
                "наймдугаар сар",
                "есдүгээр сар",
                "аравдугаар сар",
                "арван нэгдүгээр сар",
                "арван хоёрдугаар сар",
            ],
            &[
                "1-р сар",
                "2-р сар",
                "3-р сар",
                "4-р сар",
                "5-р сар",
                "6-р сар",
                "7-р сар",
                "8-р сар",
                "9-р сар",
                "10-р сар",
                "11-р сар",
                "12-р сар",
            ],
            &[
                "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X", "XI", "XII",
            ],
        ),
        standalone: widths(
            &[
                "Нэгдүгээр сар",
                "Хоёрдугаар сар",
                "Гуравдугаар сар",
                "Дөрөвдүгээр сар",
                "Тавдугаар сар",
                "Зургаадугаар сар",
                "Долоодугаар сар",
                "Наймдугаар сар",
                "Есдүгээр сар",
                "Аравдугаар сар",
                "Арван нэгдүгээр сар",
                "Арван хоёрдугаар сар",
            ],
            &[
                "1-р сар",
                "2-р сар",
                "3-р сар",
                "4-р сар",
                "5-р сар",
                "6-р сар",
                "7-р сар",
                "8-р сар",
                "9-р сар",
                "10-р сар",
                "11-р сар",
                "12-р сар",
            ],
            &[
                "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X", "XI", "XII",
            ],
        ),
    })],
    gregorian_eras(&["манай эриний өмнөх", "манай эриний"], &["МЭӨ", "МЭ"], &[]),
    ContextualNames::same(widths(
        &["1-р улирал", "2-р улирал", "3-р улирал", "4-р улирал"],
        &["I улирал", "II улирал", "III улирал", "IV улирал"],
        &["I", "II", "III", "IV"],
    )),
)];

/// The `mn` entry.
pub(super) const MN: LocaleData = LocaleData {
    tag: "mn",
    sources: "Unicode CLDR 48, common/main/mn.xml (cldr48-regional-locales), read 2026-09-29: every group of names the file states",
    english_name: "Mongolian",
    native_name: "монгол",
    script: "Cyrl",
    direction: Direction::LeftToRight,
    casing: CasingStyle::Standard,
    capitalises_month_names: false,
    templates: MN_TEMPLATES,
    calendar_names: MN_CALENDAR_NAMES,
    numbering: "latn",
    first_day_of_week: Weekday::Monday,
    weekdays: ContextualNames {
        format: weekday_widths(
            &[
                "даваа",
                "мягмар",
                "лхагва",
                "пүрэв",
                "баасан",
                "бямба",
                "ням",
            ],
            &["Да", "Мя", "Лх", "Пү", "Ба", "Бя", "Ня"],
            &[],
            &[],
        ),
        standalone: weekday_widths(
            &[
                "Даваа",
                "Мягмар",
                "Лхагва",
                "Пүрэв",
                "Баасан",
                "Бямба",
                "Ням",
            ],
            &["Да", "Мя", "Лх", "Пү", "Ба", "Бя", "Ня"],
            &[],
            &[],
        ),
    },
    day_periods: ContextualNames::same(widths(&["ү.ө.", "ү.х."], &[], &[])),
    cycle: SexagenaryNames::EMPTY,
    calendars: MN_CALENDARS,
};

// --- shi-Latn: Tachelhit (Latin) ---------------------------------------------
//
// CLDR 48 `shi_Latn.xml`. It carries the weekdays, the day periods, the
// Gregorian months, the quarters and the Gregorian eras. Templates from `Gy`
// "G y", `d` "d" and the whole date "d MMMM y"; digits `latn`.

const SHI_LATN_TEMPLATES: DateTemplates = DateTemplates {
    year: "{era} {year}",
    date: "{day} {month} {year}",
    ..DateTemplates::NONE
};

const SHI_LATN_CALENDARS: &[CalendarNames] = &[
    gregorian(
        &[month_cycle(ContextualNames::same(widths(
            &[
                "innayr",
                "bṛayṛ",
                "maṛṣ",
                "ibrir",
                "mayyu",
                "yunyu",
                "yulyuz",
                "ɣuct",
                "cutanbir",
                "ktubr",
                "nuwanbir",
                "dujanbir",
            ],
            &[
                "inn", "bṛa", "maṛ", "ibr", "may", "yun", "yul", "ɣuc", "cut", "ktu", "nuw", "duj",
            ],
            &["i", "b", "m", "i", "m", "y", "y", "ɣ", "c", "k", "n", "d"],
        )))],
        gregorian_eras(&["dat n ɛisa", "dffir n ɛisa"], &["daɛ", "dfɛ"], &[]),
        ContextualNames::same(widths(
            &["akṛaḍyur 1", "akṛaḍyur 2", "akṛaḍyur 3", "akṛaḍyur 4"],
            &["ak 1", "ak 2", "ak 3", "ak 4"],
            &[],
        )),
    ),
    calendar_entry(
        &[CalendarId("berber")],
        &[month_cycle(ContextualNames::same(widths(
            &[
                "innayr",
                "bṛayṛ",
                "maṛṣ",
                "ibrir",
                "mayyu",
                "yunyu",
                "yulyuz",
                "ɣuct",
                "cutanbir",
                "ktubr",
                "nuwanbir",
                "dujanbir",
            ],
            &[
                "inn", "bṛa", "maṛ", "ibr", "may", "yun", "yul", "ɣuc", "cut", "ktu", "nuw", "duj",
            ],
            &["i", "b", "m", "i", "m", "y", "y", "ɣ", "c", "k", "n", "d"],
        )))],
        EraNames::EMPTY,
    ),
];

/// The `shi-Latn` entry.
pub(super) const SHI_LATN: LocaleData = LocaleData {
    tag: "shi-Latn",
    sources: "Unicode CLDR 48, common/main/shi_Latn.xml (cldr48-regional-locales), read 2026-09-29: every group of names the file states",
    english_name: "Tachelhit (Latin)",
    native_name: "Tashelḥiyt",
    script: "Latn",
    direction: Direction::LeftToRight,
    casing: CasingStyle::Standard,
    capitalises_month_names: false,
    templates: SHI_LATN_TEMPLATES,
    calendar_names: &[],
    numbering: "latn",
    first_day_of_week: Weekday::Monday,
    weekdays: ContextualNames::same(weekday_widths(
        &[
            "aynas",
            "asinas",
            "akṛas",
            "akwas",
            "asimwas",
            "asiḍyas",
            "asamas",
        ],
        &["ayn", "asi", "akṛ", "akw", "asim", "asiḍ", "asa"],
        &[],
        &[],
    )),
    day_periods: ContextualNames::same(widths(&["tifawt", "tadggʷat"], &[], &[])),
    cycle: SexagenaryNames::EMPTY,
    calendars: SHI_LATN_CALENDARS,
};

// --- ur-IN: Urdu (India) -----------------------------------------------------
//
// CLDR 48 `ur_IN.xml`, `ur.xml`. Its files state no group of names apart from
// its parent's. Every other group is the parent entry's, `ur`, as CLDR's
// inheritance gives it. Templates from `Gy` "y G", `d` "d" and the whole date
// "d MMMM، y"; digits `arabext`.

const UR_IN_TEMPLATES: DateTemplates = DateTemplates {
    year: "{year} {era}",
    date: "{day} {month}، {year}",
    ..DateTemplates::NONE
};

const UR_IN_CALENDARS: &[CalendarNames] = &[];

/// The `ur-IN` entry.
pub(super) const UR_IN: LocaleData = LocaleData {
    tag: "ur-IN",
    sources: "Unicode CLDR 48, common/main/ur_IN.xml, common/main/ur.xml (cldr48-regional-locales), read 2026-09-29: every group of names the files resolve apart from the parent",
    english_name: "Urdu (India)",
    native_name: "اردو (بھارت)",
    script: super::UR.script,
    direction: super::UR.direction,
    casing: super::UR.casing,
    capitalises_month_names: super::UR.capitalises_month_names,
    templates: UR_IN_TEMPLATES,
    calendar_names: &[],
    numbering: "arabext",
    first_day_of_week: Weekday::Sunday,
    weekdays: ContextualNames::EMPTY,
    day_periods: ContextualNames::EMPTY,
    cycle: SexagenaryNames::EMPTY,
    calendars: UR_IN_CALENDARS,
};

// --- zh-Hant-HK: Chinese (Traditional, Hong Kong SAR China) ------------------
//
// CLDR 48 `zh_Hant_HK.xml`, `zh_Hant.xml`. It carries the quarters, the
// Gregorian eras and the Chinese months. Every other group is the parent
// entry's, `zh-Hant`, as CLDR's inheritance gives it. Templates from `Gy`
// "Gy年", `d` "d日" and the whole date "y年M月d日"; digits `latn`.

/// The chinese date, the long pattern "U（r）年MMMd".
const ZH_HANT_HK_CHINESE_TEMPLATES: DateTemplates = DateTemplates {
    year: "{sexagenary}（{extra:related-gregorian-year}）年",
    ..CHINESE_TEMPLATES
};

const ZH_HANT_HK_TEMPLATES: DateTemplates = DateTemplates {
    year: "{era}{year}年",
    day: "{day}日",
    date: "{year}{month:abbreviated}{day}",
    ..DateTemplates::NONE
};

const ZH_HANT_HK_CALENDAR_NAMES: &[CalendarDisplayName] =
    &[CalendarDisplayName::new("ethiopic", "埃塞俄比亞曆")];

const ZH_HANT_HK_CALENDARS: &[CalendarNames] = &[
    gregorian(
        &[],
        gregorian_eras(&["公元前", "公元"], &[], &[]),
        ContextualNames::same(widths(
            &["第1季", "第2季", "第3季", "第4季"],
            &["Q1", "Q2", "Q3", "Q4"],
            &[],
        )),
    ),
    lunisolar(
        CHINESE_AND_VIETNAMESE_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
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
            ],
            &[],
            &[
                "正", "二", "三", "四", "五", "六", "七", "八", "九", "十", "十一", "十二",
            ],
        )))],
        "閏",
    )
    .with_templates(ZH_HANT_HK_CHINESE_TEMPLATES),
];

/// The `zh-Hant-HK` entry.
pub(super) const ZH_HANT_HK: LocaleData = LocaleData {
    tag: "zh-Hant-HK",
    sources: "Unicode CLDR 48, common/main/zh_Hant_HK.xml, common/main/zh_Hant.xml (cldr48-regional-locales), read 2026-09-29: every group of names the files resolve apart from the parent",
    english_name: "Chinese (Traditional, Hong Kong SAR China)",
    native_name: "中文（繁體字，中國香港特別行政區）",
    script: super::ZH_HANT.script,
    direction: super::ZH_HANT.direction,
    casing: super::ZH_HANT.casing,
    capitalises_month_names: super::ZH_HANT.capitalises_month_names,
    templates: ZH_HANT_HK_TEMPLATES,
    calendar_names: ZH_HANT_HK_CALENDAR_NAMES,
    numbering: "latn",
    first_day_of_week: Weekday::Sunday,
    weekdays: ContextualNames::EMPTY,
    day_periods: ContextualNames::EMPTY,
    cycle: SexagenaryNames::EMPTY,
    calendars: ZH_HANT_HK_CALENDARS,
};

/// CLDR 48's `parentLocales` (`supplementalData.xml`, the default component):
/// each child with the parent that CLDR's inheritance takes it to in place of
/// its truncated tag, in BCP 47 spelling, `und` for root, sorted by child so
/// that a lookup can search it.
pub static PARENT_LOCALES: &[(&str, &str)] = &[
    ("az-Arab", "und"),
    ("az-Cyrl", "und"),
    ("bal-Latn", "und"),
    ("blt-Latn", "und"),
    ("bm-Nkoo", "und"),
    ("bs-Cyrl", "und"),
    ("byn-Latn", "und"),
    ("cu-Glag", "und"),
    ("dje-Arab", "und"),
    ("dyo-Arab", "und"),
    ("en-150", "en-001"),
    ("en-AG", "en-001"),
    ("en-AI", "en-001"),
    ("en-AT", "en-150"),
    ("en-AU", "en-001"),
    ("en-BB", "en-001"),
    ("en-BE", "en-150"),
    ("en-BM", "en-001"),
    ("en-BS", "en-001"),
    ("en-BW", "en-001"),
    ("en-BZ", "en-001"),
    ("en-CC", "en-001"),
    ("en-CH", "en-150"),
    ("en-CK", "en-001"),
    ("en-CM", "en-001"),
    ("en-CX", "en-001"),
    ("en-CY", "en-001"),
    ("en-CZ", "en-150"),
    ("en-DE", "en-150"),
    ("en-DG", "en-001"),
    ("en-DK", "en-150"),
    ("en-DM", "en-001"),
    ("en-Dsrt", "und"),
    ("en-EE", "en-150"),
    ("en-ER", "en-001"),
    ("en-ES", "en-150"),
    ("en-FI", "en-150"),
    ("en-FJ", "en-001"),
    ("en-FK", "en-001"),
    ("en-FM", "en-001"),
    ("en-FR", "en-150"),
    ("en-GB", "en-001"),
    ("en-GD", "en-001"),
    ("en-GE", "en-150"),
    ("en-GG", "en-001"),
    ("en-GH", "en-001"),
    ("en-GI", "en-001"),
    ("en-GM", "en-001"),
    ("en-GS", "en-001"),
    ("en-GY", "en-001"),
    ("en-HK", "en-001"),
    ("en-HU", "en-150"),
    ("en-ID", "en-001"),
    ("en-IE", "en-001"),
    ("en-IL", "en-001"),
    ("en-IM", "en-001"),
    ("en-IN", "en-001"),
    ("en-IO", "en-001"),
    ("en-IT", "en-150"),
    ("en-JE", "en-001"),
    ("en-JM", "en-001"),
    ("en-KE", "en-001"),
    ("en-KI", "en-001"),
    ("en-KN", "en-001"),
    ("en-KY", "en-001"),
    ("en-LC", "en-001"),
    ("en-LR", "en-001"),
    ("en-LS", "en-001"),
    ("en-LT", "en-150"),
    ("en-LV", "en-150"),
    ("en-MG", "en-001"),
    ("en-MO", "en-001"),
    ("en-MS", "en-001"),
    ("en-MT", "en-001"),
    ("en-MU", "en-001"),
    ("en-MV", "en-001"),
    ("en-MW", "en-001"),
    ("en-MY", "en-001"),
    ("en-NA", "en-001"),
    ("en-NF", "en-001"),
    ("en-NG", "en-001"),
    ("en-NL", "en-150"),
    ("en-NO", "en-150"),
    ("en-NR", "en-001"),
    ("en-NU", "en-001"),
    ("en-NZ", "en-001"),
    ("en-PG", "en-001"),
    ("en-PK", "en-001"),
    ("en-PL", "en-150"),
    ("en-PN", "en-001"),
    ("en-PT", "en-150"),
    ("en-PW", "en-001"),
    ("en-RO", "en-150"),
    ("en-RW", "en-001"),
    ("en-SB", "en-001"),
    ("en-SC", "en-001"),
    ("en-SD", "en-001"),
    ("en-SE", "en-150"),
    ("en-SG", "en-001"),
    ("en-SH", "en-001"),
    ("en-SI", "en-150"),
    ("en-SK", "en-150"),
    ("en-SL", "en-001"),
    ("en-SS", "en-001"),
    ("en-SX", "en-001"),
    ("en-SZ", "en-001"),
    ("en-Shaw", "und"),
    ("en-TC", "en-001"),
    ("en-TK", "en-001"),
    ("en-TO", "en-001"),
    ("en-TT", "en-001"),
    ("en-TV", "en-001"),
    ("en-TZ", "en-001"),
    ("en-UA", "en-150"),
    ("en-UG", "en-001"),
    ("en-VC", "en-001"),
    ("en-VG", "en-001"),
    ("en-VU", "en-001"),
    ("en-WS", "en-001"),
    ("en-ZA", "en-001"),
    ("en-ZM", "en-001"),
    ("en-ZW", "en-001"),
    ("es-AR", "es-419"),
    ("es-BO", "es-419"),
    ("es-BR", "es-419"),
    ("es-BZ", "es-419"),
    ("es-CL", "es-419"),
    ("es-CO", "es-419"),
    ("es-CR", "es-419"),
    ("es-CU", "es-419"),
    ("es-DO", "es-419"),
    ("es-EC", "es-419"),
    ("es-GT", "es-419"),
    ("es-HN", "es-419"),
    ("es-JP", "es-419"),
    ("es-MX", "es-419"),
    ("es-NI", "es-419"),
    ("es-PA", "es-419"),
    ("es-PE", "es-419"),
    ("es-PR", "es-419"),
    ("es-PY", "es-419"),
    ("es-SV", "es-419"),
    ("es-US", "es-419"),
    ("es-UY", "es-419"),
    ("es-VE", "es-419"),
    ("ff-Adlm", "und"),
    ("ff-Arab", "und"),
    ("ha-Arab", "und"),
    ("hi-Latn", "en-IN"),
    ("ht", "fr-HT"),
    ("iu-Latn", "und"),
    ("kaa-Latn", "und"),
    ("kk-Arab", "und"),
    ("kok-Latn", "und"),
    ("ks-Deva", "und"),
    ("ku-Arab", "und"),
    ("kxv-Deva", "und"),
    ("kxv-Orya", "und"),
    ("kxv-Telu", "und"),
    ("ky-Arab", "und"),
    ("ky-Latn", "und"),
    ("ml-Arab", "und"),
    ("mn-Mong", "und"),
    ("mni-Mtei", "und"),
    ("ms-Arab", "und"),
    ("nb", "no"),
    ("nn", "no"),
    ("no-NO", "no"),
    ("pa-Arab", "und"),
    ("pt-AO", "pt-PT"),
    ("pt-CH", "pt-PT"),
    ("pt-CV", "pt-PT"),
    ("pt-FR", "pt-PT"),
    ("pt-GQ", "pt-PT"),
    ("pt-GW", "pt-PT"),
    ("pt-LU", "pt-PT"),
    ("pt-MO", "pt-PT"),
    ("pt-MZ", "pt-PT"),
    ("pt-ST", "pt-PT"),
    ("pt-TL", "pt-PT"),
    ("sat-Deva", "und"),
    ("sd-Deva", "und"),
    ("sd-Khoj", "und"),
    ("sd-Sind", "und"),
    ("shi-Latn", "und"),
    ("so-Arab", "und"),
    ("sr-Latn", "und"),
    ("suz-Sunu", "und"),
    ("sw-Arab", "und"),
    ("tg-Arab", "und"),
    ("ug-Cyrl", "und"),
    ("uz-Arab", "und"),
    ("uz-Cyrl", "und"),
    ("vai-Latn", "und"),
    ("wo-Arab", "und"),
    ("yo-Arab", "und"),
    ("yue-Hans", "und"),
    ("zh-Hant", "und"),
    ("zh-Hant-MO", "zh-Hant-HK"),
];

/// CLDR 48's `numbers/defaultNumberingSystem`, resolved: each carried
/// entry's, and each regional file's whose system is not that of the entry
/// its tag falls back to (`ar-SA`'s `arab` beside `ar`'s `latn`), sorted by tag.
pub static DEFAULT_NUMBERING: &[(&str, &str)] = &[
    ("am", "latn"),
    ("ar", "latn"),
    ("ar-BH", "arab"),
    ("ar-DJ", "arab"),
    ("ar-EG", "arab"),
    ("ar-ER", "arab"),
    ("ar-IL", "arab"),
    ("ar-IQ", "arab"),
    ("ar-JO", "arab"),
    ("ar-KM", "arab"),
    ("ar-KW", "arab"),
    ("ar-LB", "arab"),
    ("ar-MR", "arab"),
    ("ar-OM", "arab"),
    ("ar-PS", "arab"),
    ("ar-QA", "arab"),
    ("ar-SA", "arab"),
    ("ar-SD", "arab"),
    ("ar-SO", "arab"),
    ("ar-SS", "arab"),
    ("ar-SY", "arab"),
    ("ar-TD", "arab"),
    ("ar-YE", "arab"),
    ("bn", "beng"),
    ("bo", "latn"),
    ("cs", "latn"),
    ("de", "latn"),
    ("en", "latn"),
    ("en-001", "latn"),
    ("en-GB", "latn"),
    ("es", "latn"),
    ("es-419", "latn"),
    ("fa", "arabext"),
    ("fil", "latn"),
    ("fr", "latn"),
    ("ha", "latn"),
    ("he", "latn"),
    ("hi", "latn"),
    ("id", "latn"),
    ("it", "latn"),
    ("ja", "latn"),
    ("jv", "latn"),
    ("kab", "latn"),
    ("ko", "latn"),
    ("ml", "latn"),
    ("mn", "latn"),
    ("mr", "deva"),
    ("my", "mymr"),
    ("ne", "deva"),
    ("nl", "latn"),
    ("pa-Arab", "arabext"),
    ("pa-Guru", "latn"),
    ("pcm", "latn"),
    ("pl", "latn"),
    ("ps", "arabext"),
    ("pt", "latn"),
    ("pt-PT", "latn"),
    ("ru", "latn"),
    ("sa", "deva"),
    ("sw", "latn"),
    ("syr", "latn"),
    ("ta", "latn"),
    ("te", "latn"),
    ("th", "latn"),
    ("tr", "latn"),
    ("ur", "latn"),
    ("ur-IN", "arabext"),
    ("vi", "latn"),
    ("yue-Hans", "latn"),
    ("yue-Hant", "latn"),
    ("zgh", "latn"),
    ("zh-Hans", "latn"),
    ("zh-Hant", "latn"),
    ("zh-Hant-HK", "latn"),
];

/// Each carried locale's other numbering systems, CLDR 48's
/// `numbers/otherNumberingSystems`, resolved: (tag, `native`, `traditional`),
/// empty where the locale's files state none.
pub static OTHER_NUMBERING: &[(&str, &str, &str)] = &[
    ("am", "latn", "ethi"),
    ("ar", "arab", ""),
    ("ar-EG", "arab", ""),
    ("bn", "beng", ""),
    ("bo", "tibt", ""),
    ("cs", "latn", ""),
    ("de", "latn", ""),
    ("en", "latn", ""),
    ("en-001", "latn", ""),
    ("en-GB", "latn", ""),
    ("es", "latn", ""),
    ("es-419", "latn", ""),
    ("fa", "arabext", ""),
    ("fil", "latn", ""),
    ("fr", "latn", ""),
    ("ha", "latn", ""),
    ("he", "latn", "hebr"),
    ("hi", "deva", ""),
    ("id", "latn", ""),
    ("it", "latn", ""),
    ("ja", "latn", "jpan"),
    ("jv", "java", ""),
    ("kab", "latn", ""),
    ("ko", "latn", ""),
    ("ml", "mlym", ""),
    ("mn", "latn", ""),
    ("mr", "deva", ""),
    ("my", "mymr", ""),
    ("ne", "deva", ""),
    ("nl", "latn", ""),
    ("pa-Arab", "arabext", ""),
    ("pa-Guru", "guru", ""),
    ("pcm", "latn", ""),
    ("pl", "latn", ""),
    ("ps", "arabext", ""),
    ("pt", "latn", ""),
    ("pt-PT", "latn", ""),
    ("ru", "latn", ""),
    ("sa", "deva", ""),
    ("sw", "latn", ""),
    ("syr", "latn", ""),
    ("ta", "tamldec", "taml"),
    ("te", "telu", ""),
    ("th", "thai", ""),
    ("tr", "latn", ""),
    ("ur", "arabext", ""),
    ("ur-IN", "arabext", ""),
    ("vi", "latn", ""),
    ("yue-Hans", "hanidec", "hant"),
    ("yue-Hant", "hanidec", "hant"),
    ("zgh", "latn", ""),
    ("zh-Hans", "hanidec", "hans"),
    ("zh-Hant", "hanidec", "hant"),
    ("zh-Hant-HK", "hanidec", "hant"),
];
