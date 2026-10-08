//! The other calendars of the hand-written locale entries, read out of
//! Unicode CLDR 48, generated.
//!
//! **Do not edit.** `scripts/locales-cldr.py` writes this file from CLDR 48
//! (`release-48`, `common/main/<file>.xml` over `root.xml`, read 2026-10-09;
//! `cldr48-regional-locales`), as the script's documentation says: for each
//! hand-written entry of `super` that follows a CLDR file, every calendar
//! its files state months or eras for that the entry does not serve itself,
//! with the templates the calendar's own date formats give where they
//! differ from the entry's Gregorian ones. Each entry's `cldr_calendars`
//! field names its constant, and `LocaleData::entries_for` reads it after
//! `calendars`.

use hc_calendar::CalendarId;

use super::{
    BUDDHIST_CALENDARS, COPTIC_CALENDARS, ETHIOPIC_CALENDARS, GENERIC_DATE_CALENDARS,
    HEBREW_CALENDARS, ISLAMIC_CALENDARS, JAPANESE_CALENDARS, PERSIAN_CALENDARS,
    SOLAR_HIJRI_CALENDARS, calendar_entry, era_names, month_cycle, widths,
};
use crate::names::{CalendarNames, ContextualNames, DateTemplates, EraNames, LeapMonthNames};

/// The other calendars of `am`, CLDR 48 `am.xml`: the Hijri months, the Coptic
/// date formats, the Ethiopic date formats, the Japanese date formats and the
/// generic date formats; the hand-written entry serves coptic, ethiopic,
/// gregorian, japanese already.
pub(super) const AM_CLDR: &[CalendarNames] = &[
    calendar_entry(
        ISLAMIC_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "ሙሀረም",
                "ሳፈር",
                "ረቢዑል አወል",
                "ረቢዑል አኺር",
                "ጀማደል አወል",
                "ጀማደል አኺር",
                "ረጀብ",
                "ሻእባን",
                "ረመዳን",
                "ሸዋል",
                "ዙልቂዳህ",
                "ዙልሂጃህ",
            ],
            &[],
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(COPTIC_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(ETHIOPIC_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(JAPANESE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(GENERIC_DATE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
];

/// The other calendars of `ar`, CLDR 48 `ar.xml`: the Buddhist era, the Minguo
/// eras, the Hebrew months and eras, the Ethiopic months, the Persian months,
/// the Persian eras, the Hijri date formats, the Coptic date formats, the
/// Japanese date formats and the generic date formats; the hand-written entry
/// serves coptic, gregorian, islamic, japanese already.
pub(super) const AR_CLDR: &[CalendarNames] = &[
    calendar_entry(
        BUDDHIST_CALENDARS,
        &[],
        era_names(&["be"], &["التقويم البوذي"], &["BE"], &[]),
    )
    .with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        &[CalendarId("roc")],
        &[],
        era_names(&["broc", "roc"], &["BROC", "جمهورية الصي"], &[], &[]),
    )
    .with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        HEBREW_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "تشري",
                "مرحشوان",
                "كيسلو",
                "طيفت",
                "شباط",
                "آذار",
                "نيسان",
                "أيار",
                "سيفان",
                "تموز",
                "آب",
                "أيلول",
            ],
            &[],
            &[],
        )))],
        era_names(&["am"], &["ص"], &[], &[]),
    )
    .with_leap_names(LeapMonthNames {
        intercalary: &[(5, "آذار الأول")],
        in_leap_years: &[(6, "آذار الثاني")],
        leap_day: None,
    })
    .with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        ETHIOPIC_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "مسكريم",
                "تكمت",
                "هدار",
                "تهساس",
                "تر",
                "يكتت",
                "مجابيت",
                "ميازيا",
                "جنبت",
                "سين",
                "هامل",
                "نهاس",
                "باجمن",
            ],
            &[],
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        PERSIAN_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "فرفردن",
                "أذربيهشت",
                "خرداد",
                "تار",
                "مرداد",
                "شهرفار",
                "مهر",
                "آيان",
                "آذر",
                "دي",
                "بهمن",
                "اسفندار",
            ],
            &[],
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        SOLAR_HIJRI_CALENDARS,
        &[],
        era_names(&["ap"], &["ه‍.ش"], &[], &[]),
    )
    .with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(ISLAMIC_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(COPTIC_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(JAPANESE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(GENERIC_DATE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
];

/// The other calendars of `bn`, CLDR 48 `bn.xml`: the Minguo eras, the Hijri
/// months and eras, the Hebrew months, the Coptic months, the Ethiopic months,
/// the Persian months, the Indian national date formats, the Japanese date
/// formats and the generic date formats; the hand-written entry serves
/// gregorian, indian, japanese already.
pub(super) const BN_CLDR: &[CalendarNames] = &[
    calendar_entry(
        &[CalendarId("roc")],
        &[],
        era_names(&["broc", "roc"], &["আগে R.O.C.", "মিঙ্গুয়া"], &[], &[]),
    )
    .with_templates(DateTemplates {
        date: "{day} {month}, {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        ISLAMIC_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "মুহররম",
                "সফর",
                "রবিউল আউয়াল",
                "রবিউস সানি",
                "জমাদিউল আউয়াল",
                "জমাদিউস সানি",
                "রজব",
                "শা‘বান",
                "রমজান",
                "শাওয়াল",
                "জ্বিলকদ",
                "জ্বিলহজ্জ",
            ],
            &[],
            &[
                "১", "২", "৩", "৪", "৫", "৬", "৭", "৮", "৯", "১০", "১১", "১২",
            ],
        )))],
        era_names(&["ah"], &["যুগ"], &[], &[]),
    )
    .with_templates(DateTemplates {
        date: "{day} {month}, {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        HEBREW_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "তিশরি",
                "হেশভান",
                "কিসলেভ",
                "তেভেত",
                "শেভাত",
                "আডার",
                "নিশান",
                "আয়ার",
                "সিভান",
                "তামুজ",
                "অভ",
                "এলুল",
            ],
            &[],
            &[
                "১", "২", "৩", "৪", "৫", "৭", "৮", "৯", "১০", "১১", "১২", "১৩",
            ],
        )))],
        EraNames::EMPTY,
    )
    .with_leap_names(LeapMonthNames {
        intercalary: &[(5, "আডার I")],
        in_leap_years: &[(6, "আডার II")],
        leap_day: None,
    })
    .with_templates(DateTemplates {
        date: "{day} {month}, {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        COPTIC_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "টাউট",
                "বাবা",
                "হাটর",
                "কিয়াক",
                "টোবা",
                "আমশির",
                "বারামহাট",
                "বারামৌডা",
                "বাসহান্স",
                "পাওনা",
                "এপেপ",
                "মেশ্রা",
                "ন্যাশি",
            ],
            &[],
            &[
                "১", "২", "৩", "৪", "৫", "৬", "৭", "৮", "৯", "১০", "১১", "১২", "১৩",
            ],
        )))],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        date: "{day} {month}, {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        ETHIOPIC_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "মাস্কেরেম",
                "টেকেমট",
                "হিডার",
                "তাহসাস",
                "টের",
                "ইয়েকাটিট",
                "মেগাবিট",
                "মিয়াজিয়া",
                "গেনবট",
                "সিনি",
                "হ্যামলি",
                "নেহাসে",
                "পাগুমেন",
            ],
            &[],
            &[
                "১", "২", "৩", "৪", "৫", "৬", "৭", "৮", "৯", "১০", "১১", "১২", "১৩",
            ],
        )))],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        date: "{day} {month}, {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        PERSIAN_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "ফ্যাভার্ডিন",
                "অরডিবেহেশ্ত",
                "খোর্দ্দ",
                "তীর",
                "মর্যাদ",
                "শাহরিবার",
                "মেহের",
                "আবান",
                "বাজার",
                "দে",
                "বাহমান",
                "এসফ্যান্ড",
            ],
            &[
                "ফ্যাভার্ডিন",
                "অরডিবেহেশ্ত",
                "খোর্দ্দ",
                "তীর",
                "মর্যাদ",
                "শাহরিবার",
                "মেহের",
                "আবান",
                "সেপ্ট",
                "দে",
                "বাহমান",
                "এসফ্যান্ড",
            ],
            &[
                "১", "২", "৩", "৪", "৫", "৬", "৭", "৮", "৯", "১০", "১১", "১২",
            ],
        )))],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        date: "{day} {month}, {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(&[CalendarId("indian")], &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month}, {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(JAPANESE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month}, {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(GENERIC_DATE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month}, {year:1} {era}",
        ..DateTemplates::NONE
    }),
];

/// The other calendars of `bo`, CLDR 48 `bo.xml`: the Japanese date formats and
/// the generic date formats; the hand-written entry serves gregorian, japanese
/// already.
pub(super) const BO_CLDR: &[CalendarNames] = &[
    calendar_entry(JAPANESE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{era} སྤྱི་ལོ་{year:1} {month}འི་ཚེས་{day}",
        ..DateTemplates::NONE
    }),
    calendar_entry(GENERIC_DATE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{era} སྤྱི་ལོ་{year:1} {month}འི་ཚེས་{day}",
        ..DateTemplates::NONE
    }),
];

/// The other calendars of `cs`, CLDR 48 `cs.xml`: the Minguo eras, the Hijri
/// months, the Hebrew months, the Coptic months, the Ethiopic months, the
/// Persian months, the Indian national months and eras, the Japanese date
/// formats and the generic date formats; the hand-written entry serves
/// gregorian, japanese already.
pub(super) const CS_CLDR: &[CalendarNames] = &[
    calendar_entry(
        &[CalendarId("roc")],
        &[],
        era_names(&["broc", "roc"], &["před ROC", "ROC"], &[], &[]),
    )
    .with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        ISLAMIC_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "muharrem",
                "safar",
                "rebí’u l-awwal",
                "rebí’u s-sání",
                "džumádá al-úlá",
                "džumádá al-áchira",
                "redžeb",
                "ša’bán",
                "ramadán",
                "šawwal",
                "zú l-ka’da",
                "zú l-hidždža",
            ],
            &[
                "muh.",
                "saf.",
                "reb. I",
                "reb. II",
                "džum. I",
                "džum. II",
                "red.",
                "ša.",
                "ram.",
                "šaw.",
                "zú l-k.",
                "zú l-h.",
            ],
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        HEBREW_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "tišri", "chešvan", "kislev", "tevet", "ševat", "adar", "nisan", "ijar", "sivan",
                "tamuz", "av", "elul",
            ],
            &[],
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_leap_names(LeapMonthNames {
        intercalary: &[(5, "adar I")],
        in_leap_years: &[(6, "adar II")],
        leap_day: None,
    })
    .with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        COPTIC_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "tout",
                "baba",
                "hatour",
                "kiahk",
                "touba",
                "amshir",
                "baramhat",
                "baramouda",
                "bashans",
                "ba’ouna",
                "abib",
                "mesra",
                "nasie",
            ],
            &[],
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        ETHIOPIC_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "meskerem", "tikemet", "hidar", "tahesas", "tir", "yekatit", "megabit", "miyaza",
                "ginbot", "sene", "hamle", "nehase", "pagume",
            ],
            &[],
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        PERSIAN_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "farvardin",
                "ordibehešt",
                "chordád",
                "tír",
                "mordád",
                "šahrívar",
                "mehr",
                "ábán",
                "ázar",
                "dei",
                "bahman",
                "esfand",
            ],
            &[],
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        &[CalendarId("indian")],
        &[month_cycle(ContextualNames::same(widths(
            &[
                "čaitra",
                "vaišákh",
                "džjéšth",
                "ášádh",
                "šrávana",
                "bhádrapad",
                "ášvin",
                "kártik",
                "agrahajana",
                "pauš",
                "mágh",
                "phálgun",
            ],
            &[],
            &[],
        )))],
        era_names(&["saka"], &["Šaka"], &[], &[]),
    )
    .with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(JAPANESE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(GENERIC_DATE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
];

/// The other calendars of `de`, CLDR 48 `de.xml`: the Buddhist era, the Minguo
/// eras, the Hijri months, the Hebrew months, the Coptic months, the Ethiopic
/// months, the Persian months, the Indian national months, the Japanese date
/// formats and the generic date formats; the hand-written entry serves
/// gregorian, japanese already.
pub(super) const DE_CLDR: &[CalendarNames] = &[
    calendar_entry(
        BUDDHIST_CALENDARS,
        &[],
        era_names(&["be"], &["B.E."], &["BE"], &[]),
    )
    .with_templates(DateTemplates {
        date: "{day}. {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        &[CalendarId("roc")],
        &[],
        era_names(
            &["broc", "roc"],
            &["vor Volksrepublik China", "Minguo"],
            &["BROC", "Minguo"],
            &["v. VR China", "Minguo"],
        ),
    )
    .with_templates(DateTemplates {
        date: "{day}. {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        ISLAMIC_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "Muharram",
                "Safar",
                "Rabiʻ I",
                "Rabiʻ II",
                "Dschumada I",
                "Dschumada II",
                "Radschab",
                "Shaʻban",
                "Ramadan",
                "Shawwal",
                "Dhu l-qaʿda",
                "Dhu l-Hiddscha",
            ],
            &[],
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        date: "{day}. {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        HEBREW_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "Tischri",
                "Cheschwan",
                "Kislew",
                "Tevet",
                "Schevat",
                "Adar",
                "Nisan",
                "Ijjar",
                "Siwan",
                "Tammus",
                "Aw",
                "Elul",
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
    })
    .with_templates(DateTemplates {
        date: "{day}. {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        COPTIC_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "Thout",
                "Paopi",
                "Hathor",
                "Koiak",
                "Tobi",
                "Meschir",
                "Paremhat",
                "Paremoude",
                "Paschons",
                "Paoni",
                "Epip",
                "Mesori",
                "Nasie",
            ],
            &[],
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        date: "{day}. {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        ETHIOPIC_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
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
            ],
            &[],
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        date: "{day}. {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        PERSIAN_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "Farwardin",
                "Ordibehescht",
                "Chordād",
                "Tir",
                "Mordād",
                "Schahriwar",
                "Mehr",
                "Ābān",
                "Āsar",
                "Déi",
                "Bahman",
                "Essfand",
            ],
            &[],
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        date: "{day}. {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        &[CalendarId("indian")],
        &[month_cycle(ContextualNames::same(widths(
            &[
                "Chaitra",
                "Vaisakha",
                "Jyaishtha",
                "Ashadha",
                "Sravana",
                "Bhadrapada",
                "Ashvina",
                "Kartika",
                "Margasirsha",
                "Pausha",
                "Magha",
                "Phalguna",
            ],
            &[],
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        date: "{day}. {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(JAPANESE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day}. {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(GENERIC_DATE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day}. {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
];

/// The other calendars of `en`, CLDR 48 `en.xml`: the Buddhist date formats,
/// the Minguo date formats, the Hijri date formats, the Hebrew date formats,
/// the Coptic date formats, the Ethiopic date formats, the Persian date
/// formats, the Indian national date formats, the Japanese date formats and the
/// generic date formats; the hand-written entry serves buddhist, chinese,
/// coptic, dangi, ethiopic, gregorian, hebrew, indian, islamic, japanese,
/// persian, roc already.
pub(super) const EN_CLDR: &[CalendarNames] = &[
    calendar_entry(BUDDHIST_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{month} {day}, {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(&[CalendarId("roc")], &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{month} {day}, {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(ISLAMIC_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{month} {day}, {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(HEBREW_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month} {year:1}",
        ..DateTemplates::NONE
    }),
    calendar_entry(COPTIC_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{month} {day}, {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(ETHIOPIC_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{month} {day}, {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(PERSIAN_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{month} {day}, {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(&[CalendarId("indian")], &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{month} {day}, {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(JAPANESE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{month} {day}, {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(GENERIC_DATE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{month} {day}, {year:1} {era}",
        ..DateTemplates::NONE
    }),
];

/// The other calendars of `es`, CLDR 48 `es.xml`: the Minguo eras, the Hijri
/// months, the Hebrew months, the Coptic months, the Ethiopic months, the
/// Persian months, the Indian national months and eras, the Japanese date
/// formats and the generic date formats; the hand-written entry serves
/// gregorian, japanese already.
pub(super) const ES_CLDR: &[CalendarNames] = &[
    calendar_entry(
        &[CalendarId("roc")],
        &[],
        era_names(&["broc", "roc"], &["antes de RDC", "minguo"], &[], &[]),
    )
    .with_templates(DateTemplates {
        date: "{day} de {month} de {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        ISLAMIC_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "muharram",
                "safar",
                "rabiʻ I",
                "rabiʻ II",
                "jumada I",
                "jumada II",
                "rajab",
                "shaʻban",
                "ramadán",
                "shawwal",
                "dhuʻl-qiʻdah",
                "dhuʻl-hijjah",
            ],
            &[
                "muh.",
                "saf.",
                "rab. I",
                "rab. II",
                "jum. I",
                "jum. II",
                "raj.",
                "sha.",
                "ram.",
                "shaw.",
                "dhuʻl-q.",
                "dhuʻl-h.",
            ],
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        date: "{day} de {month} de {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        HEBREW_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "tishri", "heshvan", "kislev", "tevet", "shevat", "adar", "nisan", "iyar", "sivan",
                "tamuz", "av", "elul",
            ],
            &[],
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_leap_names(LeapMonthNames {
        intercalary: &[(5, "adar I")],
        in_leap_years: &[(6, "adar II")],
        leap_day: None,
    })
    .with_templates(DateTemplates {
        date: "{day} de {month} de {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        COPTIC_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "tout",
                "baba",
                "hator",
                "kiahk",
                "toba",
                "amshir",
                "baramhat",
                "baramouda",
                "bashans",
                "paona",
                "epep",
                "mesra",
                "nasie",
            ],
            &[],
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        date: "{day} de {month} de {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        ETHIOPIC_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "meskerem", "tekemt", "hedar", "tahsas", "ter", "yekatit", "megabit", "miazia",
                "genbot", "sene", "hamle", "nehasse", "pagumen",
            ],
            &[],
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        date: "{day} de {month} de {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        PERSIAN_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "farvardin",
                "ordibehesht",
                "khordad",
                "tir",
                "mordad",
                "shahrivar",
                "mehr",
                "aban",
                "azar",
                "dey",
                "bahman",
                "esfand",
            ],
            &[],
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        date: "{day} de {month} de {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        &[CalendarId("indian")],
        &[month_cycle(ContextualNames::same(widths(
            &[
                "chaitra",
                "vaisakha",
                "jyaistha",
                "asadha",
                "sravana",
                "bhadra",
                "asvina",
                "kartika",
                "agrahayana",
                "pausa",
                "magha",
                "phalguna",
            ],
            &[],
            &[],
        )))],
        era_names(&["saka"], &["saka"], &[], &[]),
    )
    .with_templates(DateTemplates {
        date: "{day} de {month} de {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(JAPANESE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} de {month} de {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(GENERIC_DATE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} de {month} de {year:1} {era}",
        ..DateTemplates::NONE
    }),
];

/// The other calendars of `fa`, CLDR 48 `fa.xml`: the Buddhist era, the Minguo
/// eras, the Hijri months and eras, the Hebrew months and eras, the Coptic
/// months, the Ethiopic months, the Indian national months and eras, the
/// Persian date formats, the Japanese date formats and the generic date
/// formats; the hand-written entry serves gregorian, japanese, persian already.
pub(super) const FA_CLDR: &[CalendarNames] = &[
    calendar_entry(
        BUDDHIST_CALENDARS,
        &[],
        era_names(&["be"], &["تقویم بودایی"], &[], &[]),
    )
    .with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        &[CalendarId("roc")],
        &[],
        era_names(
            &["broc", "roc"],
            &["قبل از R.O.C.", "تقویم مینگو"],
            &[],
            &[],
        ),
    )
    .with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        ISLAMIC_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "محرم",
                "صفر",
                "ربیع‌الاول",
                "ربیع‌الثانی",
                "جمادی‌الاول",
                "جمادی‌الثانی",
                "رجب",
                "شعبان",
                "رمضان",
                "شوال",
                "ذیقعده",
                "ذیحجه",
            ],
            &[],
            &["م", "ص", "ر", "ر", "ج", "ج", "ر", "ش", "ر", "ش", "ذ", "ذ"],
        )))],
        era_names(&["ah"], &["هجری قمری"], &["ه‍.ق."], &[]),
    )
    .with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        HEBREW_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "تشری",
                "حشوان",
                "کسلو",
                "طوت",
                "شباط",
                "واذار",
                "نیسان",
                "ایار",
                "سیوان",
                "تموز",
                "آب",
                "ایلول",
            ],
            &[],
            &["ت", "ح", "ک", "ط", "ش", "و", "ن", "ا", "س", "ت", "آ", "ا"],
        )))],
        era_names(&["am"], &["تقویم عبری"], &[], &[]),
    )
    .with_leap_names(LeapMonthNames {
        intercalary: &[(5, "آذار")],
        in_leap_years: &[(6, "واذار الثانی")],
        leap_day: None,
    })
    .with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        COPTIC_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "توت",
                "پاوی",
                "اثور",
                "کواق",
                "طوفی",
                "ماخیر",
                "فامینوث",
                "فرموثی",
                "پاخون",
                "پاونی",
                "افیفی",
                "ماسوری",
                "ماه کوچک",
            ],
            &[],
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        ETHIOPIC_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "مسکرم",
                "تکیمت",
                "هیدار",
                "طه‌ساز",
                "تر",
                "یکوتیت",
                "مگابیت",
                "میازیا",
                "گین‌بوت",
                "سنه",
                "حمله",
                "نحسه",
                "پاگومه",
            ],
            &[],
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        &[CalendarId("indian")],
        &[month_cycle(ContextualNames::same(widths(
            &[
                "چیتره",
                "ویشاکهه",
                "جییشته",
                "آشادهه",
                "شراونه",
                "بهادره",
                "آشوین",
                "کارتیکه",
                "آگرهینه",
                "پاوشه",
                "ماگهه",
                "پهالگونه",
            ],
            &[],
            &[],
        )))],
        era_names(&["saka"], &["تقویم ساکا"], &[], &[]),
    )
    .with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(PERSIAN_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month} {year:1}",
        ..DateTemplates::NONE
    }),
    calendar_entry(JAPANESE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(GENERIC_DATE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
];

/// The other calendars of `fr`, CLDR 48 `fr.xml`: the Buddhist era, the Hijri
/// months and eras, the Hebrew months and eras, the Coptic months and eras, the
/// Ethiopic months, the Persian months, the Persian eras, the Indian national
/// months and eras, the Minguo date formats, the Japanese date formats and the
/// generic date formats; the hand-written entry serves gregorian, japanese, roc
/// already.
pub(super) const FR_CLDR: &[CalendarNames] = &[
    calendar_entry(
        BUDDHIST_CALENDARS,
        &[],
        era_names(&["be"], &["ère bouddhique"], &["E. B."], &["EB"]),
    )
    .with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        ISLAMIC_CALENDARS,
        &[month_cycle(ContextualNames {
            format: widths(
                &[
                    "mouharram",
                    "safar",
                    "rabia al awal",
                    "rabia ath-thani",
                    "joumada al oula",
                    "joumada ath-thania",
                    "rajab",
                    "chaabane",
                    "ramadan",
                    "chawwal",
                    "dhou al qi`da",
                    "dhou al-hijja",
                ],
                &[
                    "mouh.",
                    "saf.",
                    "rab. aw.",
                    "rab. th.",
                    "joum. oul.",
                    "joum. tha.",
                    "raj.",
                    "chaa.",
                    "ram.",
                    "chaw.",
                    "dhou. q.",
                    "dhou. h.",
                ],
                &[],
            ),
            standalone: widths(
                &[],
                &[
                    "mouh.",
                    "saf.",
                    "rab. aw.",
                    "rab. th.",
                    "joum. ou.",
                    "joum. th.",
                    "raj.",
                    "chaa.",
                    "ram.",
                    "chaw.",
                    "dhou. qi.",
                    "dhou. hi.",
                ],
                &[],
            ),
        })],
        era_names(&["ah"], &["ère de l’Hégire"], &["AH"], &["H"]),
    )
    .with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        HEBREW_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "tichri", "hèchvan", "kislev", "téveth", "chevat", "adar", "nissan", "iyar",
                "sivan", "tamouz", "av", "éloul",
            ],
            &[
                "tich.", "hèch.", "kis.", "tév.", "chev.", "adar", "nis.", "iyar", "siv.", "tam.",
                "av", "él.",
            ],
            &[],
        )))],
        era_names(&["am"], &["Anno Mundi"], &["A. M."], &[]),
    )
    .with_leap_names(LeapMonthNames {
        intercalary: &[(5, "adar I")],
        in_leap_years: &[(6, "adar II")],
        leap_day: None,
    })
    .with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        COPTIC_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "tout",
                "bâbâ",
                "hâtour",
                "kyahk",
                "toubah",
                "amshîr",
                "barmahât",
                "barmoudah",
                "bashans",
                "ba’ounah",
                "abîb",
                "misra",
                "al-nasi",
            ],
            &[
                "tout", "bâb.", "hât.", "kya.", "toub.", "amsh.", "barma.", "barmo.", "bash.",
                "ba’o.", "abî.", "mis.", "al-n.",
            ],
            &[],
        )))],
        era_names(&["am"], &["après Dioclétien"], &["ap. D."], &[]),
    )
    .with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        ETHIOPIC_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "mäskäräm",
                "teqemt",
                "hedar",
                "tahesas",
                "ter",
                "yäkatit",
                "mägabit",
                "miyazya",
                "guenbot",
                "säné",
                "hamlé",
                "nähasé",
                "pagumén",
            ],
            &[
                "mäs.", "teq.", "hed.", "tah.", "ter", "yäk.", "mäg.", "miy.", "gue.", "sän.",
                "ham.", "näh.", "pag.",
            ],
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        PERSIAN_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "farvardin",
                "ordibehešt",
                "khordâd",
                "tir",
                "mordâd",
                "šahrivar",
                "mehr",
                "âbân",
                "âzar",
                "dey",
                "bahman",
                "esfand",
            ],
            &[
                "far.", "ord.", "kho.", "tir", "mor.", "šah.", "mehr", "âbân", "âzar", "dey",
                "bah.", "esf.",
            ],
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        SOLAR_HIJRI_CALENDARS,
        &[],
        era_names(&["ap"], &["Anno Persico"], &["A. P."], &[]),
    )
    .with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        &[CalendarId("indian")],
        &[month_cycle(ContextualNames::same(widths(
            &[
                "chaitra",
                "vaishākh",
                "jyaishtha",
                "āshādha",
                "shrāvana",
                "bhādrapad",
                "āshwin",
                "kārtik",
                "mārgashīrsha",
                "paush",
                "māgh",
                "phālgun",
            ],
            &[
                "chai.", "vai.", "jyai.", "āsha.", "shrā.", "bhā.", "āshw.", "kār.", "mār.",
                "pau.", "māgh", "phāl.",
            ],
            &[],
        )))],
        era_names(&["saka"], &["ère Saka"], &["Śaka"], &[]),
    )
    .with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(&[CalendarId("roc")], &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(JAPANESE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(GENERIC_DATE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
];

/// The other calendars of `he`, CLDR 48 `he.xml`: the Buddhist era, the Hijri
/// months and eras, the Coptic months, the Ethiopic months, the Persian months,
/// the Persian eras, the Indian national months and eras, the Minguo date
/// formats, the Hebrew date formats, the Japanese date formats and the generic
/// date formats; the hand-written entry serves gregorian, hebrew, japanese, roc
/// already.
pub(super) const HE_CLDR: &[CalendarNames] = &[
    calendar_entry(
        BUDDHIST_CALENDARS,
        &[],
        era_names(&["be"], &["הספירה הבודהיסטית"], &["BE"], &[]),
    )
    .with_templates(DateTemplates {
        date: "{day} ב{month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        ISLAMIC_CALENDARS,
        &[month_cycle(ContextualNames {
            format: widths(
                &[
                    "מוחרם",
                    "צפר",
                    "רביע אל-אוול",
                    "רביע א-ת׳אני",
                    "ג׳ומאדא אל-אולא",
                    "ג׳ומאדא א-ת׳אניה",
                    "רג׳ב",
                    "שעבאן",
                    "רמדאן",
                    "שוואל",
                    "ד׳ו אל־קעדה",
                    "ד׳ו אל־חיג׳ה",
                ],
                &[
                    "מוחרם",
                    "צפר",
                    "רביע א׳",
                    "רביע ב׳",
                    "ג׳ומאדא א׳",
                    "ג׳ומאדא ב׳",
                    "רג׳ב",
                    "שעבאן",
                    "רמדאן",
                    "שוואל",
                    "ד׳ו אל־קעדה",
                    "ד׳ו אל־חיג׳ה",
                ],
                &[],
            ),
            standalone: widths(
                &[
                    "מוחרם",
                    "צפר",
                    "רביע אל־אוול",
                    "רביע א־ת׳אני",
                    "ג׳ומאדא אל־אולא",
                    "ג׳ומאדא א־ת׳אניה",
                    "רג׳ב",
                    "שעבאן",
                    "רמדאן",
                    "שוואל",
                    "ד׳ו אל־קעדה",
                    "ד׳ו אל־חיג׳ה",
                ],
                &[
                    "מוחרם",
                    "צפר",
                    "רביע א׳",
                    "רביע ב׳",
                    "ג׳ומאדא א׳",
                    "ג׳ומאדא ב׳",
                    "רג׳ב",
                    "שעבאן",
                    "רמדאן",
                    "שוואל",
                    "ד׳ו אל־קעדה",
                    "ד׳ו אל־חיג׳ה",
                ],
                &[],
            ),
        })],
        era_names(&["ah"], &["שנת היג׳רה"], &["הג׳רה"], &[]),
    )
    .with_templates(DateTemplates {
        date: "{day} ב{month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        COPTIC_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "טאוט",
                "בבה",
                "הטור",
                "קיאק",
                "טובה",
                "אמשיר",
                "ברמהט",
                "ברמודה",
                "בשאנס",
                "פאונה",
                "אפיפ",
                "מסרה",
                "נאסי",
            ],
            &[],
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        date: "{day} ב{month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        ETHIOPIC_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "מסקרם",
                "טקמת",
                "הדר",
                "תהסס",
                "טר",
                "יכתית",
                "מגבית",
                "מיאזיה",
                "גנבות",
                "סאנה",
                "המלה",
                "נהסה",
                "פגומן",
            ],
            &[],
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        date: "{day} ב{month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        PERSIAN_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "פרורדין",
                "ארדיבהשת",
                "ח׳רדאד",
                "תיר",
                "מרדאד",
                "שהריור",
                "מהר",
                "אבאן",
                "אד׳ר",
                "די",
                "בהמן",
                "אספנד",
            ],
            &[],
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        date: "{day} ב{month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        SOLAR_HIJRI_CALENDARS,
        &[],
        era_names(&["ap"], &["הספירה הפרסית"], &["AP"], &[]),
    )
    .with_templates(DateTemplates {
        date: "{day} ב{month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        &[CalendarId("indian")],
        &[month_cycle(ContextualNames::same(widths(
            &[
                "צ׳ייטרה",
                "וייסקהה",
                "ג׳יאסטהה",
                "אשדהה",
                "סראוואנה",
                "בהרדה",
                "אסווינה",
                "קרטיקה",
                "אגרהיאנה",
                "פאוסה",
                "מאגהה",
                "פאלגונה",
            ],
            &[
                "צ׳ייטרה",
                "וייסקהה",
                "ג׳יאסטהה",
                "אשדהה",
                "סראוואנה",
                "בהרדה",
                "אסווינה",
                "קרטיקה",
                "אגרהיאנה",
                "פאוסה",
                "מאגהה",
                "פלגונה",
            ],
            &[],
        )))],
        era_names(&["saka"], &["סאקא"], &[], &[]),
    )
    .with_templates(DateTemplates {
        date: "{day} ב{month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(&[CalendarId("roc")], &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} ב{month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(HEBREW_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} ב{month} {year:1}",
        ..DateTemplates::NONE
    }),
    calendar_entry(JAPANESE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} ב{month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(GENERIC_DATE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} ב{month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
];

/// The other calendars of `hi`, CLDR 48 `hi.xml`: the Buddhist era, the Hijri
/// months, the Ethiopic months, the Persian months, the Indian national date
/// formats, the Japanese date formats and the generic date formats; the
/// hand-written entry serves gregorian, indian, japanese already.
pub(super) const HI_CLDR: &[CalendarNames] = &[
    calendar_entry(
        BUDDHIST_CALENDARS,
        &[],
        era_names(&["be"], &["बौद्ध संवत"], &[], &[]),
    )
    .with_templates(DateTemplates {
        date: "{era} {day} {month} {year:1}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        ISLAMIC_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "मुहर्रम",
                "सफर",
                "राबी प्रथम",
                "राबी द्वितीय",
                "जुम्डा प्रथम",
                "जुम्डा द्वितीय",
                "रजब",
                "शावन",
                "रमजान",
                "शव्व्ल",
                "जिल-क्दाह",
                "जिल्-हिज्जाह",
            ],
            &[],
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        year: "{era} {year}",
        date: "{era} {day} {month} {year:1}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        ETHIOPIC_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "मस्केरेम",
                "टेकेम्ट",
                "हेदर",
                "तहसास",
                "टर",
                "येकाटिट",
                "मेगाबिट",
                "मियाज़िया",
                "गनबोट",
                "सेन",
                "हम्ले",
                "नेहासे",
                "पागूमन",
            ],
            &[],
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        year: "{era} {year}",
        date: "{era} {day} {month} {year:1}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        PERSIAN_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "फर्वादिन",
                "ओर्दिवेहेस्ट",
                "खोरर्दाद",
                "टिर",
                "मोरदाद",
                "शाहरीवर्",
                "मेहर",
                "अवन",
                "अज़र",
                "डे",
                "बहमन",
                "ईस्फन्द्",
            ],
            &[],
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        year: "{era} {year}",
        date: "{era} {day} {month} {year:1}",
        ..DateTemplates::NONE
    }),
    calendar_entry(&[CalendarId("indian")], &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(JAPANESE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        year: "{era} {year}",
        date: "{era} {day} {month} {year:1}",
        ..DateTemplates::NONE
    }),
    calendar_entry(GENERIC_DATE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        year: "{era} {year}",
        date: "{era} {day} {month} {year:1}",
        ..DateTemplates::NONE
    }),
];

/// The other calendars of `id`, CLDR 48 `id.xml`: the Buddhist era, the Minguo
/// eras, the Hijri months and eras, the Indian national eras, the Japanese date
/// formats and the generic date formats; the hand-written entry serves
/// gregorian, japanese already.
pub(super) const ID_CLDR: &[CalendarNames] = &[
    calendar_entry(
        BUDDHIST_CALENDARS,
        &[],
        era_names(&["be"], &["Era Buddhis"], &["EB"], &[]),
    )
    .with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        &[CalendarId("roc")],
        &[],
        era_names(&["broc", "roc"], &["Sebelum R.O.C.", "ROC"], &[], &[]),
    )
    .with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        ISLAMIC_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "Muharam",
                "Safar",
                "Rabiulawal",
                "Rabiulakhir",
                "Jumadilawal",
                "Jumadilakhir",
                "Rajab",
                "Syakban",
                "Ramadan",
                "Syawal",
                "Zulkaidah",
                "Zulhijah",
            ],
            &[
                "Muh.",
                "Saf.",
                "Rab. Awal",
                "Rab. Akhir",
                "Jum. Awal",
                "Jum. Akhir",
                "Raj.",
                "Sya.",
                "Ram.",
                "Syaw.",
                "Zulka.",
                "Zulhi.",
            ],
            &[],
        )))],
        era_names(&["ah"], &["H"], &[], &[]),
    )
    .with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        &[CalendarId("indian")],
        &[],
        era_names(&["saka"], &["SAKA"], &[], &[]),
    )
    .with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(JAPANESE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(GENERIC_DATE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
];

/// The other calendars of `it`, CLDR 48 `it.xml`: the Buddhist era, the Minguo
/// eras, the Japanese date formats and the generic date formats; the
/// hand-written entry serves gregorian, japanese already.
pub(super) const IT_CLDR: &[CalendarNames] = &[
    calendar_entry(
        BUDDHIST_CALENDARS,
        &[],
        era_names(&["be"], &["EB"], &[], &[]),
    )
    .with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        &[CalendarId("roc")],
        &[],
        era_names(&["broc", "roc"], &["Prima di R.O.C.", "Minguo"], &[], &[]),
    )
    .with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(JAPANESE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(GENERIC_DATE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
];

/// The other calendars of `ja`, CLDR 48 `ja.xml`: the Hijri months, the Hebrew
/// months, the Coptic months, the Ethiopic months, the Persian months and the
/// Indian national months and eras; the hand-written entry serves buddhist,
/// chinese, dangi, gregorian, japanese, roc already.
pub(super) const JA_CLDR: &[CalendarNames] = &[
    calendar_entry(
        ISLAMIC_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "ムハッラム",
                "サフアル",
                "ラビー・ウル・アウワル",
                "ラビー・ウッ・サーニー",
                "ジュマーダル・アウワル",
                "ジュマーダッサーニー",
                "ラジャブ",
                "シャアバーン",
                "ラマダーン",
                "シャウワール",
                "ズル・カイダ",
                "ズル・ヒッジャ",
            ],
            &[],
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        date: "{era}{year:1}年{month:1}月{day}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        HEBREW_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "ティスレ",
                "へシボン",
                "キスレブ",
                "テベット",
                "シバット",
                "アダル",
                "ニサン",
                "イヤル",
                "シバン",
                "タムズ",
                "アヴ",
                "エルル",
            ],
            &[],
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_leap_names(LeapMonthNames {
        intercalary: &[(5, "アダル I")],
        in_leap_years: &[(6, "アダル II")],
        leap_day: None,
    })
    .with_templates(DateTemplates {
        date: "{era}{year:1}年{month}{day}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        COPTIC_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "トウト",
                "ババ",
                "ハトール",
                "キアック",
                "トーバ",
                "アムシール",
                "バラムハート",
                "バラモウダ",
                "バシャンス",
                "パオーナ",
                "エペープ",
                "メスラ",
                "ナシエ",
            ],
            &[],
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        date: "{era}{year:1}年{month:1}月{day}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        ETHIOPIC_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "メスケレム",
                "テケムト",
                "ヘダル",
                "ターサス",
                "テル",
                "イェカティト",
                "メガビト",
                "ミアジア",
                "ゲンボト",
                "セネ",
                "ハムレ",
                "ネハッセ",
                "パグメン",
            ],
            &[],
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        date: "{era}{year:1}年{month:1}月{day}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        PERSIAN_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "ファルヴァルディーン",
                "オルディーベヘシュト",
                "ホルダード",
                "ティール",
                "モルダード",
                "シャハリーヴァル",
                "メフル",
                "アーバーン",
                "アーザル",
                "デイ",
                "バフマン",
                "エスファンド",
            ],
            &[],
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        date: "{era}{year:1}年{month:1}月{day}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        &[CalendarId("indian")],
        &[month_cycle(ContextualNames::same(widths(
            &[
                "カイトラ",
                "ヴァイサカ",
                "ジャイスタ",
                "アーサダ",
                "スラバナ",
                "バードラ",
                "アスビナ",
                "カルディカ",
                "アヴラハヤナ",
                "パウサ",
                "マーガ",
                "パルグナ",
            ],
            &[],
            &[],
        )))],
        era_names(&["saka"], &["サカ"], &[], &[]),
    )
    .with_templates(DateTemplates {
        date: "{era}{year:1}年{month:1}月{day}",
        ..DateTemplates::NONE
    }),
];

/// The other calendars of `jv`, CLDR 48 `jv.xml`: the Hijri months, the
/// Japanese date formats and the generic date formats; the hand-written entry
/// serves gregorian, japanese already.
pub(super) const JV_CLDR: &[CalendarNames] = &[
    calendar_entry(
        ISLAMIC_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "Sura",
                "Sapar",
                "Mulud",
                "Bakda Mulud",
                "Jumadilawal",
                "Jumadilakir",
                "Rejeb",
                "Ruwah",
                "Pasa",
                "Sawal",
                "Selo",
                "Besar",
            ],
            &[
                "Sur.", "Sap.", "Mul.", "B. Mul.", "Jum. Aw.", "Jum. Ak.", "Rej.", "Ruw.", "Pso.",
                "Shaw.", "Slo.", "Bsar.",
            ],
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        year: "{year} {era}",
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(JAPANESE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        year: "{year} {era}",
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(GENERIC_DATE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        year: "{year} {era}",
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
];

/// The other calendars of `kab`, CLDR 48 `kab.xml`: the Japanese date formats
/// and the generic date formats; the hand-written entry serves gregorian,
/// japanese already.
pub(super) const KAB_CLDR: &[CalendarNames] = &[
    calendar_entry(JAPANESE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(GENERIC_DATE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
];

/// The other calendars of `ko`, CLDR 48 `ko.xml`: the Buddhist era, the Hijri
/// months and eras, the Hebrew months and eras, the Coptic months, the Ethiopic
/// months and the Persian months; the hand-written entry serves chinese, dangi,
/// gregorian, japanese, roc already.
pub(super) const KO_CLDR: &[CalendarNames] = &[
    calendar_entry(
        BUDDHIST_CALENDARS,
        &[],
        era_names(&["be"], &["불기"], &[], &[]),
    )
    .with_templates(DateTemplates {
        date: "{era} {year:1}년 {month:1}월 {day}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        ISLAMIC_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "무하람",
                "사파르",
                "라비 알 아왈",
                "라비 알 쎄니",
                "주마다 알 아왈",
                "주마다 알 쎄니",
                "라잡",
                "쉐아반",
                "라마단",
                "쉐왈",
                "듀 알 까다",
                "듀 알 히자",
            ],
            &[],
            &[],
        )))],
        era_names(&["ah"], &["히즈라력"], &["AH"], &[]),
    )
    .with_templates(DateTemplates {
        date: "{era} {year:1}년 {month:1}월 {day}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        HEBREW_CALENDARS,
        &[month_cycle(ContextualNames {
            format: widths(
                &[
                    "디스리",
                    "말케스",
                    "기슬르",
                    "데벳",
                    "스밧",
                    "아달",
                    "닛산",
                    "이야르",
                    "시완",
                    "담무르",
                    "압",
                    "엘룰",
                ],
                &[],
                &[],
            ),
            standalone: widths(
                &[
                    "디스리월",
                    "말케스월",
                    "기슬르월",
                    "데벳월",
                    "스밧월",
                    "아달월",
                    "닛산월",
                    "이야르월",
                    "시완월",
                    "담무르월",
                    "압월",
                    "엘룰월",
                ],
                &[],
                &[],
            ),
        })],
        era_names(&["am"], &["유대력"], &["AM"], &[]),
    )
    .with_leap_names(LeapMonthNames {
        intercalary: &[(5, "아달 1")],
        in_leap_years: &[(6, "아달 2")],
        leap_day: None,
    })
    .with_templates(DateTemplates {
        date: "{era} {year:1}년 {month} {day}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        COPTIC_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "투트",
                "바바흐",
                "하투르",
                "키야흐크",
                "투바흐",
                "암쉬르",
                "바라마트",
                "바라문다흐",
                "바샨스",
                "바우나흐",
                "아비브",
                "미스라",
                "나시",
            ],
            &[],
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        date: "{era} {year:1}년 {month:1}월 {day}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        ETHIOPIC_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "매스캐램",
                "테켐트",
                "헤다르",
                "타흐사스",
                "테르",
                "얘카티트",
                "매가비트",
                "미야지야",
                "겐보트",
                "새네",
                "함레",
                "내하세",
                "파구맨",
            ],
            &[],
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        date: "{era} {year:1}년 {month:1}월 {day}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        PERSIAN_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "화르바딘",
                "오르디베헤쉬트",
                "호르다드",
                "티르",
                "모르다드",
                "샤흐리바르",
                "메흐르",
                "아반",
                "아자르",
                "다이",
                "바흐만",
                "에스판드",
            ],
            &[],
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        date: "{era} {year:1}년 {month:1}월 {day}",
        ..DateTemplates::NONE
    }),
];

/// The other calendars of `ml`, CLDR 48 `ml.xml`: the Minguo eras, the Hijri
/// months and eras, the Hebrew months, the Coptic months, the Ethiopic months,
/// the Persian months, the Indian national date formats, the Japanese date
/// formats and the generic date formats; the hand-written entry serves
/// gregorian, indian, japanese already.
pub(super) const ML_CLDR: &[CalendarNames] = &[
    calendar_entry(
        &[CalendarId("roc")],
        &[],
        era_names(
            &["broc", "roc"],
            &["R.O.C-യ്‌ക്ക് മുമ്പ്", "മിംഗ്വോ"],
            &["R.O.C-യ്‌ക്ക് മു.", "മിംഗ്വോ"],
            &[],
        ),
    )
    .with_templates(DateTemplates {
        date: "{era} {year:1} {month} {day}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        ISLAMIC_CALENDARS,
        &[month_cycle(ContextualNames {
            format: widths(
                &[
                    "മുഹറം",
                    "സഫർ",
                    "റബീഹുൽ അവ്വൽ",
                    "റബീഹുൽ ആഖിർ",
                    "ജമാദുൽ അവ്വൽ",
                    "ജമാദുൽ ആഖിർ",
                    "റജബ്",
                    "ശഹബാൻ",
                    "റമളാൻ",
                    "ശവ്വാൽ",
                    "ദുൽ ഖഹദ്",
                    "ദുൽ ഹിജ്ജ",
                ],
                &[
                    "മുഹ.",
                    "സഫ.",
                    "റബീഹുൽ അവ്വ.",
                    "റബീഹുൽ ആഖി.",
                    "ജമാദുൽ അവ്വ.",
                    "ജമാദുൽ ആഖി.",
                    "റജ.",
                    "ശഹബാ.",
                    "റമദാ.",
                    "ശവ്വാ.",
                    "ദുൽ ഖഹ.",
                    "ദുൽ ഹി.",
                ],
                &["മു", "സ", "റ", "റ", "ജ", "ജ", "റ", "ശ", "റ", "ശ", "ദു", "ദു"],
            ),
            standalone: widths(
                &[
                    "മുഹറം",
                    "സഫർ",
                    "റബീഹുൽ അവ്വൽ",
                    "റബീഹുൽ ആഖിർ",
                    "ജമാദുൽ അവ്വൽ",
                    "ജമാദുൽ ആഖിർ",
                    "റജബ്",
                    "ശഹബാൻ",
                    "റമദാൻ",
                    "ശവ്വാൽ",
                    "ദുൽ ഖഹദ്",
                    "ദുൽ ഹിജ്ജ",
                ],
                &[
                    "മുഹ.",
                    "സഫ.",
                    "റബീഹുൽ അവ്വ.",
                    "റബീഹുൽ ആഖി.",
                    "ജമാദുൽ അവ്വ.",
                    "ജമാദുൽ ആഖി.",
                    "റജ.",
                    "ശഹബാ.",
                    "റമദാ.",
                    "ശവ്വാ.",
                    "ദുൽ ഖഹ.",
                    "ദുൽ ഹി.",
                ],
                &["മു", "സ", "റ", "റ", "ജ", "ജ", "റ", "ശ", "റ", "ശ", "ദു", "ദു"],
            ),
        })],
        era_names(&["ah"], &["ഹിജറ"], &[], &[]),
    )
    .with_templates(DateTemplates {
        date: "{era} {year:1} {month} {day}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        HEBREW_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "തിഷ്റി",
                "ഹെഷ്‌വൻ",
                "കിസ്‌ലെവ്",
                "ടിവെറ്റ്",
                "സീബാറ്റ്",
                "അദാർ",
                "നിസാൻ",
                "ഇയാർ",
                "സിവാൻ",
                "താമൂസ്",
                "അബ്",
                "ഏലുൾ",
            ],
            &[],
            &[
                "തി.", "ഹെ.", "കി.", "ടി.", "സീ.", "അ.", "നി.", "ഇ.", "സി.", "താ.", "അ.", "ഏ.",
            ],
        )))],
        EraNames::EMPTY,
    )
    .with_leap_names(LeapMonthNames {
        intercalary: &[(5, "അദാർ I")],
        in_leap_years: &[(6, "അദാർ II")],
        leap_day: None,
    })
    .with_templates(DateTemplates {
        date: "{era} {year:1} {month} {day}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        COPTIC_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "ടൗട്ട്",
                "ബാബ",
                "ഹാറ്റർ",
                "കിയാക്ക്",
                "ടോബ",
                "ആംഷിർ",
                "ബാരംഹാത്ത്",
                "ബാരമൗഡ",
                "ബാഷൻസ്",
                "പവോണ",
                "ഈപെപ്",
                "മെസ്ര",
                "നസീ",
            ],
            &[],
            &[
                "ടൗ.", "ബാ.", "ഹാ.", "കി.", "ടോ.", "ആം.", "ബാ.", "ബാ.", "ബാ.", "പ.", "ഈ.", "മെ.", "ന.",
            ],
        )))],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        date: "{era} {year:1} {month} {day}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        ETHIOPIC_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "മെസ്‌കെരെം",
                "ടെക്കെംറ്റ്",
                "ഹേദർ",
                "തഹ്‌സാസ്",
                "ടെർ",
                "യെക്കാറ്റിറ്റ്",
                "മെഗാബിറ്റ്",
                "മിയാസിയ",
                "ഗെൻബോട്ട്",
                "സെനെ",
                "ഹാംലെ",
                "നെഹാസെ",
                "പാഗുമെൻ",
            ],
            &[],
            &[
                "മെ.",
                "ടെ.",
                "ഹേ.",
                "ത.",
                "ടെ.",
                "യെ.",
                "മെ.",
                "മി.",
                "ഗെ.",
                "സെ.",
                "ഹാം",
                "നെ.",
                "പാ.",
            ],
        )))],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        date: "{era} {year:1} {month} {day}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        PERSIAN_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "ഫർവാർദിൻ",
                "ഓർഡിബെഹെഷ്‌റ്റ്",
                "ഖോർദാദ്",
                "ടിർ",
                "മോർദാദ്",
                "ഷഹ്‌രിവാർ",
                "മെഹർ",
                "അബാൻ",
                "അസർ",
                "ഡെയ്",
                "ബഹ്‌മാൻ",
                "എസ്‌ഫാൻഡ്",
            ],
            &[],
            &[
                "ഫ.", "ഓ.", "ഖോ", "ടി.", "മോ.", "ഷ.", "മെ.", "അ.", "അ.", "ഡെ.", "ബ.", "എ.",
            ],
        )))],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        date: "{era} {year:1} {month} {day}",
        ..DateTemplates::NONE
    }),
    calendar_entry(&[CalendarId("indian")], &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{era} {year:1} {month} {day}",
        ..DateTemplates::NONE
    }),
    calendar_entry(JAPANESE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{era} {year:1} {month} {day}",
        ..DateTemplates::NONE
    }),
    calendar_entry(GENERIC_DATE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{era} {year:1} {month} {day}",
        ..DateTemplates::NONE
    }),
];

/// The other calendars of `my`, CLDR 48 `my.xml`: the Japanese date formats and
/// the generic date formats; the hand-written entry serves gregorian, japanese
/// already.
pub(super) const MY_CLDR: &[CalendarNames] = &[
    calendar_entry(JAPANESE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{era} {day} {month} {year:1}",
        ..DateTemplates::NONE
    }),
    calendar_entry(GENERIC_DATE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{era} {day} {month} {year:1}",
        ..DateTemplates::NONE
    }),
];

/// The other calendars of `ne`, CLDR 48 `ne.xml`: the Indian national months
/// and eras, the Japanese date formats and the generic date formats; the
/// hand-written entry serves gregorian, japanese already.
pub(super) const NE_CLDR: &[CalendarNames] = &[
    calendar_entry(
        &[CalendarId("indian")],
        &[month_cycle(ContextualNames {
            format: widths(
                &[
                    "चैत",
                    "वैशाख",
                    "जेठ",
                    "असार",
                    "साउन",
                    "भदौ",
                    "असोज",
                    "कात्तिक",
                    "मङसिर",
                    "पुस",
                    "माघ",
                    "फागुन",
                ],
                &[
                    "चै",
                    "बै",
                    "जे",
                    "अ",
                    "श्रा",
                    "भा",
                    "अश्वि",
                    "का",
                    "मं",
                    "पौ",
                    "मा",
                    "फा",
                ],
                &[
                    "१", "२", "३", "४", "५", "६", "७", "८", "९", "१०", "११", "१२",
                ],
            ),
            standalone: widths(
                &[
                    "चेत्र",
                    "वैसाख",
                    "जेष्ठ",
                    "आषाढ",
                    "श्रावन",
                    "भाद्र",
                    "आश्विन",
                    "कार्तिक",
                    "मंसिर",
                    "पौष",
                    "माघ",
                    "फाल्गुन",
                ],
                &[
                    "चै",
                    "बै",
                    "जे",
                    "अ",
                    "श्रा",
                    "भा",
                    "अश्वि",
                    "का",
                    "मं",
                    "पौ",
                    "मा",
                    "फा",
                ],
                &[
                    "१", "२", "३", "४", "५", "६", "७", "८", "९", "१०", "११", "१२",
                ],
            ),
        })],
        era_names(&["saka"], &["साक"], &[], &[]),
    )
    .with_templates(DateTemplates {
        date: "{era} {year:1} {month} {day}",
        ..DateTemplates::NONE
    }),
    calendar_entry(JAPANESE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{era} {year:1} {month} {day}",
        ..DateTemplates::NONE
    }),
    calendar_entry(GENERIC_DATE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{era} {year:1} {month} {day}",
        ..DateTemplates::NONE
    }),
];

/// The other calendars of `nl`, CLDR 48 `nl.xml`: the Minguo eras, the Hijri
/// months and eras, the Hebrew months, the Coptic months, the Ethiopic months,
/// the Indian national months, the Japanese date formats and the generic date
/// formats; the hand-written entry serves gregorian, japanese already.
pub(super) const NL_CLDR: &[CalendarNames] = &[
    calendar_entry(
        &[CalendarId("roc")],
        &[],
        era_names(&["broc", "roc"], &["voor R.O.C.", "Minguo"], &[], &[]),
    )
    .with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        ISLAMIC_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "Moeharram",
                "Safar",
                "Rabiʻa al awal",
                "Rabiʻa al thani",
                "Joemadʻal awal",
                "Joemadʻal thani",
                "Rajab",
                "Sjaʻaban",
                "Ramadan",
                "Sjawal",
                "Doe al kaʻaba",
                "Doe al hizja",
            ],
            &[
                "Moeh.",
                "Saf.",
                "Rab. I",
                "Rab. II",
                "Joem. I",
                "Joem. II",
                "Raj.",
                "Sja.",
                "Ram.",
                "Sjaw.",
                "Doe al k.",
                "Doe al h.",
            ],
            &[],
        )))],
        era_names(&["ah"], &["Saʻna Hizjria"], &["AH"], &[]),
    )
    .with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        HEBREW_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "Tisjrie", "Chesjwan", "Kislev", "Tevet", "Sjevat", "Adar", "Nisan", "Ijar",
                "Sivan", "Tammoez", "Av", "Elloel",
            ],
            &[],
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_leap_names(LeapMonthNames {
        intercalary: &[(5, "Adar A")],
        in_leap_years: &[(6, "Adar B")],
        leap_day: None,
    })
    .with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        COPTIC_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "Tut",
                "Babah",
                "Hatur",
                "Kiyahk",
                "Tubah",
                "Amshir",
                "Baramhat",
                "Baramundah",
                "Bashans",
                "Ba’unah",
                "Abib",
                "Misra",
                "Nasi",
            ],
            &[],
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        ETHIOPIC_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "Mäskäräm",
                "Teqemt",
                "Hedar",
                "Tahsas",
                "T’er",
                "Yäkatit",
                "Mägabit",
                "Miyazya",
                "Genbot",
                "Säne",
                "Hamle",
                "Nähase",
                "Pagumän",
            ],
            &[],
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        &[CalendarId("indian")],
        &[month_cycle(ContextualNames::same(widths(
            &[
                "Chaitra",
                "Vaishakha",
                "Jyeshtha",
                "Aashaadha",
                "Shraavana",
                "Bhaadrapada",
                "Ashvina",
                "Kaartika",
                "Agrahayana",
                "Pausha",
                "Maagha",
                "Phaalguna",
            ],
            &[],
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(JAPANESE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(GENERIC_DATE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
];

/// The other calendars of `pl`, CLDR 48 `pl.xml`: the Buddhist era, the Minguo
/// eras, the Hijri months, the Hebrew months, the Persian months, the Indian
/// national months, the Japanese date formats and the generic date formats; the
/// hand-written entry serves gregorian, japanese already.
pub(super) const PL_CLDR: &[CalendarNames] = &[
    calendar_entry(
        BUDDHIST_CALENDARS,
        &[],
        era_names(&["be"], &["e.b."], &[], &[]),
    )
    .with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        &[CalendarId("roc")],
        &[],
        era_names(
            &["broc", "roc"],
            &["przed ROC", "ROC"],
            &["Przed ROC", "ROC"],
            &["przed ROC", "ROC"],
        ),
    )
    .with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        ISLAMIC_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "Muharram",
                "Safar",
                "Rabiʻ I",
                "Rabiʻ II",
                "Dżumada I",
                "Dżumada II",
                "Radżab",
                "Szaban",
                "Ramadan",
                "Szawwal",
                "Zu al-kada",
                "Zu al-hidżdża",
            ],
            &[
                "Muh.", "Saf.", "Rab. I", "Rab. II", "Dżu. I", "Dżu. II", "Ra.", "Sza.", "Ram.",
                "Szaw.", "Zu al-k.", "Zu al-h.",
            ],
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        HEBREW_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "Tiszri", "Cheszwan", "Kislew", "Tewet", "Szwat", "Adar", "Nisan", "Ijar", "Siwan",
                "Tamuz", "Aw", "Elul",
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
    })
    .with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        PERSIAN_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "Farwardin",
                "Ordibeheszt",
                "Chordād",
                "Tir",
                "Mordād",
                "Szahriwar",
                "Mehr",
                "Ābān",
                "Āsar",
                "Déi",
                "Bahman",
                "Esfand",
            ],
            &[],
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        &[CalendarId("indian")],
        &[month_cycle(ContextualNames::same(widths(
            &[
                "Ćajtra",
                "Wajśakha",
                "Dźjesztha",
                "Aszadha",
                "Śrawana",
                "Bhadrapada",
                "Aświna",
                "Karttika",
                "Margaśirsza-Agrahayana",
                "Pausza",
                "Magha",
                "Phalguna",
            ],
            &[],
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(JAPANESE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(GENERIC_DATE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
];

/// The other calendars of `ps`, CLDR 48 `ps.xml`: the Hijri months, the Indian
/// national months and eras, the Persian date formats, the Japanese date
/// formats and the generic date formats; the hand-written entry serves
/// gregorian, japanese, persian already.
pub(super) const PS_CLDR: &[CalendarNames] = &[
    calendar_entry(
        ISLAMIC_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "محرم",
                "صفر",
                "ربیع اول",
                "ربيع II",
                "جمادي اول",
                "جماعه II",
                "رجب",
                "شعبان",
                "رمضان",
                "شوال",
                "ذي القعده",
                "ذي الحج",
            ],
            &[
                "محرم",
                "صفر",
                "ربيع",
                "ربيع II",
                "جماد I",
                "جماد ۲",
                "رجب",
                "شعبان",
                "رمضان",
                "شوال",
                "دالقاعده",
                "ذي الحج",
            ],
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        date: "{era} {year:1} {month} {day}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        &[CalendarId("indian")],
        &[month_cycle(ContextualNames::same(widths(
            &[
                "چيترا",
                "ويساکا",
                "جياستا",
                "اسادها",
                "سراوانا",
                "بهادرا",
                "اسوينا",
                "کارتيکا",
                "اگراهايانا",
                "پاوسا",
                "مگها",
                "پهالگونا",
            ],
            &[],
            &[],
        )))],
        era_names(&["saka"], &["ساکا"], &[], &[]),
    )
    .with_templates(DateTemplates {
        date: "{era} {year:1} {month} {day}",
        ..DateTemplates::NONE
    }),
    calendar_entry(PERSIAN_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{era} {year:1} {month} {day}",
        ..DateTemplates::NONE
    }),
    calendar_entry(JAPANESE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{era} {year:1} {month} {day}",
        ..DateTemplates::NONE
    }),
    calendar_entry(GENERIC_DATE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{era} {year:1} {month} {day}",
        ..DateTemplates::NONE
    }),
];

/// The other calendars of `pt`, CLDR 48 `pt.xml`: the Buddhist era, the Minguo
/// eras, the Japanese date formats and the generic date formats; the
/// hand-written entry serves gregorian, japanese already.
pub(super) const PT_CLDR: &[CalendarNames] = &[
    calendar_entry(
        BUDDHIST_CALENDARS,
        &[],
        era_names(&["be"], &["EB"], &["BE"], &["EB"]),
    )
    .with_templates(DateTemplates {
        date: "{day} de {month} de {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        &[CalendarId("roc")],
        &[],
        era_names(&["broc", "roc"], &["Antes da R.C.", "Minguo"], &[], &[]),
    )
    .with_templates(DateTemplates {
        date: "{day} de {month} de {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(JAPANESE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} de {month} de {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(GENERIC_DATE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} de {month} de {year:1} {era}",
        ..DateTemplates::NONE
    }),
];

/// The other calendars of `ru`, CLDR 48 `ru.xml`: the Buddhist era, the Hijri
/// months and eras, the Hebrew months and eras, the Coptic months and eras, the
/// Ethiopic months, the Persian months, the Persian eras, the Indian national
/// months and eras, the Minguo date formats, the Japanese date formats and the
/// generic date formats; the hand-written entry serves gregorian, japanese, roc
/// already.
pub(super) const RU_CLDR: &[CalendarNames] = &[
    calendar_entry(
        BUDDHIST_CALENDARS,
        &[],
        era_names(&["be"], &["буддийская эра"], &["BE"], &["бэ"]),
    )
    .with_templates(DateTemplates {
        date: "{day} {month} {year:1} г. {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        ISLAMIC_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "мухаррам",
                "сафар",
                "раби-уль-авваль",
                "раби-уль-ахир",
                "джумад-уль-авваль",
                "джумад-уль-ахир",
                "раджаб",
                "шаабан",
                "рамадан",
                "шавваль",
                "зуль-каада",
                "зуль-хиджжа",
            ],
            &[
                "мух.",
                "саф.",
                "раб. I",
                "раб. II",
                "джум. I",
                "джум. II",
                "радж.",
                "шааб.",
                "рам.",
                "шав.",
                "зуль-к.",
                "зуль-х.",
            ],
            &[],
        )))],
        era_names(&["ah"], &["после хиджры"], &["AH"], &[]),
    )
    .with_templates(DateTemplates {
        date: "{day} {month} {year:1} г. {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        HEBREW_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "тишрей",
                "хешван",
                "кислев",
                "тевет",
                "шеват",
                "адар",
                "нисан",
                "ияр",
                "сиван",
                "таммуз",
                "ав",
                "элул",
            ],
            &[],
            &[],
        )))],
        era_names(&["am"], &["от сотворения мира"], &["AM"], &[]),
    )
    .with_leap_names(LeapMonthNames {
        intercalary: &[(5, "адар I")],
        in_leap_years: &[(6, "адар II")],
        leap_day: None,
    })
    .with_templates(DateTemplates {
        date: "{day} {month} {year:1} г. {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        COPTIC_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "тот",
                "бабэ",
                "хатур",
                "кихак",
                "тубэ",
                "амшир",
                "барамхат",
                "бармуда",
                "башнас",
                "бауна",
                "абиб",
                "мисра",
                "наси",
            ],
            &[],
            &[],
        )))],
        era_names(&["am"], &["от Диоклетиана"], &["от Диокл."], &[]),
    )
    .with_templates(DateTemplates {
        date: "{day} {month} {year:1} г. {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        ETHIOPIC_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "мескерем",
                "текемт",
                "хедар",
                "тахсас",
                "тер",
                "якатит",
                "магабит",
                "миазия",
                "генбот",
                "сэнэ",
                "хамлэ",
                "нахасэ",
                "эпагомен",
            ],
            &[],
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        date: "{day} {month} {year:1} г. {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        PERSIAN_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "фарвардин",
                "ордибехешт",
                "хордад",
                "тир",
                "мордад",
                "шахривер",
                "мехр",
                "абан",
                "азер",
                "дей",
                "бахман",
                "эсфанд",
            ],
            &[],
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        date: "{day} {month} {year:1} г. {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        SOLAR_HIJRI_CALENDARS,
        &[],
        era_names(&["ap"], &["персидский год"], &["перс. год"], &[]),
    )
    .with_templates(DateTemplates {
        date: "{day} {month} {year:1} г. {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        &[CalendarId("indian")],
        &[month_cycle(ContextualNames::same(widths(
            &[
                "чайтра",
                "ваисакха",
                "джанштха",
                "асадха",
                "сравана",
                "бхадра",
                "азвина",
                "картика",
                "аграхайана",
                "пауза",
                "магха",
                "пхалгуна",
            ],
            &[],
            &[],
        )))],
        era_names(&["saka"], &["Сака"], &[], &[]),
    )
    .with_templates(DateTemplates {
        date: "{day} {month} {year:1} г. {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(&[CalendarId("roc")], &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month} {year:1} г. {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(JAPANESE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month} {year:1} г. {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(GENERIC_DATE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month} {year:1} г. {era}",
        ..DateTemplates::NONE
    }),
];

/// The other calendars of `sa`, CLDR 48 `sa.xml`: the Japanese date formats and
/// the generic date formats; the hand-written entry serves gregorian, japanese
/// already.
pub(super) const SA_CLDR: &[CalendarNames] = &[
    calendar_entry(JAPANESE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        year: "{era} {year}",
        date: "{era} {day} {month} {year:1}",
        ..DateTemplates::NONE
    }),
    calendar_entry(GENERIC_DATE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        year: "{era} {year}",
        date: "{era} {day} {month} {year:1}",
        ..DateTemplates::NONE
    }),
];

/// The other calendars of `syr`, CLDR 48 `syr.xml`: the Hijri months, the
/// Japanese date formats and the generic date formats; the hand-written entry
/// serves gregorian, japanese already.
pub(super) const SYR_CLDR: &[CalendarNames] = &[
    calendar_entry(
        ISLAMIC_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "ܡܘܚܪܡ",
                "ܨܦܪ",
                "ܪܒܝܥ ܩܕܡܝܐ",
                "ܪܒܝܥ ܬܪܝܢܐ",
                "ܓܘܡܕܐ ܩܕܡܝܐ",
                "ܓܘܡܕܐ ܬܪܝܢܐ",
                "ܪܓܒ",
                "ܫܥܒܐܢ",
                "ܪܡܨܐܢ",
                "ܫܘܐܠ",
                "ܕܘܠܩܥܕܗ",
                "ܕܘܠܚܓܗ",
            ],
            &[],
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        date: "{day} ܒ{month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(JAPANESE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} ܒ{month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(GENERIC_DATE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} ܒ{month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
];

/// The other calendars of `ta`, CLDR 48 `ta.xml`: the Minguo eras, the Hijri
/// months, the Hebrew months, the Coptic months, the Ethiopic months, the
/// Persian months, the Indian national date formats, the Japanese date formats
/// and the generic date formats; the hand-written entry serves gregorian,
/// indian, japanese already.
pub(super) const TA_CLDR: &[CalendarNames] = &[
    calendar_entry(
        &[CalendarId("roc")],
        &[],
        era_names(
            &["broc", "roc"],
            &["R.O.C. -க்கு முன்பு", "ROC"],
            &["ROCக்கு முன்", "ROC"],
            &[],
        ),
    )
    .with_templates(DateTemplates {
        year: "{year} {era}",
        date: "{day} {month}, {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        ISLAMIC_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "முஹர்ரம்",
                "சஃபர்",
                "ரபி 1",
                "ரபி 2",
                "ஜுமதா 1",
                "ஜுமதா 2",
                "ரஜப்",
                "ஷஃபான்",
                "ரமலான்",
                "ஷவ்வால்",
                "துல் கஃதா",
                "துல் ஹிஜ்ஜா",
            ],
            &[
                "முஹ.",
                "சஃப.",
                "ரபி 1",
                "ரபி 2",
                "ஜும. 1",
                "ஜும. 2",
                "ரஜ.",
                "ஷஃ.",
                "ரம.",
                "ஷவ்.",
                "துல் கஃ.",
                "துல் ஹிஜ்.",
            ],
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        year: "{year} {era}",
        date: "{day} {month}, {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        HEBREW_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "டிஷ்ரி",
                "ஹெஷ்வான்",
                "கிஸ்லெவ்",
                "டெவெட்",
                "ஷெவாட்",
                "அடார்",
                "நிசான்",
                "ஐயார்",
                "சிவான்",
                "தமுஸ்",
                "அவ்",
                "எலுல்",
            ],
            &[
                "டிஷ்.",
                "ஹெஷ்.",
                "கிஸ்.",
                "டெவெ.",
                "ஷெவா.",
                "அடா.",
                "நிசா.",
                "ஐயா.",
                "சிவா.",
                "தமு.",
                "அவ்",
                "எலு.",
            ],
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_leap_names(LeapMonthNames {
        intercalary: &[(5, "அடார் 1")],
        in_leap_years: &[(6, "அடார் 2")],
        leap_day: None,
    })
    .with_templates(DateTemplates {
        year: "{year} {era}",
        date: "{day} {month}, {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        COPTIC_CALENDARS,
        &[month_cycle(ContextualNames {
            format: widths(
                &[
                    "டட்",
                    "பாபா",
                    "ஹடுர்",
                    "கியாக்",
                    "டுபா",
                    "அம்ஷீர்",
                    "பரம்ஹாட்",
                    "பரமுதா",
                    "பாஷன்ஸ்",
                    "பவுனா",
                    "அபீப்",
                    "மஸ்ரா",
                    "நசி",
                ],
                &[
                    "டட்",
                    "பாபா",
                    "ஹடு.",
                    "கியா.",
                    "டுபா",
                    "அம்.",
                    "பரம்.",
                    "பரமு.",
                    "பாஷ.",
                    "பவு.",
                    "அபீ.",
                    "மஸ்.",
                    "நசி",
                ],
                &[],
            ),
            standalone: widths(
                &[],
                &[
                    "டட்",
                    "பாபா",
                    "ஹடு.",
                    "கியா.",
                    "டுபா",
                    "அம்.",
                    "பரம்.",
                    "பரமு.",
                    "பாஷ.",
                    "பவு.",
                    "அபீ.",
                    "மஸ்ரா",
                    "நசி",
                ],
                &[],
            ),
        })],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        year: "{year} {era}",
        date: "{day} {month}, {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        ETHIOPIC_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "மஸ்கரம்",
                "தெகெம்ப்த்",
                "ஹெதர்",
                "தஹ்சாஸ்",
                "தெர்",
                "யாகாடிட்",
                "மகாபிட்",
                "மியாஸ்யா",
                "கென்போ",
                "சனே",
                "ஹமேல்",
                "நஹாசே",
                "பாகுமே",
            ],
            &[
                "மஸ்.",
                "தெகெ.",
                "ஹெத.",
                "தஹ்.",
                "தெர்",
                "யாகா.",
                "மகா.",
                "மியா.",
                "கென்.",
                "சனே",
                "ஹமே.",
                "நஹா.",
                "பாகு.",
            ],
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        year: "{year} {era}",
        date: "{day} {month}, {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        PERSIAN_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "ஃபர்வாதின்",
                "ஆர்டிபெஹெஷ்த்",
                "கொர்தாத்",
                "திர்",
                "மொர்தாத்",
                "ஷாரிவார்",
                "மெஹ்ர்",
                "அபான்",
                "அசார்",
                "தே",
                "பஹ்மான்",
                "எஃபான்",
            ],
            &[
                "ஃபர்.",
                "ஆர்டி.",
                "கொர்.",
                "திர்",
                "மொர்.",
                "ஷாரி.",
                "மெஹ்.",
                "அபா.",
                "அசா.",
                "தே",
                "பஹ்.",
                "எஃ.",
            ],
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        year: "{year} {era}",
        date: "{day} {month}, {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(&[CalendarId("indian")], &[], EraNames::EMPTY).with_templates(DateTemplates {
        year: "{year} {era}",
        date: "{day} {month}, {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(JAPANESE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        year: "{year} {era}",
        date: "{day} {month}, {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(GENERIC_DATE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        year: "{year} {era}",
        date: "{day} {month}, {year:1} {era}",
        ..DateTemplates::NONE
    }),
];

/// The other calendars of `th`, CLDR 48 `th.xml`: the Hijri months and eras,
/// the Hebrew months and eras, the Coptic months, the Ethiopic months, the
/// Persian months, the Persian eras, the Indian national months and eras, the
/// Buddhist date formats, the Minguo date formats and the Japanese date
/// formats; the hand-written entry serves buddhist, gregorian, japanese, roc
/// already.
pub(super) const TH_CLDR: &[CalendarNames] = &[
    calendar_entry(
        ISLAMIC_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "มุฮะร์รอม",
                "ซอฟาร์",
                "รอบี I",
                "รอบี II",
                "จุมาดา I",
                "จุมาดา II",
                "รอจับ",
                "ชะอะบาน",
                "รอมะดอน",
                "เชาวัล",
                "ซุลกิอฺดะฮฺ",
                "ซุลหิจญะฮฺ",
            ],
            &[
                "มุฮัร.",
                "เศาะ.",
                "รอบี I",
                "รอบี II",
                "จุมาดา I",
                "จุมาดา II",
                "เราะ.",
                "ชะอ์.",
                "เราะมะ.",
                "เชาว.",
                "ซุลกิอฺ.",
                "ซุลหิจ.",
            ],
            &[],
        )))],
        era_names(&["ah"], &["ฮิจเราะห์ศักราช"], &["ฮ.ศ."], &[]),
    ),
    calendar_entry(
        HEBREW_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "ทิชรี",
                "เฮวาน",
                "กีสเลฟ",
                "เตเวต",
                "เชวัต",
                "อาดาร์",
                "นิสซาน",
                "อิยาร์",
                "สีวัน",
                "ตามูซ",
                "อัฟ",
                "เอลอุล",
            ],
            &[],
            &[],
        )))],
        era_names(&["am"], &["ย.ศ."], &[], &[]),
    )
    .with_leap_names(LeapMonthNames {
        intercalary: &[(5, "อาดาร์ I")],
        in_leap_years: &[(6, "อาดาร์ II")],
        leap_day: None,
    }),
    calendar_entry(
        COPTIC_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "เทาท์",
                "บาบา",
                "ฮาเทอร์",
                "เคียฟ",
                "โทบา",
                "อัมเชอร์",
                "บารัมฮัท",
                "บาราเมาดา",
                "บาชันส์",
                "พาโอนา",
                "อีเปป",
                "เมสรา",
                "นาซี",
            ],
            &[],
            &[],
        )))],
        EraNames::EMPTY,
    ),
    calendar_entry(
        ETHIOPIC_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "เมสเคอเรม",
                "เตเกมท",
                "เฮดาร์",
                "ทาฮ์ซัส",
                "เทอร์",
                "เยคาทิท",
                "เมกาบิต",
                "เมียเซีย",
                "เจนบอต",
                "เซเน",
                "ฮัมเล",
                "เนแฮซ",
                "พากูเมน",
            ],
            &[],
            &[],
        )))],
        EraNames::EMPTY,
    ),
    calendar_entry(
        PERSIAN_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "ฟาร์วาร์ดิน",
                "ออร์ดิเบเฮชต์",
                "คอร์แดด",
                "เตอร์",
                "มอร์แดด",
                "ชาหริวาร์",
                "เมฮร์",
                "อะบาน",
                "อะซาร์",
                "เดย์",
                "บาฮ์มาน",
                "เอสฟานด์",
            ],
            &[],
            &[],
        )))],
        EraNames::EMPTY,
    ),
    calendar_entry(
        SOLAR_HIJRI_CALENDARS,
        &[],
        era_names(&["ap"], &["ปีเปอร์เซีย"], &[], &[]),
    ),
    calendar_entry(
        &[CalendarId("indian")],
        &[month_cycle(ContextualNames::same(widths(
            &[
                "จิตรา",
                "วิสาขา",
                "เชษฐา",
                "อัษฎา",
                "ศรวณา",
                "พัตรา",
                "อัศวิชา",
                "การติกา",
                "มฤคศิรา",
                "ปุษยา",
                "มาฆะ",
                "ผลคุณี",
            ],
            &[],
            &[],
        )))],
        era_names(&["saka"], &["ม.ศ."], &[], &[]),
    ),
    calendar_entry(BUDDHIST_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month} {year:1}",
        ..DateTemplates::NONE
    }),
    calendar_entry(&[CalendarId("roc")], &[], EraNames::EMPTY).with_templates(DateTemplates {
        year: "ปี{era}ที่ {year}",
        date: "{day} {month} ปี{era} {year:1}",
        ..DateTemplates::NONE
    }),
    calendar_entry(JAPANESE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month} ปี{era} {year:1}",
        ..DateTemplates::NONE
    }),
];

/// The other calendars of `tr`, CLDR 48 `tr.xml`: the Minguo eras, the Hijri
/// months and eras, the Hebrew months, the Coptic months, the Ethiopic months,
/// the Persian months, the Japanese date formats and the generic date formats;
/// the hand-written entry serves gregorian, japanese already.
pub(super) const TR_CLDR: &[CalendarNames] = &[
    calendar_entry(
        &[CalendarId("roc")],
        &[],
        era_names(
            &["broc", "roc"],
            &["R.O.C. Öncesi", "Minguo"],
            &["BROC", "Minguo"],
            &[],
        ),
    )
    .with_templates(DateTemplates {
        date: "{era} {day} {month} {year:1}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        ISLAMIC_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "Muharrem",
                "Safer",
                "Rebiülevvel",
                "Rebiülahir",
                "Cemaziyelevvel",
                "Cemaziyelahir",
                "Recep",
                "Şaban",
                "Ramazan",
                "Şevval",
                "Zilkade",
                "Zilhicce",
            ],
            &[
                "Muhar.", "Safer", "R.evvel", "R.ahir", "C.evvel", "C.ahir", "Recep", "Şaban",
                "Ram.", "Şevval", "Zilkade", "Zilhicce",
            ],
            &[],
        )))],
        era_names(&["ah"], &["Hicri"], &[], &[]),
    )
    .with_templates(DateTemplates {
        date: "{era} {day} {month} {year:1}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        HEBREW_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "Tişri", "Heşvan", "Kislev", "Tevet", "Şevat", "Adar", "Nisan", "İyar", "Sivan",
                "Tamuz", "Av", "Elul",
            ],
            &[],
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_leap_names(LeapMonthNames {
        intercalary: &[(5, "Adar Rişon")],
        in_leap_years: &[(6, "Veadar")],
        leap_day: None,
    })
    .with_templates(DateTemplates {
        date: "{era} {day} {month} {year:1}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        COPTIC_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "Tût", "Bâbe", "Hatur", "Keyhek", "Tûbe", "Imşir", "Bermuhat", "Bermude",
                "Peyştes", "Bune", "Ebip", "Mısrî", "Nesî",
            ],
            &[],
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        date: "{era} {day} {month} {year:1}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        ETHIOPIC_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "Meskerem",
                "Tikimt",
                "Hidar",
                "Tahsas",
                "Tir",
                "Yakatit",
                "Magabit",
                "Miyazya",
                "Ginbot",
                "Sene",
                "Hamle",
                "Nehasa",
                "Pagumiene",
            ],
            &[],
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        date: "{era} {day} {month} {year:1}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        PERSIAN_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "Ferverdin",
                "Ordibeheşt",
                "Hordad",
                "Tir",
                "Mordad",
                "Şehriver",
                "Mehr",
                "Aban",
                "Azer",
                "Dey",
                "Behmen",
                "Esfend",
            ],
            &[],
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        date: "{era} {day} {month} {year:1}",
        ..DateTemplates::NONE
    }),
    calendar_entry(JAPANESE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(GENERIC_DATE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{era} {day} {month} {year:1}",
        ..DateTemplates::NONE
    }),
];

/// The other calendars of `zgh`, CLDR 48 `zgh.xml`: the Japanese date formats
/// and the generic date formats; the hand-written entry serves gregorian,
/// japanese already.
pub(super) const ZGH_CLDR: &[CalendarNames] = &[
    calendar_entry(JAPANESE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(GENERIC_DATE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
];

/// The other calendars of `zh-Hans`, CLDR 48 `zh.xml`: the Buddhist era, the
/// Hijri months and eras, the Hebrew months and eras, the Coptic months and
/// eras, the Ethiopic months, the Persian months, the Persian eras and the
/// Indian national months and eras; the hand-written entry serves chinese,
/// dangi, gregorian, japanese, roc already.
pub(super) const ZH_HANS_CLDR: &[CalendarNames] = &[
    calendar_entry(
        BUDDHIST_CALENDARS,
        &[],
        era_names(&["be"], &["佛历"], &[], &[]),
    )
    .with_templates(DateTemplates {
        date: "{era}{year:1}年{month:1}月{day}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        ISLAMIC_CALENDARS,
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
            &[],
        )))],
        era_names(&["ah"], &["伊斯兰历"], &[], &[]),
    )
    .with_templates(DateTemplates {
        date: "{era}{year:1}年{month:1}月{day}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        HEBREW_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "一月",
                "二月",
                "三月",
                "四月",
                "五月",
                "七月",
                "八月",
                "九月",
                "十月",
                "十一月",
                "十二月",
                "十三月",
            ],
            &[
                "1月", "2月", "3月", "4月", "5月", "7月", "8月", "9月", "10月", "11月", "12月",
                "13月",
            ],
            &[],
        )))],
        era_names(&["am"], &["希伯来历"], &[], &[]),
    )
    .with_leap_names(LeapMonthNames {
        intercalary: &[(5, "六月")],
        in_leap_years: &[(6, "闰七月")],
        leap_day: None,
    })
    .with_templates(DateTemplates {
        date: "{era}{year:1}年{month}{day}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        COPTIC_CALENDARS,
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
                "十三月",
            ],
            &[
                "1月", "2月", "3月", "4月", "5月", "6月", "7月", "8月", "9月", "10月", "11月",
                "12月", "13月",
            ],
            &[],
        )))],
        era_names(&["am"], &["科普特历"], &[], &[]),
    )
    .with_templates(DateTemplates {
        date: "{era}{year:1}年{month:1}月{day}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        ETHIOPIC_CALENDARS,
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
                "十三月",
            ],
            &[
                "1月", "2月", "3月", "4月", "5月", "6月", "7月", "8月", "9月", "10月", "11月",
                "12月", "13月",
            ],
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        date: "{era}{year:1}年{month:1}月{day}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        PERSIAN_CALENDARS,
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
            &[],
        )))],
        EraNames::EMPTY,
    )
    .with_templates(DateTemplates {
        date: "{era}{year:1}年{month:1}月{day}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        SOLAR_HIJRI_CALENDARS,
        &[],
        era_names(&["ap"], &["波斯历"], &[], &[]),
    )
    .with_templates(DateTemplates {
        date: "{era}{year:1}年{month:1}月{day}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        &[CalendarId("indian")],
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
            &[],
        )))],
        era_names(&["saka"], &["印度历"], &[], &[]),
    )
    .with_templates(DateTemplates {
        date: "{era}{year:1}年{month:2}月{day}",
        ..DateTemplates::NONE
    }),
];

/// The other calendars of `zh-Hant`, CLDR 48 `zh_Hant.xml`: the Buddhist era,
/// the Hijri months and eras, the Hebrew months and eras, the Persian eras and
/// the Indian national months and eras; the hand-written entry serves chinese,
/// dangi, gregorian, japanese, roc already.
pub(super) const ZH_HANT_CLDR: &[CalendarNames] = &[
    calendar_entry(
        BUDDHIST_CALENDARS,
        &[],
        era_names(&["be"], &["佛曆"], &[], &[]),
    )
    .with_templates(DateTemplates {
        date: "{era}{year:1}年{month:1}月{day}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        ISLAMIC_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "穆哈蘭姆月",
                "色法爾月",
                "賴比月 I",
                "賴比月 II",
                "主馬達月 I",
                "主馬達月 II",
                "賴哲卜月",
                "舍爾邦月",
                "賴買丹月",
                "閃瓦魯月",
                "都爾喀爾德月",
                "都爾黑哲月",
            ],
            &[],
            &[],
        )))],
        era_names(&["ah"], &["伊斯蘭曆"], &[], &[]),
    )
    .with_templates(DateTemplates {
        date: "{era}{year:1}年{month:1}月{day}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        HEBREW_CALENDARS,
        &[month_cycle(ContextualNames::same(widths(
            &[
                "提斯利月",
                "瑪西班月",
                "基斯流月",
                "提別月",
                "細罷特月",
                "亞達月",
                "尼散月",
                "以珥月",
                "西彎月",
                "搭模斯月",
                "埃波月",
                "以祿月",
            ],
            &[],
            &[],
        )))],
        era_names(&["am"], &["創世紀元"], &[], &[]),
    )
    .with_leap_names(LeapMonthNames {
        intercalary: &[(5, "亞達月 I")],
        in_leap_years: &[(6, "亞達月 II")],
        leap_day: None,
    })
    .with_templates(DateTemplates {
        year: "{era} {year}年",
        date: "{era}{year:1}年{month}{day}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        SOLAR_HIJRI_CALENDARS,
        &[],
        era_names(&["ap"], &["波斯曆"], &[], &[]),
    )
    .with_templates(DateTemplates {
        year: "{era} {year}年",
        date: "{era} {year:1}年{month:1}月{day}",
        ..DateTemplates::NONE
    }),
    calendar_entry(
        &[CalendarId("indian")],
        &[month_cycle(ContextualNames::same(widths(
            &[
                "制檀邏月",
                "吠舍佉月",
                "逝瑟吒月",
                "頞沙荼月",
                "室羅伐拏月",
                "婆羅鉢陀月",
                "頞涇縛庚闍月",
                "迦剌底迦月",
                "末伽始羅月",
                "報沙月",
                "磨祛月",
                "頗勒窶拏月",
            ],
            &[],
            &[],
        )))],
        era_names(&["saka"], &["印度曆"], &[], &[]),
    )
    .with_templates(DateTemplates {
        year: "{era} {year}年",
        date: "{era} {year:1}年{month:1}月{day}",
        ..DateTemplates::NONE
    }),
];

/// The other calendars of `fil`, CLDR 48 `fil.xml`: the Minguo date formats,
/// the Japanese date formats and the generic date formats; the hand-written
/// entry serves gregorian, japanese, roc already.
pub(super) const FIL_CLDR: &[CalendarNames] = &[
    calendar_entry(&[CalendarId("roc")], &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{month} {day}, {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(JAPANESE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{month} {day}, {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(GENERIC_DATE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{month} {day}, {year:1} {era}",
        ..DateTemplates::NONE
    }),
];

/// The other calendars of `ha`, CLDR 48 `ha.xml`: the Hijri date formats, the
/// Japanese date formats and the generic date formats; the hand-written entry
/// serves gregorian, islamic, japanese already.
pub(super) const HA_CLDR: &[CalendarNames] = &[
    calendar_entry(ISLAMIC_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        year: "{year} {era}",
        date: "{day} {month}, {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(JAPANESE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month}, {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(GENERIC_DATE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month}, {year:1} {era}",
        ..DateTemplates::NONE
    }),
];

/// The other calendars of `mr`, CLDR 48 `mr.xml`: the Buddhist date formats,
/// the Minguo date formats, the Hijri date formats, the Hebrew date formats,
/// the Coptic date formats, the Ethiopic date formats, the Persian date
/// formats, the Indian national date formats, the Japanese date formats and the
/// generic date formats; the hand-written entry serves buddhist, coptic,
/// ethiopic, gregorian, hebrew, indian, islamic, japanese, persian, roc
/// already.
pub(super) const MR_CLDR: &[CalendarNames] = &[
    calendar_entry(BUDDHIST_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        year: "{year} {era}",
        date: "{day} {month}, {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(&[CalendarId("roc")], &[], EraNames::EMPTY).with_templates(DateTemplates {
        year: "{year} {era}",
        date: "{day} {month}, {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(ISLAMIC_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        year: "{year} {era}",
        date: "{day} {month}, {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(HEBREW_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        year: "{year} {era}",
        date: "{day} {month}, {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(COPTIC_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        year: "{year} {era}",
        date: "{day} {month}, {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(ETHIOPIC_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        year: "{year} {era}",
        date: "{day} {month}, {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(PERSIAN_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        year: "{year} {era}",
        date: "{day} {month}, {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(&[CalendarId("indian")], &[], EraNames::EMPTY).with_templates(DateTemplates {
        year: "{year} {era}",
        date: "{day} {month}, {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(JAPANESE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        year: "{year} {era}",
        date: "{day} {month}, {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(GENERIC_DATE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        year: "{year} {era}",
        date: "{day} {month}, {year:1} {era}",
        ..DateTemplates::NONE
    }),
];

/// The other calendars of `pa-Arab`, CLDR 48 `pa_Arab.xml`: the Japanese date
/// formats and the generic date formats; the hand-written entry serves
/// gregorian, japanese already.
pub(super) const PA_ARAB_CLDR: &[CalendarNames] = &[
    calendar_entry(JAPANESE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(GENERIC_DATE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
];

/// The other calendars of `pa-Guru`, CLDR 48 `pa.xml`: the Buddhist date
/// formats, the Minguo date formats, the Hijri date formats, the Hebrew date
/// formats, the Coptic date formats, the Ethiopic date formats, the Persian
/// date formats, the Indian national date formats, the Japanese date formats
/// and the generic date formats; the hand-written entry serves buddhist,
/// coptic, ethiopic, gregorian, hebrew, indian, islamic, japanese, persian, roc
/// already.
pub(super) const PA_GURU_CLDR: &[CalendarNames] = &[
    calendar_entry(BUDDHIST_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(&[CalendarId("roc")], &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(ISLAMIC_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(HEBREW_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(COPTIC_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(ETHIOPIC_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(PERSIAN_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(&[CalendarId("indian")], &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(JAPANESE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(GENERIC_DATE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
];

/// The other calendars of `pcm`, CLDR 48 `pcm.xml`: the Japanese date formats
/// and the generic date formats; the hand-written entry serves gregorian,
/// japanese already.
pub(super) const PCM_CLDR: &[CalendarNames] = &[
    calendar_entry(JAPANESE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(GENERIC_DATE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
];

/// The other calendars of `pt-PT`, CLDR 48 `pt_PT.xml`, `pt.xml`: the Buddhist
/// date formats, the Japanese date formats and the generic date formats; the
/// hand-written entry serves buddhist, gregorian, japanese already.
pub(super) const PT_PT_CLDR: &[CalendarNames] = &[
    calendar_entry(BUDDHIST_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} de {month} de {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(JAPANESE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} de {month} de {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(GENERIC_DATE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} de {month} de {year:1} {era}",
        ..DateTemplates::NONE
    }),
];

/// The other calendars of `sw`, CLDR 48 `sw.xml`: the Japanese date formats and
/// the generic date formats; the hand-written entry serves gregorian, japanese
/// already.
pub(super) const SW_CLDR: &[CalendarNames] = &[
    calendar_entry(JAPANESE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(GENERIC_DATE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
];

/// The other calendars of `te`, CLDR 48 `te.xml`: the Minguo date formats, the
/// Hebrew date formats, the Coptic date formats, the Ethiopic date formats, the
/// Persian date formats, the Indian national date formats, the Japanese date
/// formats and the generic date formats; the hand-written entry serves coptic,
/// ethiopic, gregorian, hebrew, indian, japanese, persian, roc already.
pub(super) const TE_CLDR: &[CalendarNames] = &[
    calendar_entry(&[CalendarId("roc")], &[], EraNames::EMPTY).with_templates(DateTemplates {
        year: "{year} {era}",
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(HEBREW_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        year: "{year} {era}",
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(COPTIC_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        year: "{year} {era}",
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(ETHIOPIC_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        year: "{year} {era}",
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(PERSIAN_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        year: "{year} {era}",
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(&[CalendarId("indian")], &[], EraNames::EMPTY).with_templates(DateTemplates {
        year: "{year} {era}",
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(JAPANESE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        year: "{year} {era}",
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(GENERIC_DATE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        year: "{year} {era}",
        date: "{day} {month} {year:1} {era}",
        ..DateTemplates::NONE
    }),
];

/// The other calendars of `ur`, CLDR 48 `ur.xml`: the Minguo date formats, the
/// Hijri date formats, the Hebrew date formats, the Coptic date formats, the
/// Ethiopic date formats, the Persian date formats, the Indian national date
/// formats, the Japanese date formats and the generic date formats; the
/// hand-written entry serves coptic, ethiopic, gregorian, hebrew, indian,
/// islamic, japanese, persian, roc already.
pub(super) const UR_CLDR: &[CalendarNames] = &[
    calendar_entry(&[CalendarId("roc")], &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month}، {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(ISLAMIC_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month}، {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(HEBREW_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month}، {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(COPTIC_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month}، {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(ETHIOPIC_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month}، {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(PERSIAN_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month}، {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(&[CalendarId("indian")], &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month}، {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(JAPANESE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month}، {year:1} {era}",
        ..DateTemplates::NONE
    }),
    calendar_entry(GENERIC_DATE_CALENDARS, &[], EraNames::EMPTY).with_templates(DateTemplates {
        date: "{day} {month}، {year:1} {era}",
        ..DateTemplates::NONE
    }),
];
