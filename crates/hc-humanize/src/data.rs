//! The relative-time data itself.
//!
//! Everything in this module is `&'static` data and nothing in it is logic.
//! One language is one [`LocaleData`] value plus one line in [`LOCALES`];
//! the lookup in [`crate::lookup`] does not know which languages exist and
//! gains no branch when a new one arrives.
//!
//! # Provenance
//!
//! The relative-time phrases follow the Unicode CLDR common locale data —
//! the `main/<locale>.xml` `<fields>` section, which is where
//! `<relative>` and `<relativeTime>` live — and the undirected unit phrases
//! follow the `<unit type="duration-…">` section of the same file. They are
//! hand-checked, not generated, and they are a subset: CLDR carries roughly
//! 600 locales and this crate carries 21.
//!
//! Three kinds of string here are **not** from CLDR, because CLDR has no
//! field for them, and are ordinary translations kept in the same table so
//! that adding a language stays a single edit:
//!
//! * the approximation hedges (*just over*, *nearly*) used by
//!   [`crate::approximate`](mod@crate::approximate);
//! * the half-unit idioms (*half an hour*, *anderthalb Stunden*);
//! * the compact suffixes of *2h30m*.
//!
//! # Deliberate deviations
//!
//! * English uses `{0} and {1}` to close a list, not CLDR's `{0}, and {1}`.
//!   The serial comma is a house-style choice, not a linguistic one, and the
//!   comma-free form is what the rest of this workspace writes.
//! * The `zh` entry carries Simplified Chinese, so `zh-Hans` reaches it by
//!   truncation inheritance. `zh-Hant` is a separate entry. CLDR would do
//!   this with likely-subtags; this crate does not model those, so `zh-TW`
//!   and `zh-HK` resolve to Simplified and a caller who cares must spell
//!   `zh-Hant` out.
//! * Weekday phrases avoid agreement wherever a language inflects the
//!   demonstrative for gender. Russian says *понедельник на прошлой неделе*
//!   rather than *в прошлый понедельник* because the latter is wrong for
//!   *среда*, and Portuguese and Italian do the same.

use crate::pattern::{
    ApproximatePatterns, ListForms, ListPatterns, LocaleData, PluralForms, StyleData, UnitPatterns,
    UnitStrings, WeekdayPatterns,
};

// --- construction helpers -------------------------------------------------
//
// These exist only to keep a locale entry readable; they add no behaviour.

/// A language with no numeral agreement at all: one pattern, category
/// `other`.
const fn p1(other: &'static str) -> PluralForms {
    PluralForms {
        zero: "",
        one: "",
        two: "",
        few: "",
        many: "",
        other,
    }
}

/// The `one`/`other` shape: English, German, Dutch, the Romance languages.
const fn p2(one: &'static str, other: &'static str) -> PluralForms {
    PluralForms {
        zero: "",
        one,
        two: "",
        few: "",
        many: "",
        other,
    }
}

/// The Slavic shape: `one`/`few`/`many`, with `other` carrying the decimals.
const fn p4(
    one: &'static str,
    few: &'static str,
    many: &'static str,
    other: &'static str,
) -> PluralForms {
    PluralForms {
        zero: "",
        one,
        two: "",
        few,
        many,
        other,
    }
}

/// All six categories: Arabic and Welsh.
const fn p6(
    zero: &'static str,
    one: &'static str,
    two: &'static str,
    few: &'static str,
    many: &'static str,
    other: &'static str,
) -> PluralForms {
    PluralForms {
        zero,
        one,
        two,
        few,
        many,
        other,
    }
}

/// One unit: the three pattern sets plus the three usual special words.
const fn u(
    past: PluralForms,
    future: PluralForms,
    count: PluralForms,
    previous: &'static str,
    current: &'static str,
    next: &'static str,
) -> UnitPatterns {
    UnitPatterns {
        past,
        future,
        count,
        previous,
        current,
        next,
        previous_2: "",
        next_2: "",
        half: "",
        one_and_a_half: "",
    }
}

/// The day unit, which is the one that usually has words for ±2 as well.
#[allow(clippy::too_many_arguments)]
const fn u_day(
    past: PluralForms,
    future: PluralForms,
    count: PluralForms,
    previous_2: &'static str,
    previous: &'static str,
    current: &'static str,
    next: &'static str,
    next_2: &'static str,
) -> UnitPatterns {
    UnitPatterns {
        past,
        future,
        count,
        previous,
        current,
        next,
        previous_2,
        next_2,
        half: "",
        one_and_a_half: "",
    }
}

/// Attach the half-unit idioms to a unit that has them.
const fn u_half(
    mut patterns: UnitPatterns,
    half: &'static str,
    one_and_a_half: &'static str,
) -> UnitPatterns {
    patterns.half = half;
    patterns.one_and_a_half = one_and_a_half;
    patterns
}

/// A list that joins everything with the same pattern.
const fn list_uniform(pattern: &'static str) -> ListForms {
    ListForms {
        two: pattern,
        start: pattern,
        middle: pattern,
        end: pattern,
    }
}

/// The usual shape: commas in the middle, a conjunction at the end.
const fn list_conjunction(comma: &'static str, conjunction: &'static str) -> ListForms {
    ListForms {
        two: conjunction,
        start: comma,
        middle: comma,
        end: conjunction,
    }
}

/// The list set of a language that writes `a, b` and `a <and> b`.
const fn lists(
    comma: &'static str,
    conjunction: &'static str,
    narrow: &'static str,
) -> ListPatterns {
    ListPatterns {
        standard: list_conjunction(comma, conjunction),
        unit: list_uniform(comma),
        narrow: list_uniform(narrow),
    }
}

/// One plain string per unit, in unit order.
#[allow(clippy::too_many_arguments)]
const fn strings(
    second: &'static str,
    minute: &'static str,
    hour: &'static str,
    day: &'static str,
    week: &'static str,
    month: &'static str,
    quarter: &'static str,
    year: &'static str,
) -> UnitStrings {
    UnitStrings {
        second,
        minute,
        hour,
        day,
        week,
        month,
        quarter,
        year,
    }
}

// --- root -----------------------------------------------------------------

/// The root entry, and the floor of every lookup.
///
/// CLDR's root is deliberately language-free: its relative fields are signed
/// abbreviations such as `-3 d`, not English. This entry keeps that, because
/// an unknown locale that silently answered in English would be a bug that
/// only a speaker of the missing language could notice. Root states no
/// special words at all, so `Numeric::Auto` degrades to the numeric form
/// rather than inventing a word for *yesterday*.
pub static ROOT: LocaleData = LocaleData {
    tag: "und",
    long: StyleData {
        second: u(p1("-{0} s"), p1("+{0} s"), p1("{0} s"), "", "", ""),
        minute: u(p1("-{0} min"), p1("+{0} min"), p1("{0} min"), "", "", ""),
        hour: u(p1("-{0} h"), p1("+{0} h"), p1("{0} h"), "", "", ""),
        day: u(p1("-{0} d"), p1("+{0} d"), p1("{0} d"), "", "", ""),
        week: u(p1("-{0} w"), p1("+{0} w"), p1("{0} w"), "", "", ""),
        month: u(p1("-{0} m"), p1("+{0} m"), p1("{0} m"), "", "", ""),
        quarter: u(p1("-{0} q"), p1("+{0} q"), p1("{0} q"), "", "", ""),
        year: u(p1("-{0} y"), p1("+{0} y"), p1("{0} y"), "", "", ""),
    },
    short: StyleData::EMPTY,
    narrow: StyleData::EMPTY,
    compact: UnitStrings {
        second: "s",
        minute: "min",
        hour: "h",
        day: "d",
        week: "w",
        month: "mo",
        quarter: "q",
        year: "y",
    },
    indefinite: UnitStrings::EMPTY,
    list: lists("{0}, {1}", "{0}, {1}", "{0} {1}"),
    approximate: ApproximatePatterns {
        exactly: "{0}",
        about: "~{0}",
        just_over: ">{0}",
        over: ">{0}",
        nearly: "<{0}",
        less_than: "<{0}",
        more_than: ">{0}",
    },
    weekday: WeekdayPatterns {
        previous: "{0}",
        current: "{0}",
        next: "{0}",
    },
    decimal_separator: ".",
    at_pattern: "{0}, {1}",
};

// --- Arabic ---------------------------------------------------------------
//
// Arabic is the reason the six-category path exists: ٠ takes `zero`, ١
// `one`, ٢ the dual `two`, ٣–١٠ `few`, ١١–٩٩ `many` and ١٠٠ `other`, and
// each of those six is a different sentence. The `one` and `two` patterns
// carry no `{0}` at all, because the noun's own form already says how many
// — يومين *is* "two days".

const AR_LONG: StyleData = StyleData {
    second: u(
        p6(
            "قبل {0} ثانية",
            "قبل ثانية واحدة",
            "قبل ثانيتين",
            "قبل {0} ثوانٍ",
            "قبل {0} ثانية",
            "قبل {0} ثانية",
        ),
        p6(
            "خلال {0} ثانية",
            "خلال ثانية واحدة",
            "خلال ثانيتين",
            "خلال {0} ثوانٍ",
            "خلال {0} ثانية",
            "خلال {0} ثانية",
        ),
        p6(
            "{0} ثانية",
            "ثانية واحدة",
            "ثانيتان",
            "{0} ثوانٍ",
            "{0} ثانية",
            "{0} ثانية",
        ),
        "",
        "الآن",
        "",
    ),
    minute: u(
        p6(
            "قبل {0} دقيقة",
            "قبل دقيقة واحدة",
            "قبل دقيقتين",
            "قبل {0} دقائق",
            "قبل {0} دقيقة",
            "قبل {0} دقيقة",
        ),
        p6(
            "خلال {0} دقيقة",
            "خلال دقيقة واحدة",
            "خلال دقيقتين",
            "خلال {0} دقائق",
            "خلال {0} دقيقة",
            "خلال {0} دقيقة",
        ),
        p6(
            "{0} دقيقة",
            "دقيقة واحدة",
            "دقيقتان",
            "{0} دقائق",
            "{0} دقيقة",
            "{0} دقيقة",
        ),
        "",
        "هذه الدقيقة",
        "",
    ),
    hour: u_half(
        u(
            p6(
                "قبل {0} ساعة",
                "قبل ساعة واحدة",
                "قبل ساعتين",
                "قبل {0} ساعات",
                "قبل {0} ساعة",
                "قبل {0} ساعة",
            ),
            p6(
                "خلال {0} ساعة",
                "خلال ساعة واحدة",
                "خلال ساعتين",
                "خلال {0} ساعات",
                "خلال {0} ساعة",
                "خلال {0} ساعة",
            ),
            p6(
                "{0} ساعة",
                "ساعة واحدة",
                "ساعتان",
                "{0} ساعات",
                "{0} ساعة",
                "{0} ساعة",
            ),
            "",
            "هذه الساعة",
            "",
        ),
        "نصف ساعة",
        "ساعة ونصف",
    ),
    day: u_day(
        p6(
            "قبل {0} يوم",
            "قبل يوم واحد",
            "قبل يومين",
            "قبل {0} أيام",
            "قبل {0} يومًا",
            "قبل {0} يوم",
        ),
        p6(
            "خلال {0} يوم",
            "خلال يوم واحد",
            "خلال يومين",
            "خلال {0} أيام",
            "خلال {0} يومًا",
            "خلال {0} يوم",
        ),
        p6(
            "{0} يوم",
            "يوم واحد",
            "يومان",
            "{0} أيام",
            "{0} يومًا",
            "{0} يوم",
        ),
        "أول أمس",
        "أمس",
        "اليوم",
        "غدًا",
        "بعد الغد",
    ),
    week: u(
        p6(
            "قبل {0} أسبوع",
            "قبل أسبوع واحد",
            "قبل أسبوعين",
            "قبل {0} أسابيع",
            "قبل {0} أسبوعًا",
            "قبل {0} أسبوع",
        ),
        p6(
            "خلال {0} أسبوع",
            "خلال أسبوع واحد",
            "خلال أسبوعين",
            "خلال {0} أسابيع",
            "خلال {0} أسبوعًا",
            "خلال {0} أسبوع",
        ),
        p6(
            "{0} أسبوع",
            "أسبوع واحد",
            "أسبوعان",
            "{0} أسابيع",
            "{0} أسبوعًا",
            "{0} أسبوع",
        ),
        "الأسبوع الماضي",
        "هذا الأسبوع",
        "الأسبوع القادم",
    ),
    month: u(
        p6(
            "قبل {0} شهر",
            "قبل شهر واحد",
            "قبل شهرين",
            "قبل {0} أشهر",
            "قبل {0} شهرًا",
            "قبل {0} شهر",
        ),
        p6(
            "خلال {0} شهر",
            "خلال شهر واحد",
            "خلال شهرين",
            "خلال {0} أشهر",
            "خلال {0} شهرًا",
            "خلال {0} شهر",
        ),
        p6(
            "{0} شهر",
            "شهر واحد",
            "شهران",
            "{0} أشهر",
            "{0} شهرًا",
            "{0} شهر",
        ),
        "الشهر الماضي",
        "هذا الشهر",
        "الشهر القادم",
    ),
    quarter: u(
        p6(
            "قبل {0} ربع سنة",
            "قبل ربع سنة",
            "قبل ربعي سنة",
            "قبل {0} أرباع سنة",
            "قبل {0} ربع سنة",
            "قبل {0} ربع سنة",
        ),
        p6(
            "خلال {0} ربع سنة",
            "خلال ربع سنة",
            "خلال ربعي سنة",
            "خلال {0} أرباع سنة",
            "خلال {0} ربع سنة",
            "خلال {0} ربع سنة",
        ),
        p6(
            "{0} ربع سنة",
            "ربع سنة",
            "ربعا سنة",
            "{0} أرباع سنة",
            "{0} ربع سنة",
            "{0} ربع سنة",
        ),
        "الربع الأخير",
        "هذا الربع",
        "الربع القادم",
    ),
    year: u(
        p6(
            "قبل {0} سنة",
            "قبل سنة واحدة",
            "قبل سنتين",
            "قبل {0} سنوات",
            "قبل {0} سنة",
            "قبل {0} سنة",
        ),
        p6(
            "خلال {0} سنة",
            "خلال سنة واحدة",
            "خلال سنتين",
            "خلال {0} سنوات",
            "خلال {0} سنة",
            "خلال {0} سنة",
        ),
        p6(
            "{0} سنة",
            "سنة واحدة",
            "سنتان",
            "{0} سنوات",
            "{0} سنة",
            "{0} سنة",
        ),
        "السنة الماضية",
        "هذه السنة",
        "السنة القادمة",
    ),
};

const AR: LocaleData = LocaleData {
    tag: "ar",
    long: AR_LONG,
    short: StyleData::EMPTY,
    narrow: StyleData::EMPTY,
    // Left empty on purpose: Arabic has no `2h30m` convention, and mixing
    // the root's Latin suffixes into a right-to-left string would need bidi
    // isolation the compact form does not carry. Use `Narrow` instead.
    compact: UnitStrings::EMPTY,
    indefinite: UnitStrings::EMPTY,
    list: lists("{0} و{1}", "{0} و{1}", "{0} {1}"),
    approximate: ApproximatePatterns {
        exactly: "{0}",
        about: "حوالي {0}",
        just_over: "أكثر من {0} بقليل",
        over: "أكثر من {0}",
        nearly: "ما يقرب من {0}",
        less_than: "أقل من {0}",
        more_than: "أكثر من {0}",
    },
    weekday: WeekdayPatterns {
        previous: "{0} الأسبوع الماضي",
        current: "{0} هذا الأسبوع",
        next: "{0} الأسبوع القادم",
    },
    decimal_separator: "٫",
    at_pattern: "{0} في {1}",
};

// --- Czech ----------------------------------------------------------------
//
// Czech puts the past under a preposition that governs the instrumental —
// *před třemi dny* — so its past patterns share no stem with its undirected
// ones: *3 dny* but *před 3 dny*, *3 roky* but *před 3 lety*. Its `many`
// category is the fraction category, so an integer never reaches it.

const CS_LONG: StyleData = StyleData {
    second: u(
        p4(
            "před {0} sekundou",
            "před {0} sekundami",
            "před {0} sekundy",
            "před {0} sekundami",
        ),
        p4(
            "za {0} sekundu",
            "za {0} sekundy",
            "za {0} sekundy",
            "za {0} sekund",
        ),
        p4("{0} sekunda", "{0} sekundy", "{0} sekundy", "{0} sekund"),
        "",
        "nyní",
        "",
    ),
    minute: u(
        p4(
            "před {0} minutou",
            "před {0} minutami",
            "před {0} minuty",
            "před {0} minutami",
        ),
        p4(
            "za {0} minutu",
            "za {0} minuty",
            "za {0} minuty",
            "za {0} minut",
        ),
        p4("{0} minuta", "{0} minuty", "{0} minuty", "{0} minut"),
        "",
        "tato minuta",
        "",
    ),
    hour: u_half(
        u(
            p4(
                "před {0} hodinou",
                "před {0} hodinami",
                "před {0} hodiny",
                "před {0} hodinami",
            ),
            p4(
                "za {0} hodinu",
                "za {0} hodiny",
                "za {0} hodiny",
                "za {0} hodin",
            ),
            p4("{0} hodina", "{0} hodiny", "{0} hodiny", "{0} hodin"),
            "",
            "tato hodina",
            "",
        ),
        "půl hodiny",
        "hodina a půl",
    ),
    day: u_day(
        p4(
            "před {0} dnem",
            "před {0} dny",
            "před {0} dne",
            "před {0} dny",
        ),
        p4("za {0} den", "za {0} dny", "za {0} dne", "za {0} dní"),
        p4("{0} den", "{0} dny", "{0} dne", "{0} dní"),
        "předevčírem",
        "včera",
        "dnes",
        "zítra",
        "pozítří",
    ),
    week: u(
        p4(
            "před {0} týdnem",
            "před {0} týdny",
            "před {0} týdne",
            "před {0} týdny",
        ),
        p4(
            "za {0} týden",
            "za {0} týdny",
            "za {0} týdne",
            "za {0} týdnů",
        ),
        p4("{0} týden", "{0} týdny", "{0} týdne", "{0} týdnů"),
        "minulý týden",
        "tento týden",
        "příští týden",
    ),
    month: u(
        p4(
            "před {0} měsícem",
            "před {0} měsíci",
            "před {0} měsíce",
            "před {0} měsíci",
        ),
        p4(
            "za {0} měsíc",
            "za {0} měsíce",
            "za {0} měsíce",
            "za {0} měsíců",
        ),
        p4("{0} měsíc", "{0} měsíce", "{0} měsíce", "{0} měsíců"),
        "minulý měsíc",
        "tento měsíc",
        "příští měsíc",
    ),
    quarter: u(
        p4(
            "před {0} čtvrtletím",
            "před {0} čtvrtletími",
            "před {0} čtvrtletí",
            "před {0} čtvrtletími",
        ),
        p4(
            "za {0} čtvrtletí",
            "za {0} čtvrtletí",
            "za {0} čtvrtletí",
            "za {0} čtvrtletí",
        ),
        p4(
            "{0} čtvrtletí",
            "{0} čtvrtletí",
            "{0} čtvrtletí",
            "{0} čtvrtletí",
        ),
        "minulé čtvrtletí",
        "toto čtvrtletí",
        "příští čtvrtletí",
    ),
    year: u(
        p4(
            "před {0} rokem",
            "před {0} lety",
            "před {0} roku",
            "před {0} lety",
        ),
        p4("za {0} rok", "za {0} roky", "za {0} roku", "za {0} let"),
        p4("{0} rok", "{0} roky", "{0} roku", "{0} let"),
        "minulý rok",
        "tento rok",
        "příští rok",
    ),
};

const CS: LocaleData = LocaleData {
    tag: "cs",
    long: CS_LONG,
    short: StyleData::EMPTY,
    narrow: StyleData::EMPTY,
    compact: UnitStrings {
        second: "s",
        minute: "min",
        hour: "h",
        day: "d",
        week: "týd",
        month: "měs",
        quarter: "čtvrtl",
        year: "r",
    },
    indefinite: UnitStrings::EMPTY,
    list: lists("{0}, {1}", "{0} a {1}", "{0} {1}"),
    approximate: ApproximatePatterns {
        exactly: "{0}",
        about: "přibližně {0}",
        just_over: "něco přes {0}",
        over: "přes {0}",
        nearly: "téměř {0}",
        less_than: "méně než {0}",
        more_than: "více než {0}",
    },
    weekday: WeekdayPatterns {
        previous: "{0} minulý týden",
        current: "{0} tento týden",
        next: "{0} příští týden",
    },
    decimal_separator: ",",
    at_pattern: "{0} v {1}",
};

// --- German ---------------------------------------------------------------
//
// German is the counterexample that makes `UnitPatterns::count` a separate
// field rather than a prefix stripped off the past form: the preposition
// *vor* takes the dative, so it is *3 Tage* but *vor 3 Tagen*, *3 Monate*
// but *vor 3 Monaten*, *3 Jahre* but *vor 3 Jahren*. Minutes, hours and
// weeks happen to coincide, which is exactly why the mistake is easy to
// miss.

const DE_LONG: StyleData = StyleData {
    second: u(
        p2("vor {0} Sekunde", "vor {0} Sekunden"),
        p2("in {0} Sekunde", "in {0} Sekunden"),
        p2("{0} Sekunde", "{0} Sekunden"),
        "",
        "jetzt",
        "",
    ),
    minute: u(
        p2("vor {0} Minute", "vor {0} Minuten"),
        p2("in {0} Minute", "in {0} Minuten"),
        p2("{0} Minute", "{0} Minuten"),
        "",
        "in dieser Minute",
        "",
    ),
    hour: u_half(
        u(
            p2("vor {0} Stunde", "vor {0} Stunden"),
            p2("in {0} Stunde", "in {0} Stunden"),
            p2("{0} Stunde", "{0} Stunden"),
            "",
            "in dieser Stunde",
            "",
        ),
        "eine halbe Stunde",
        "anderthalb Stunden",
    ),
    day: u_day(
        p2("vor {0} Tag", "vor {0} Tagen"),
        p2("in {0} Tag", "in {0} Tagen"),
        p2("{0} Tag", "{0} Tage"),
        "vorgestern",
        "gestern",
        "heute",
        "morgen",
        "übermorgen",
    ),
    week: u(
        p2("vor {0} Woche", "vor {0} Wochen"),
        p2("in {0} Woche", "in {0} Wochen"),
        p2("{0} Woche", "{0} Wochen"),
        "letzte Woche",
        "diese Woche",
        "nächste Woche",
    ),
    month: u(
        p2("vor {0} Monat", "vor {0} Monaten"),
        p2("in {0} Monat", "in {0} Monaten"),
        p2("{0} Monat", "{0} Monate"),
        "letzten Monat",
        "diesen Monat",
        "nächsten Monat",
    ),
    quarter: u(
        p2("vor {0} Quartal", "vor {0} Quartalen"),
        p2("in {0} Quartal", "in {0} Quartalen"),
        p2("{0} Quartal", "{0} Quartale"),
        "letztes Quartal",
        "dieses Quartal",
        "nächstes Quartal",
    ),
    year: u(
        p2("vor {0} Jahr", "vor {0} Jahren"),
        p2("in {0} Jahr", "in {0} Jahren"),
        p2("{0} Jahr", "{0} Jahre"),
        "letztes Jahr",
        "dieses Jahr",
        "nächstes Jahr",
    ),
};

/// German abbreviates the unit noun but not the preposition, and it does not
/// abbreviate *Tag* at all — so the day entry is left empty and inherits the
/// long form, which is what CLDR does too.
const DE_SHORT: StyleData = StyleData {
    second: u(
        p1("vor {0} Sek."),
        p1("in {0} Sek."),
        p1("{0} Sek."),
        "",
        "jetzt",
        "",
    ),
    minute: u(
        p1("vor {0} Min."),
        p1("in {0} Min."),
        p1("{0} Min."),
        "",
        "in dieser Minute",
        "",
    ),
    hour: u(
        p1("vor {0} Std."),
        p1("in {0} Std."),
        p1("{0} Std."),
        "",
        "in dieser Stunde",
        "",
    ),
    day: UnitPatterns::EMPTY,
    week: u(
        p1("vor {0} Wo."),
        p1("in {0} Wo."),
        p1("{0} Wo."),
        "letzte Wo.",
        "diese Wo.",
        "nächste Wo.",
    ),
    month: u(
        p1("vor {0} Mon."),
        p1("in {0} Mon."),
        p1("{0} Mon."),
        "letzten Mon.",
        "diesen Mon.",
        "nächsten Mon.",
    ),
    quarter: u(
        p1("vor {0} Q"),
        p1("in {0} Q"),
        p1("{0} Q"),
        "letztes Q",
        "dieses Q",
        "nächstes Q",
    ),
    year: u(
        p1("vor {0} J"),
        p1("in {0} J"),
        p1("{0} J"),
        "letztes J",
        "dieses J",
        "nächstes J",
    ),
};

const DE: LocaleData = LocaleData {
    tag: "de",
    long: DE_LONG,
    short: DE_SHORT,
    // German narrow and short coincide, so narrow states nothing and the
    // style fallback finds the short forms.
    narrow: StyleData::EMPTY,
    compact: UnitStrings {
        second: "s",
        minute: "min",
        hour: "h",
        day: "d",
        week: "W",
        month: "M",
        quarter: "Q",
        year: "J",
    },
    indefinite: strings(
        "eine Sekunde",
        "eine Minute",
        "eine Stunde",
        "ein Tag",
        "eine Woche",
        "ein Monat",
        "ein Quartal",
        "ein Jahr",
    ),
    list: lists("{0}, {1}", "{0} und {1}", "{0} {1}"),
    approximate: ApproximatePatterns {
        exactly: "{0}",
        about: "ungefähr {0}",
        just_over: "etwas mehr als {0}",
        over: "mehr als {0}",
        nearly: "fast {0}",
        less_than: "weniger als {0}",
        more_than: "mehr als {0}",
    },
    weekday: WeekdayPatterns {
        previous: "letzten {0}",
        current: "diesen {0}",
        next: "nächsten {0}",
    },
    decimal_separator: ",",
    at_pattern: "{0} um {1}",
};

// --- English --------------------------------------------------------------

const EN_LONG: StyleData = StyleData {
    second: u(
        p2("{0} second ago", "{0} seconds ago"),
        p2("in {0} second", "in {0} seconds"),
        p2("{0} second", "{0} seconds"),
        "",
        "now",
        "",
    ),
    minute: u(
        p2("{0} minute ago", "{0} minutes ago"),
        p2("in {0} minute", "in {0} minutes"),
        p2("{0} minute", "{0} minutes"),
        "",
        "this minute",
        "",
    ),
    hour: u_half(
        u(
            p2("{0} hour ago", "{0} hours ago"),
            p2("in {0} hour", "in {0} hours"),
            p2("{0} hour", "{0} hours"),
            "",
            "this hour",
            "",
        ),
        "half an hour",
        "an hour and a half",
    ),
    day: u_half(
        u_day(
            p2("{0} day ago", "{0} days ago"),
            p2("in {0} day", "in {0} days"),
            p2("{0} day", "{0} days"),
            "the day before yesterday",
            "yesterday",
            "today",
            "tomorrow",
            "the day after tomorrow",
        ),
        "half a day",
        "a day and a half",
    ),
    week: u(
        p2("{0} week ago", "{0} weeks ago"),
        p2("in {0} week", "in {0} weeks"),
        p2("{0} week", "{0} weeks"),
        "last week",
        "this week",
        "next week",
    ),
    month: u(
        p2("{0} month ago", "{0} months ago"),
        p2("in {0} month", "in {0} months"),
        p2("{0} month", "{0} months"),
        "last month",
        "this month",
        "next month",
    ),
    quarter: u(
        p2("{0} quarter ago", "{0} quarters ago"),
        p2("in {0} quarter", "in {0} quarters"),
        p2("{0} quarter", "{0} quarters"),
        "last quarter",
        "this quarter",
        "next quarter",
    ),
    year: u(
        p2("{0} year ago", "{0} years ago"),
        p2("in {0} year", "in {0} years"),
        p2("{0} year", "{0} years"),
        "last year",
        "this year",
        "next year",
    ),
};

const EN_SHORT: StyleData = StyleData {
    second: u(
        p1("{0} sec. ago"),
        p1("in {0} sec."),
        p1("{0} sec."),
        "",
        "now",
        "",
    ),
    minute: u(
        p1("{0} min. ago"),
        p1("in {0} min."),
        p1("{0} min."),
        "",
        "this minute",
        "",
    ),
    hour: u(
        p1("{0} hr. ago"),
        p1("in {0} hr."),
        p1("{0} hr."),
        "",
        "this hour",
        "",
    ),
    // English short does not abbreviate "day", so only the special words
    // differ from the long style — and they do not, which is why this entry
    // repeats the long patterns rather than inheriting: a short-style caller
    // asking for a day must still get "yesterday".
    day: u_day(
        p2("{0} day ago", "{0} days ago"),
        p2("in {0} day", "in {0} days"),
        p2("{0} day", "{0} days"),
        "the day before yesterday",
        "yesterday",
        "today",
        "tomorrow",
        "the day after tomorrow",
    ),
    week: u(
        p1("{0} wk. ago"),
        p1("in {0} wk."),
        p1("{0} wk."),
        "last wk.",
        "this wk.",
        "next wk.",
    ),
    month: u(
        p1("{0} mo. ago"),
        p1("in {0} mo."),
        p1("{0} mo."),
        "last mo.",
        "this mo.",
        "next mo.",
    ),
    quarter: u(
        p1("{0} qtr. ago"),
        p1("in {0} qtr."),
        p1("{0} qtr."),
        "last qtr.",
        "this qtr.",
        "next qtr.",
    ),
    year: u(
        p1("{0} yr. ago"),
        p1("in {0} yr."),
        p1("{0} yr."),
        "last yr.",
        "this yr.",
        "next yr.",
    ),
};

const EN_NARROW: StyleData = StyleData {
    second: u(p1("{0}s ago"), p1("in {0}s"), p1("{0}s"), "", "now", ""),
    minute: u(
        p1("{0}m ago"),
        p1("in {0}m"),
        p1("{0}m"),
        "",
        "this minute",
        "",
    ),
    hour: u(
        p1("{0}h ago"),
        p1("in {0}h"),
        p1("{0}h"),
        "",
        "this hour",
        "",
    ),
    day: u_day(
        p1("{0}d ago"),
        p1("in {0}d"),
        p1("{0}d"),
        "",
        "yesterday",
        "today",
        "tomorrow",
        "",
    ),
    week: u(
        p1("{0}w ago"),
        p1("in {0}w"),
        p1("{0}w"),
        "last wk.",
        "this wk.",
        "next wk.",
    ),
    month: u(
        p1("{0}mo ago"),
        p1("in {0}mo"),
        p1("{0}mo"),
        "last mo.",
        "this mo.",
        "next mo.",
    ),
    quarter: u(
        p1("{0}q ago"),
        p1("in {0}q"),
        p1("{0}q"),
        "last qtr.",
        "this qtr.",
        "next qtr.",
    ),
    year: u(
        p1("{0}y ago"),
        p1("in {0}y"),
        p1("{0}y"),
        "last yr.",
        "this yr.",
        "next yr.",
    ),
};

const EN: LocaleData = LocaleData {
    tag: "en",
    long: EN_LONG,
    short: EN_SHORT,
    narrow: EN_NARROW,
    compact: UnitStrings {
        second: "s",
        minute: "m",
        hour: "h",
        day: "d",
        week: "w",
        month: "mo",
        quarter: "q",
        year: "y",
    },
    indefinite: strings(
        "a second",
        "a minute",
        "an hour",
        "a day",
        "a week",
        "a month",
        "a quarter",
        "a year",
    ),
    list: lists("{0}, {1}", "{0} and {1}", "{0} {1}"),
    approximate: ApproximatePatterns {
        exactly: "{0}",
        about: "about {0}",
        just_over: "just over {0}",
        over: "over {0}",
        nearly: "nearly {0}",
        less_than: "less than {0}",
        more_than: "more than {0}",
    },
    weekday: WeekdayPatterns {
        previous: "last {0}",
        current: "this {0}",
        next: "next {0}",
    },
    decimal_separator: ".",
    at_pattern: "{0} at {1}",
};

// --- Spanish --------------------------------------------------------------

const ES_LONG: StyleData = StyleData {
    second: u(
        p2("hace {0} segundo", "hace {0} segundos"),
        p2("dentro de {0} segundo", "dentro de {0} segundos"),
        p2("{0} segundo", "{0} segundos"),
        "",
        "ahora",
        "",
    ),
    minute: u(
        p2("hace {0} minuto", "hace {0} minutos"),
        p2("dentro de {0} minuto", "dentro de {0} minutos"),
        p2("{0} minuto", "{0} minutos"),
        "",
        "este minuto",
        "",
    ),
    hour: u_half(
        u(
            p2("hace {0} hora", "hace {0} horas"),
            p2("dentro de {0} hora", "dentro de {0} horas"),
            p2("{0} hora", "{0} horas"),
            "",
            "esta hora",
            "",
        ),
        "media hora",
        "una hora y media",
    ),
    day: u_day(
        p2("hace {0} día", "hace {0} días"),
        p2("dentro de {0} día", "dentro de {0} días"),
        p2("{0} día", "{0} días"),
        "anteayer",
        "ayer",
        "hoy",
        "mañana",
        "pasado mañana",
    ),
    week: u(
        p2("hace {0} semana", "hace {0} semanas"),
        p2("dentro de {0} semana", "dentro de {0} semanas"),
        p2("{0} semana", "{0} semanas"),
        "la semana pasada",
        "esta semana",
        "la próxima semana",
    ),
    month: u(
        p2("hace {0} mes", "hace {0} meses"),
        p2("dentro de {0} mes", "dentro de {0} meses"),
        p2("{0} mes", "{0} meses"),
        "el mes pasado",
        "este mes",
        "el próximo mes",
    ),
    quarter: u(
        p2("hace {0} trimestre", "hace {0} trimestres"),
        p2("dentro de {0} trimestre", "dentro de {0} trimestres"),
        p2("{0} trimestre", "{0} trimestres"),
        "el trimestre pasado",
        "este trimestre",
        "el próximo trimestre",
    ),
    year: u(
        p2("hace {0} año", "hace {0} años"),
        p2("dentro de {0} año", "dentro de {0} años"),
        p2("{0} año", "{0} años"),
        "el año pasado",
        "este año",
        "el próximo año",
    ),
};

const ES_SHORT: StyleData = StyleData {
    second: u(
        p1("hace {0} s"),
        p1("dentro de {0} s"),
        p1("{0} s"),
        "",
        "ahora",
        "",
    ),
    minute: u(
        p1("hace {0} min"),
        p1("dentro de {0} min"),
        p1("{0} min"),
        "",
        "este minuto",
        "",
    ),
    hour: u(
        p1("hace {0} h"),
        p1("dentro de {0} h"),
        p1("{0} h"),
        "",
        "esta hora",
        "",
    ),
    day: u_day(
        p1("hace {0} d"),
        p1("dentro de {0} d"),
        p1("{0} d"),
        "anteayer",
        "ayer",
        "hoy",
        "mañana",
        "pasado mañana",
    ),
    week: u(
        p1("hace {0} sem."),
        p1("dentro de {0} sem."),
        p1("{0} sem."),
        "sem. pasada",
        "esta sem.",
        "próx. sem.",
    ),
    month: u(
        p1("hace {0} m"),
        p1("dentro de {0} m"),
        p1("{0} m"),
        "el mes pasado",
        "este mes",
        "el próximo mes",
    ),
    quarter: u(
        p1("hace {0} trim."),
        p1("dentro de {0} trim."),
        p1("{0} trim."),
        "trim. pasado",
        "este trim.",
        "próx. trim.",
    ),
    year: u(
        p1("hace {0} a"),
        p1("dentro de {0} a"),
        p1("{0} a"),
        "el año pasado",
        "este año",
        "el próximo año",
    ),
};

const ES: LocaleData = LocaleData {
    tag: "es",
    long: ES_LONG,
    short: ES_SHORT,
    narrow: StyleData::EMPTY,
    compact: UnitStrings {
        second: "s",
        minute: "min",
        hour: "h",
        day: "d",
        week: "sem",
        month: "m",
        quarter: "trim",
        year: "a",
    },
    indefinite: strings(
        "un segundo",
        "un minuto",
        "una hora",
        "un día",
        "una semana",
        "un mes",
        "un trimestre",
        "un año",
    ),
    list: lists("{0}, {1}", "{0} y {1}", "{0} {1}"),
    approximate: ApproximatePatterns {
        exactly: "{0}",
        about: "aproximadamente {0}",
        just_over: "poco más de {0}",
        over: "más de {0}",
        nearly: "casi {0}",
        less_than: "menos de {0}",
        more_than: "más de {0}",
    },
    weekday: WeekdayPatterns {
        previous: "el {0} pasado",
        current: "este {0}",
        next: "el próximo {0}",
    },
    decimal_separator: ",",
    at_pattern: "{0} a las {1}",
};

// --- French ---------------------------------------------------------------
//
// French puts 0 in the `one` category, so *dans 0 jour* is singular. That is
// the CLDR rule `one: i = 0,1`, not an oversight.

const FR_LONG: StyleData = StyleData {
    second: u(
        p2("il y a {0} seconde", "il y a {0} secondes"),
        p2("dans {0} seconde", "dans {0} secondes"),
        p2("{0} seconde", "{0} secondes"),
        "",
        "maintenant",
        "",
    ),
    minute: u(
        p2("il y a {0} minute", "il y a {0} minutes"),
        p2("dans {0} minute", "dans {0} minutes"),
        p2("{0} minute", "{0} minutes"),
        "",
        "cette minute-ci",
        "",
    ),
    hour: u_half(
        u(
            p2("il y a {0} heure", "il y a {0} heures"),
            p2("dans {0} heure", "dans {0} heures"),
            p2("{0} heure", "{0} heures"),
            "",
            "cette heure-ci",
            "",
        ),
        "une demi-heure",
        "une heure et demie",
    ),
    day: u_day(
        p2("il y a {0} jour", "il y a {0} jours"),
        p2("dans {0} jour", "dans {0} jours"),
        p2("{0} jour", "{0} jours"),
        "avant-hier",
        "hier",
        "aujourd’hui",
        "demain",
        "après-demain",
    ),
    week: u(
        p2("il y a {0} semaine", "il y a {0} semaines"),
        p2("dans {0} semaine", "dans {0} semaines"),
        p2("{0} semaine", "{0} semaines"),
        "la semaine dernière",
        "cette semaine",
        "la semaine prochaine",
    ),
    month: u(
        p1("il y a {0} mois"),
        p1("dans {0} mois"),
        p1("{0} mois"),
        "le mois dernier",
        "ce mois-ci",
        "le mois prochain",
    ),
    quarter: u(
        p2("il y a {0} trimestre", "il y a {0} trimestres"),
        p2("dans {0} trimestre", "dans {0} trimestres"),
        p2("{0} trimestre", "{0} trimestres"),
        "le trimestre dernier",
        "ce trimestre",
        "le trimestre prochain",
    ),
    year: u(
        p2("il y a {0} an", "il y a {0} ans"),
        p2("dans {0} an", "dans {0} ans"),
        p2("{0} an", "{0} ans"),
        "l’année dernière",
        "cette année",
        "l’année prochaine",
    ),
};

const FR_SHORT: StyleData = StyleData {
    second: u(
        p1("il y a {0} s"),
        p1("dans {0} s"),
        p1("{0} s"),
        "",
        "maintenant",
        "",
    ),
    minute: u(
        p1("il y a {0} min"),
        p1("dans {0} min"),
        p1("{0} min"),
        "",
        "cette minute-ci",
        "",
    ),
    hour: u(
        p1("il y a {0} h"),
        p1("dans {0} h"),
        p1("{0} h"),
        "",
        "cette heure-ci",
        "",
    ),
    day: u_day(
        p1("il y a {0} j"),
        p1("dans {0} j"),
        p1("{0} j"),
        "avant-hier",
        "hier",
        "aujourd’hui",
        "demain",
        "après-demain",
    ),
    week: u(
        p1("il y a {0} sem."),
        p1("dans {0} sem."),
        p1("{0} sem."),
        "la semaine dernière",
        "cette semaine",
        "la semaine prochaine",
    ),
    month: u(
        p1("il y a {0} m."),
        p1("dans {0} m."),
        p1("{0} m."),
        "le mois dernier",
        "ce mois-ci",
        "le mois prochain",
    ),
    quarter: u(
        p1("il y a {0} trim."),
        p1("dans {0} trim."),
        p1("{0} trim."),
        "le trimestre dernier",
        "ce trimestre",
        "le trimestre prochain",
    ),
    year: u(
        p1("il y a {0} a"),
        p1("dans {0} a"),
        p1("{0} a"),
        "l’année dernière",
        "cette année",
        "l’année prochaine",
    ),
};

const FR: LocaleData = LocaleData {
    tag: "fr",
    long: FR_LONG,
    short: FR_SHORT,
    narrow: StyleData::EMPTY,
    compact: UnitStrings {
        second: "s",
        minute: "min",
        hour: "h",
        day: "j",
        week: "sem",
        month: "m",
        quarter: "trim",
        year: "a",
    },
    indefinite: strings(
        "une seconde",
        "une minute",
        "une heure",
        "un jour",
        "une semaine",
        "un mois",
        "un trimestre",
        "un an",
    ),
    list: lists("{0}, {1}", "{0} et {1}", "{0} {1}"),
    approximate: ApproximatePatterns {
        exactly: "{0}",
        about: "environ {0}",
        just_over: "{0} et quelques",
        over: "{0} et plus",
        nearly: "presque {0}",
        less_than: "{0} au maximum",
        more_than: "{0} au minimum",
    },
    weekday: WeekdayPatterns {
        previous: "{0} dernier",
        current: "ce {0}",
        next: "{0} prochain",
    },
    decimal_separator: ",",
    at_pattern: "{0} à {1}",
};

// --- Hindi ----------------------------------------------------------------
//
// Hindi has one word, कल, for both yesterday and tomorrow, and one word,
// परसों, for both the day before and the day after; direction comes from the
// verb. So the ±1 and ±2 special words are deliberately identical, and
// `Numeric::Auto` on a Hindi day offset produces a phrase that is only
// unambiguous inside a sentence.

const HI_LONG: StyleData = StyleData {
    second: u(
        p1("{0} सेकंड पहले"),
        p1("{0} सेकंड में"),
        p1("{0} सेकंड"),
        "",
        "अब",
        "",
    ),
    minute: u(
        p1("{0} मिनट पहले"),
        p1("{0} मिनट में"),
        p1("{0} मिनट"),
        "",
        "यह मिनट",
        "",
    ),
    hour: u_half(
        u(
            p2("{0} घंटा पहले", "{0} घंटे पहले"),
            p2("{0} घंटे में", "{0} घंटे में"),
            p2("{0} घंटा", "{0} घंटे"),
            "",
            "यह घंटा",
            "",
        ),
        "आधा घंटा",
        "डेढ़ घंटा",
    ),
    day: u_day(
        p1("{0} दिन पहले"),
        p1("{0} दिन में"),
        p1("{0} दिन"),
        "परसों",
        "कल",
        "आज",
        "कल",
        "परसों",
    ),
    week: u(
        p1("{0} सप्ताह पहले"),
        p1("{0} सप्ताह में"),
        p1("{0} सप्ताह"),
        "पिछला सप्ताह",
        "यह सप्ताह",
        "अगला सप्ताह",
    ),
    month: u(
        p1("{0} माह पहले"),
        p1("{0} माह में"),
        p1("{0} माह"),
        "पिछला माह",
        "इस माह",
        "अगला माह",
    ),
    quarter: u(
        p1("{0} तिमाही पहले"),
        p1("{0} तिमाही में"),
        p1("{0} तिमाही"),
        "पिछली तिमाही",
        "इस तिमाही",
        "अगली तिमाही",
    ),
    year: u(
        p1("{0} वर्ष पहले"),
        p1("{0} वर्ष में"),
        p1("{0} वर्ष"),
        "पिछला वर्ष",
        "इस वर्ष",
        "अगला वर्ष",
    ),
};

const HI: LocaleData = LocaleData {
    tag: "hi",
    long: HI_LONG,
    short: StyleData::EMPTY,
    narrow: StyleData::EMPTY,
    compact: UnitStrings::EMPTY,
    indefinite: UnitStrings::EMPTY,
    list: lists("{0}, {1}", "{0} और {1}", "{0} {1}"),
    approximate: ApproximatePatterns {
        exactly: "{0}",
        about: "लगभग {0}",
        just_over: "{0} से कुछ अधिक",
        over: "{0} से अधिक",
        nearly: "{0} से कुछ कम",
        less_than: "{0} से कम",
        more_than: "{0} से अधिक",
    },
    weekday: WeekdayPatterns {
        previous: "पिछला {0}",
        current: "इस {0}",
        next: "अगला {0}",
    },
    decimal_separator: ".",
    at_pattern: "{0} को {1}",
};

// --- Indonesian -----------------------------------------------------------

const ID_LONG: StyleData = StyleData {
    second: u(
        p1("{0} detik yang lalu"),
        p1("dalam {0} detik"),
        p1("{0} detik"),
        "",
        "sekarang",
        "",
    ),
    minute: u(
        p1("{0} menit yang lalu"),
        p1("dalam {0} menit"),
        p1("{0} menit"),
        "",
        "menit ini",
        "",
    ),
    hour: u_half(
        u(
            p1("{0} jam yang lalu"),
            p1("dalam {0} jam"),
            p1("{0} jam"),
            "",
            "jam ini",
            "",
        ),
        "setengah jam",
        "satu setengah jam",
    ),
    day: u_day(
        p1("{0} hari yang lalu"),
        p1("dalam {0} hari"),
        p1("{0} hari"),
        "kemarin dulu",
        "kemarin",
        "hari ini",
        "besok",
        "lusa",
    ),
    week: u(
        p1("{0} minggu yang lalu"),
        p1("dalam {0} minggu"),
        p1("{0} minggu"),
        "minggu lalu",
        "minggu ini",
        "minggu depan",
    ),
    month: u(
        p1("{0} bulan yang lalu"),
        p1("dalam {0} bulan"),
        p1("{0} bulan"),
        "bulan lalu",
        "bulan ini",
        "bulan depan",
    ),
    quarter: u(
        p1("{0} kuartal yang lalu"),
        p1("dalam {0} kuartal"),
        p1("{0} kuartal"),
        "kuartal lalu",
        "kuartal ini",
        "kuartal berikutnya",
    ),
    year: u(
        p1("{0} tahun yang lalu"),
        p1("dalam {0} tahun"),
        p1("{0} tahun"),
        "tahun lalu",
        "tahun ini",
        "tahun depan",
    ),
};

const ID: LocaleData = LocaleData {
    tag: "id",
    long: ID_LONG,
    short: StyleData::EMPTY,
    narrow: StyleData::EMPTY,
    compact: UnitStrings {
        second: "dtk",
        minute: "mnt",
        hour: "j",
        day: "h",
        week: "mgg",
        month: "bln",
        quarter: "kuartal",
        year: "thn",
    },
    indefinite: strings(
        "satu detik",
        "satu menit",
        "satu jam",
        "satu hari",
        "satu minggu",
        "satu bulan",
        "satu kuartal",
        "satu tahun",
    ),
    list: lists("{0}, {1}", "{0} dan {1}", "{0} {1}"),
    approximate: ApproximatePatterns {
        exactly: "{0}",
        about: "sekitar {0}",
        just_over: "sedikit lebih dari {0}",
        over: "lebih dari {0}",
        nearly: "hampir {0}",
        less_than: "kurang dari {0}",
        more_than: "lebih dari {0}",
    },
    weekday: WeekdayPatterns {
        previous: "{0} lalu",
        current: "{0} ini",
        next: "{0} depan",
    },
    decimal_separator: ",",
    at_pattern: "{0} pukul {1}",
};

// --- Italian --------------------------------------------------------------

const IT_LONG: StyleData = StyleData {
    second: u(
        p2("{0} secondo fa", "{0} secondi fa"),
        p2("tra {0} secondo", "tra {0} secondi"),
        p2("{0} secondo", "{0} secondi"),
        "",
        "ora",
        "",
    ),
    minute: u(
        p2("{0} minuto fa", "{0} minuti fa"),
        p2("tra {0} minuto", "tra {0} minuti"),
        p2("{0} minuto", "{0} minuti"),
        "",
        "questo minuto",
        "",
    ),
    hour: u_half(
        u(
            p2("{0} ora fa", "{0} ore fa"),
            p2("tra {0} ora", "tra {0} ore"),
            p2("{0} ora", "{0} ore"),
            "",
            "quest’ora",
            "",
        ),
        "mezz’ora",
        "un’ora e mezza",
    ),
    day: u_day(
        p2("{0} giorno fa", "{0} giorni fa"),
        p2("tra {0} giorno", "tra {0} giorni"),
        p2("{0} giorno", "{0} giorni"),
        "l’altro ieri",
        "ieri",
        "oggi",
        "domani",
        "dopodomani",
    ),
    week: u(
        p2("{0} settimana fa", "{0} settimane fa"),
        p2("tra {0} settimana", "tra {0} settimane"),
        p2("{0} settimana", "{0} settimane"),
        "settimana scorsa",
        "questa settimana",
        "settimana prossima",
    ),
    month: u(
        p2("{0} mese fa", "{0} mesi fa"),
        p2("tra {0} mese", "tra {0} mesi"),
        p2("{0} mese", "{0} mesi"),
        "mese scorso",
        "questo mese",
        "mese prossimo",
    ),
    quarter: u(
        p2("{0} trimestre fa", "{0} trimestri fa"),
        p2("tra {0} trimestre", "tra {0} trimestri"),
        p2("{0} trimestre", "{0} trimestri"),
        "trimestre scorso",
        "questo trimestre",
        "trimestre prossimo",
    ),
    year: u(
        p2("{0} anno fa", "{0} anni fa"),
        p2("tra {0} anno", "tra {0} anni"),
        p2("{0} anno", "{0} anni"),
        "anno scorso",
        "quest’anno",
        "anno prossimo",
    ),
};

const IT: LocaleData = LocaleData {
    tag: "it",
    long: IT_LONG,
    short: StyleData::EMPTY,
    narrow: StyleData::EMPTY,
    compact: UnitStrings {
        second: "s",
        minute: "min",
        hour: "h",
        day: "g",
        week: "sett",
        month: "mesi",
        quarter: "trim",
        year: "a",
    },
    indefinite: strings(
        "un secondo",
        "un minuto",
        "un'ora",
        "un giorno",
        "una settimana",
        "un mese",
        "un trimestre",
        "un anno",
    ),
    list: lists("{0}, {1}", "{0} e {1}", "{0} {1}"),
    approximate: ApproximatePatterns {
        exactly: "{0}",
        about: "circa {0}",
        just_over: "poco più di {0}",
        over: "più di {0}",
        nearly: "quasi {0}",
        less_than: "meno di {0}",
        more_than: "più di {0}",
    },
    weekday: WeekdayPatterns {
        previous: "{0} della scorsa settimana",
        current: "{0} di questa settimana",
        next: "{0} della prossima settimana",
    },
    decimal_separator: ",",
    at_pattern: "{0} alle {1}",
};

// --- Japanese -------------------------------------------------------------

const JA_LONG: StyleData = StyleData {
    second: u(p1("{0} 秒前"), p1("{0} 秒後"), p1("{0} 秒"), "", "今", ""),
    minute: u(
        p1("{0} 分前"),
        p1("{0} 分後"),
        p1("{0} 分"),
        "",
        "この分",
        "",
    ),
    hour: u_half(
        u(
            p1("{0} 時間前"),
            p1("{0} 時間後"),
            p1("{0} 時間"),
            "",
            "この時間",
            "",
        ),
        "30 分",
        "1 時間半",
    ),
    day: u_day(
        p1("{0} 日前"),
        p1("{0} 日後"),
        p1("{0} 日"),
        "一昨日",
        "昨日",
        "今日",
        "明日",
        "明後日",
    ),
    week: u(
        p1("{0} 週間前"),
        p1("{0} 週間後"),
        p1("{0} 週間"),
        "先週",
        "今週",
        "来週",
    ),
    month: u(
        p1("{0} か月前"),
        p1("{0} か月後"),
        p1("{0} か月"),
        "先月",
        "今月",
        "来月",
    ),
    quarter: u(
        p1("{0} 四半期前"),
        p1("{0} 四半期後"),
        p1("{0} 四半期"),
        "前四半期",
        "今四半期",
        "翌四半期",
    ),
    year: u(
        p1("{0} 年前"),
        p1("{0} 年後"),
        p1("{0} 年"),
        "昨年",
        "今年",
        "来年",
    ),
};

/// Japanese narrow differs from long only in dropping the space before the
/// counter, which is exactly what CLDR's `ja` narrow forms do.
const JA_NARROW: StyleData = StyleData {
    second: u(p1("{0}秒前"), p1("{0}秒後"), p1("{0}秒"), "", "今", ""),
    minute: u(p1("{0}分前"), p1("{0}分後"), p1("{0}分"), "", "この分", ""),
    hour: u(
        p1("{0}時間前"),
        p1("{0}時間後"),
        p1("{0}時間"),
        "",
        "この時間",
        "",
    ),
    day: u_day(
        p1("{0}日前"),
        p1("{0}日後"),
        p1("{0}日"),
        "一昨日",
        "昨日",
        "今日",
        "明日",
        "明後日",
    ),
    week: u(
        p1("{0}週間前"),
        p1("{0}週間後"),
        p1("{0}週間"),
        "先週",
        "今週",
        "来週",
    ),
    month: u(
        p1("{0}か月前"),
        p1("{0}か月後"),
        p1("{0}か月"),
        "先月",
        "今月",
        "来月",
    ),
    quarter: u(
        p1("{0}四半期前"),
        p1("{0}四半期後"),
        p1("{0}四半期"),
        "前四半期",
        "今四半期",
        "翌四半期",
    ),
    year: u(
        p1("{0}年前"),
        p1("{0}年後"),
        p1("{0}年"),
        "昨年",
        "今年",
        "来年",
    ),
};

const JA: LocaleData = LocaleData {
    tag: "ja",
    long: JA_LONG,
    short: StyleData::EMPTY,
    narrow: JA_NARROW,
    compact: UnitStrings {
        second: "秒",
        minute: "分",
        hour: "時間",
        day: "日",
        week: "週間",
        month: "か月",
        quarter: "四半期",
        year: "年",
    },
    indefinite: UnitStrings::EMPTY,
    list: ListPatterns {
        standard: list_uniform("{0}、{1}"),
        unit: list_uniform("{0} {1}"),
        narrow: list_uniform("{0}{1}"),
    },
    approximate: ApproximatePatterns {
        exactly: "{0}",
        about: "約{0}",
        just_over: "{0}あまり",
        over: "{0}以上",
        nearly: "{0}近く",
        less_than: "{0}未満",
        more_than: "{0}超",
    },
    weekday: WeekdayPatterns {
        previous: "先週の{0}",
        current: "今週の{0}",
        next: "来週の{0}",
    },
    decimal_separator: ".",
    at_pattern: "{0} {1}",
};

// --- Korean ---------------------------------------------------------------

const KO_LONG: StyleData = StyleData {
    second: u(p1("{0}초 전"), p1("{0}초 후"), p1("{0}초"), "", "지금", ""),
    minute: u(
        p1("{0}분 전"),
        p1("{0}분 후"),
        p1("{0}분"),
        "",
        "현재 분",
        "",
    ),
    hour: u_half(
        u(
            p1("{0}시간 전"),
            p1("{0}시간 후"),
            p1("{0}시간"),
            "",
            "현재 시간",
            "",
        ),
        "30분",
        "1시간 30분",
    ),
    day: u_day(
        p1("{0}일 전"),
        p1("{0}일 후"),
        p1("{0}일"),
        "그저께",
        "어제",
        "오늘",
        "내일",
        "모레",
    ),
    week: u(
        p1("{0}주 전"),
        p1("{0}주 후"),
        p1("{0}주"),
        "지난주",
        "이번 주",
        "다음 주",
    ),
    month: u(
        p1("{0}개월 전"),
        p1("{0}개월 후"),
        p1("{0}개월"),
        "지난달",
        "이번 달",
        "다음 달",
    ),
    quarter: u(
        p1("{0}분기 전"),
        p1("{0}분기 후"),
        p1("{0}분기"),
        "지난 분기",
        "이번 분기",
        "다음 분기",
    ),
    year: u(
        p1("{0}년 전"),
        p1("{0}년 후"),
        p1("{0}년"),
        "작년",
        "올해",
        "내년",
    ),
};

const KO: LocaleData = LocaleData {
    tag: "ko",
    long: KO_LONG,
    short: StyleData::EMPTY,
    narrow: StyleData::EMPTY,
    compact: UnitStrings {
        second: "초",
        minute: "분",
        hour: "시간",
        day: "일",
        week: "주",
        month: "개월",
        quarter: "분기",
        year: "년",
    },
    indefinite: UnitStrings::EMPTY,
    list: ListPatterns {
        standard: list_conjunction("{0}, {1}", "{0} 및 {1}"),
        unit: list_uniform("{0} {1}"),
        narrow: list_uniform("{0} {1}"),
    },
    approximate: ApproximatePatterns {
        exactly: "{0}",
        about: "약 {0}",
        just_over: "{0} 조금 넘게",
        over: "{0} 이상",
        nearly: "거의 {0}",
        less_than: "{0} 미만",
        more_than: "{0} 초과",
    },
    weekday: WeekdayPatterns {
        previous: "지난주 {0}",
        current: "이번 주 {0}",
        next: "다음 주 {0}",
    },
    decimal_separator: ".",
    at_pattern: "{0} {1}",
};

// --- Dutch ----------------------------------------------------------------

const NL_LONG: StyleData = StyleData {
    second: u(
        p2("{0} seconde geleden", "{0} seconden geleden"),
        p2("over {0} seconde", "over {0} seconden"),
        p2("{0} seconde", "{0} seconden"),
        "",
        "nu",
        "",
    ),
    minute: u(
        p2("{0} minuut geleden", "{0} minuten geleden"),
        p2("over {0} minuut", "over {0} minuten"),
        p2("{0} minuut", "{0} minuten"),
        "",
        "deze minuut",
        "",
    ),
    hour: u_half(
        u(
            p1("{0} uur geleden"),
            p1("over {0} uur"),
            p1("{0} uur"),
            "",
            "dit uur",
            "",
        ),
        "een half uur",
        "anderhalf uur",
    ),
    day: u_day(
        p2("{0} dag geleden", "{0} dagen geleden"),
        p2("over {0} dag", "over {0} dagen"),
        p2("{0} dag", "{0} dagen"),
        "eergisteren",
        "gisteren",
        "vandaag",
        "morgen",
        "overmorgen",
    ),
    week: u(
        p2("{0} week geleden", "{0} weken geleden"),
        p2("over {0} week", "over {0} weken"),
        p2("{0} week", "{0} weken"),
        "vorige week",
        "deze week",
        "volgende week",
    ),
    month: u(
        p2("{0} maand geleden", "{0} maanden geleden"),
        p2("over {0} maand", "over {0} maanden"),
        p2("{0} maand", "{0} maanden"),
        "vorige maand",
        "deze maand",
        "volgende maand",
    ),
    quarter: u(
        p2("{0} kwartaal geleden", "{0} kwartalen geleden"),
        p2("over {0} kwartaal", "over {0} kwartalen"),
        p2("{0} kwartaal", "{0} kwartalen"),
        "vorig kwartaal",
        "dit kwartaal",
        "volgend kwartaal",
    ),
    year: u(
        p1("{0} jaar geleden"),
        p1("over {0} jaar"),
        p1("{0} jaar"),
        "vorig jaar",
        "dit jaar",
        "volgend jaar",
    ),
};

const NL: LocaleData = LocaleData {
    tag: "nl",
    long: NL_LONG,
    short: StyleData::EMPTY,
    narrow: StyleData::EMPTY,
    compact: UnitStrings {
        second: "s",
        minute: "min",
        hour: "u",
        day: "d",
        week: "w",
        month: "mnd",
        quarter: "kw",
        year: "j",
    },
    indefinite: strings(
        "een seconde",
        "een minuut",
        "een uur",
        "een dag",
        "een week",
        "een maand",
        "een kwartaal",
        "een jaar",
    ),
    list: lists("{0}, {1}", "{0} en {1}", "{0} {1}"),
    approximate: ApproximatePatterns {
        exactly: "{0}",
        about: "ongeveer {0}",
        just_over: "iets meer dan {0}",
        over: "meer dan {0}",
        nearly: "bijna {0}",
        less_than: "minder dan {0}",
        more_than: "meer dan {0}",
    },
    weekday: WeekdayPatterns {
        previous: "afgelopen {0}",
        current: "deze {0}",
        next: "komende {0}",
    },
    decimal_separator: ",",
    at_pattern: "{0} om {1}",
};

// --- Polish ---------------------------------------------------------------
//
// Polish differs from Russian in exactly the place a naive implementation
// gets wrong: 22 is `few` in both, but 12 is `many` in both while 2 is `few`
// and 5 is `many`, and Polish puts `i = 1, v = 0` alone in `one` where
// Russian admits 21, 31, 101. Both are in the tests.

const PL_LONG: StyleData = StyleData {
    second: u(
        p4(
            "{0} sekundę temu",
            "{0} sekundy temu",
            "{0} sekund temu",
            "{0} sekundy temu",
        ),
        p4(
            "za {0} sekundę",
            "za {0} sekundy",
            "za {0} sekund",
            "za {0} sekundy",
        ),
        p4("{0} sekunda", "{0} sekundy", "{0} sekund", "{0} sekundy"),
        "",
        "teraz",
        "",
    ),
    minute: u(
        p4(
            "{0} minutę temu",
            "{0} minuty temu",
            "{0} minut temu",
            "{0} minuty temu",
        ),
        p4(
            "za {0} minutę",
            "za {0} minuty",
            "za {0} minut",
            "za {0} minuty",
        ),
        p4("{0} minuta", "{0} minuty", "{0} minut", "{0} minuty"),
        "",
        "ta minuta",
        "",
    ),
    hour: u_half(
        u(
            p4(
                "{0} godzinę temu",
                "{0} godziny temu",
                "{0} godzin temu",
                "{0} godziny temu",
            ),
            p4(
                "za {0} godzinę",
                "za {0} godziny",
                "za {0} godzin",
                "za {0} godziny",
            ),
            p4("{0} godzina", "{0} godziny", "{0} godzin", "{0} godziny"),
            "",
            "ta godzina",
            "",
        ),
        "pół godziny",
        "półtorej godziny",
    ),
    day: u_day(
        p4(
            "{0} dzień temu",
            "{0} dni temu",
            "{0} dni temu",
            "{0} dnia temu",
        ),
        p4("za {0} dzień", "za {0} dni", "za {0} dni", "za {0} dnia"),
        p4("{0} dzień", "{0} dni", "{0} dni", "{0} dnia"),
        "przedwczoraj",
        "wczoraj",
        "dzisiaj",
        "jutro",
        "pojutrze",
    ),
    week: u(
        p4(
            "{0} tydzień temu",
            "{0} tygodnie temu",
            "{0} tygodni temu",
            "{0} tygodnia temu",
        ),
        p4(
            "za {0} tydzień",
            "za {0} tygodnie",
            "za {0} tygodni",
            "za {0} tygodnia",
        ),
        p4("{0} tydzień", "{0} tygodnie", "{0} tygodni", "{0} tygodnia"),
        "w zeszłym tygodniu",
        "w tym tygodniu",
        "w przyszłym tygodniu",
    ),
    month: u(
        p4(
            "{0} miesiąc temu",
            "{0} miesiące temu",
            "{0} miesięcy temu",
            "{0} miesiąca temu",
        ),
        p4(
            "za {0} miesiąc",
            "za {0} miesiące",
            "za {0} miesięcy",
            "za {0} miesiąca",
        ),
        p4(
            "{0} miesiąc",
            "{0} miesiące",
            "{0} miesięcy",
            "{0} miesiąca",
        ),
        "w zeszłym miesiącu",
        "w tym miesiącu",
        "w przyszłym miesiącu",
    ),
    quarter: u(
        p4(
            "{0} kwartał temu",
            "{0} kwartały temu",
            "{0} kwartałów temu",
            "{0} kwartału temu",
        ),
        p4(
            "za {0} kwartał",
            "za {0} kwartały",
            "za {0} kwartałów",
            "za {0} kwartału",
        ),
        p4(
            "{0} kwartał",
            "{0} kwartały",
            "{0} kwartałów",
            "{0} kwartału",
        ),
        "zeszły kwartał",
        "ten kwartał",
        "przyszły kwartał",
    ),
    year: u(
        p4(
            "{0} rok temu",
            "{0} lata temu",
            "{0} lat temu",
            "{0} roku temu",
        ),
        p4("za {0} rok", "za {0} lata", "za {0} lat", "za {0} roku"),
        p4("{0} rok", "{0} lata", "{0} lat", "{0} roku"),
        "w zeszłym roku",
        "w tym roku",
        "w przyszłym roku",
    ),
};

const PL: LocaleData = LocaleData {
    tag: "pl",
    long: PL_LONG,
    short: StyleData::EMPTY,
    narrow: StyleData::EMPTY,
    compact: UnitStrings {
        second: "s",
        minute: "min",
        hour: "g",
        day: "d",
        week: "tyg",
        month: "mies",
        quarter: "kw",
        year: "l",
    },
    indefinite: UnitStrings::EMPTY,
    list: lists("{0}, {1}", "{0} i {1}", "{0} {1}"),
    approximate: ApproximatePatterns {
        exactly: "{0}",
        about: "około {0}",
        just_over: "nieco ponad {0}",
        over: "ponad {0}",
        nearly: "prawie {0}",
        less_than: "mniej niż {0}",
        more_than: "ponad {0}",
    },
    weekday: WeekdayPatterns {
        previous: "{0} w zeszłym tygodniu",
        current: "{0} w tym tygodniu",
        next: "{0} w przyszłym tygodniu",
    },
    decimal_separator: ",",
    at_pattern: "{0} o {1}",
};

// --- Portuguese -----------------------------------------------------------

const PT_LONG: StyleData = StyleData {
    second: u(
        p2("há {0} segundo", "há {0} segundos"),
        p2("em {0} segundo", "em {0} segundos"),
        p2("{0} segundo", "{0} segundos"),
        "",
        "agora",
        "",
    ),
    minute: u(
        p2("há {0} minuto", "há {0} minutos"),
        p2("em {0} minuto", "em {0} minutos"),
        p2("{0} minuto", "{0} minutos"),
        "",
        "este minuto",
        "",
    ),
    hour: u_half(
        u(
            p2("há {0} hora", "há {0} horas"),
            p2("em {0} hora", "em {0} horas"),
            p2("{0} hora", "{0} horas"),
            "",
            "esta hora",
            "",
        ),
        "meia hora",
        "uma hora e meia",
    ),
    day: u_day(
        p2("há {0} dia", "há {0} dias"),
        p2("em {0} dia", "em {0} dias"),
        p2("{0} dia", "{0} dias"),
        "anteontem",
        "ontem",
        "hoje",
        "amanhã",
        "depois de amanhã",
    ),
    week: u(
        p2("há {0} semana", "há {0} semanas"),
        p2("em {0} semana", "em {0} semanas"),
        p2("{0} semana", "{0} semanas"),
        "semana passada",
        "esta semana",
        "próxima semana",
    ),
    month: u(
        p2("há {0} mês", "há {0} meses"),
        p2("em {0} mês", "em {0} meses"),
        p2("{0} mês", "{0} meses"),
        "mês passado",
        "este mês",
        "próximo mês",
    ),
    quarter: u(
        p2("há {0} trimestre", "há {0} trimestres"),
        p2("em {0} trimestre", "em {0} trimestres"),
        p2("{0} trimestre", "{0} trimestres"),
        "trimestre passado",
        "este trimestre",
        "próximo trimestre",
    ),
    year: u(
        p2("há {0} ano", "há {0} anos"),
        p2("em {0} ano", "em {0} anos"),
        p2("{0} ano", "{0} anos"),
        "ano passado",
        "este ano",
        "próximo ano",
    ),
};

const PT: LocaleData = LocaleData {
    tag: "pt",
    long: PT_LONG,
    short: StyleData::EMPTY,
    narrow: StyleData::EMPTY,
    compact: UnitStrings {
        second: "s",
        minute: "min",
        hour: "h",
        day: "d",
        week: "sem",
        month: "m",
        quarter: "trim",
        year: "a",
    },
    indefinite: strings(
        "um segundo",
        "um minuto",
        "uma hora",
        "um dia",
        "uma semana",
        "um mês",
        "um trimestre",
        "um ano",
    ),
    list: lists("{0}, {1}", "{0} e {1}", "{0} {1}"),
    approximate: ApproximatePatterns {
        exactly: "{0}",
        about: "cerca de {0}",
        just_over: "pouco mais de {0}",
        over: "mais de {0}",
        nearly: "quase {0}",
        less_than: "menos de {0}",
        more_than: "mais de {0}",
    },
    weekday: WeekdayPatterns {
        previous: "{0} da semana passada",
        current: "{0} desta semana",
        next: "{0} da próxima semana",
    },
    decimal_separator: ",",
    at_pattern: "{0} às {1}",
};

// --- Russian --------------------------------------------------------------
//
// The canonical demonstration: 1 день, 2 дня, 5 дней, 21 день, 22 дня,
// 25 дней. The `other` column is reached only by decimals, which is how a
// half hour comes out as *1,5 часа*.

const RU_LONG: StyleData = StyleData {
    second: u(
        p4(
            "{0} секунду назад",
            "{0} секунды назад",
            "{0} секунд назад",
            "{0} секунды назад",
        ),
        p4(
            "через {0} секунду",
            "через {0} секунды",
            "через {0} секунд",
            "через {0} секунды",
        ),
        p4("{0} секунда", "{0} секунды", "{0} секунд", "{0} секунды"),
        "",
        "сейчас",
        "",
    ),
    minute: u(
        p4(
            "{0} минуту назад",
            "{0} минуты назад",
            "{0} минут назад",
            "{0} минуты назад",
        ),
        p4(
            "через {0} минуту",
            "через {0} минуты",
            "через {0} минут",
            "через {0} минуты",
        ),
        p4("{0} минута", "{0} минуты", "{0} минут", "{0} минуты"),
        "",
        "в эту минуту",
        "",
    ),
    hour: u_half(
        u(
            p4(
                "{0} час назад",
                "{0} часа назад",
                "{0} часов назад",
                "{0} часа назад",
            ),
            p4(
                "через {0} час",
                "через {0} часа",
                "через {0} часов",
                "через {0} часа",
            ),
            p4("{0} час", "{0} часа", "{0} часов", "{0} часа"),
            "",
            "в этот час",
            "",
        ),
        "полчаса",
        "полтора часа",
    ),
    day: u_day(
        p4(
            "{0} день назад",
            "{0} дня назад",
            "{0} дней назад",
            "{0} дня назад",
        ),
        p4(
            "через {0} день",
            "через {0} дня",
            "через {0} дней",
            "через {0} дня",
        ),
        p4("{0} день", "{0} дня", "{0} дней", "{0} дня"),
        "позавчера",
        "вчера",
        "сегодня",
        "завтра",
        "послезавтра",
    ),
    week: u(
        p4(
            "{0} неделю назад",
            "{0} недели назад",
            "{0} недель назад",
            "{0} недели назад",
        ),
        p4(
            "через {0} неделю",
            "через {0} недели",
            "через {0} недель",
            "через {0} недели",
        ),
        p4("{0} неделя", "{0} недели", "{0} недель", "{0} недели"),
        "на прошлой неделе",
        "на этой неделе",
        "на следующей неделе",
    ),
    month: u(
        p4(
            "{0} месяц назад",
            "{0} месяца назад",
            "{0} месяцев назад",
            "{0} месяца назад",
        ),
        p4(
            "через {0} месяц",
            "через {0} месяца",
            "через {0} месяцев",
            "через {0} месяца",
        ),
        p4("{0} месяц", "{0} месяца", "{0} месяцев", "{0} месяца"),
        "в прошлом месяце",
        "в этом месяце",
        "в следующем месяце",
    ),
    quarter: u(
        p4(
            "{0} квартал назад",
            "{0} квартала назад",
            "{0} кварталов назад",
            "{0} квартала назад",
        ),
        p4(
            "через {0} квартал",
            "через {0} квартала",
            "через {0} кварталов",
            "через {0} квартала",
        ),
        p4(
            "{0} квартал",
            "{0} квартала",
            "{0} кварталов",
            "{0} квартала",
        ),
        "в прошлом квартале",
        "в текущем квартале",
        "в следующем квартале",
    ),
    year: u(
        p4(
            "{0} год назад",
            "{0} года назад",
            "{0} лет назад",
            "{0} года назад",
        ),
        p4(
            "через {0} год",
            "через {0} года",
            "через {0} лет",
            "через {0} года",
        ),
        p4("{0} год", "{0} года", "{0} лет", "{0} года"),
        "в прошлом году",
        "в этом году",
        "в следующем году",
    ),
};

const RU_SHORT: StyleData = StyleData {
    second: u(
        p1("{0} с назад"),
        p1("через {0} с"),
        p1("{0} с"),
        "",
        "сейчас",
        "",
    ),
    minute: u(
        p1("{0} мин. назад"),
        p1("через {0} мин."),
        p1("{0} мин."),
        "",
        "в эту минуту",
        "",
    ),
    hour: u(
        p1("{0} ч назад"),
        p1("через {0} ч"),
        p1("{0} ч"),
        "",
        "в этот час",
        "",
    ),
    day: u_day(
        p1("{0} дн. назад"),
        p1("через {0} дн."),
        p1("{0} дн."),
        "позавчера",
        "вчера",
        "сегодня",
        "завтра",
        "послезавтра",
    ),
    week: u(
        p1("{0} нед. назад"),
        p1("через {0} нед."),
        p1("{0} нед."),
        "на прошлой неделе",
        "на этой неделе",
        "на следующей неделе",
    ),
    month: u(
        p1("{0} мес. назад"),
        p1("через {0} мес."),
        p1("{0} мес."),
        "в прошлом месяце",
        "в этом месяце",
        "в следующем месяце",
    ),
    quarter: u(
        p1("{0} кв. назад"),
        p1("через {0} кв."),
        p1("{0} кв."),
        "в прошлом квартале",
        "в текущем квартале",
        "в следующем квартале",
    ),
    year: u(
        p1("{0} г. назад"),
        p1("через {0} г."),
        p1("{0} г."),
        "в прошлом году",
        "в этом году",
        "в следующем году",
    ),
};

const RU: LocaleData = LocaleData {
    tag: "ru",
    long: RU_LONG,
    short: RU_SHORT,
    narrow: StyleData::EMPTY,
    compact: UnitStrings {
        second: "с",
        minute: "мин",
        hour: "ч",
        day: "д",
        week: "нед",
        month: "мес",
        quarter: "кв",
        year: "г",
    },
    indefinite: UnitStrings::EMPTY,
    list: lists("{0}, {1}", "{0} и {1}", "{0} {1}"),
    approximate: ApproximatePatterns {
        exactly: "{0}",
        about: "приблизительно {0}",
        just_over: "{0} с небольшим",
        over: "{0} с лишним",
        nearly: "почти {0}",
        less_than: "{0} и менее",
        more_than: "{0} и более",
    },
    weekday: WeekdayPatterns {
        previous: "{0} на прошлой неделе",
        current: "{0} на этой неделе",
        next: "{0} на следующей неделе",
    },
    decimal_separator: ",",
    at_pattern: "{0}, {1}",
};

// --- Thai -----------------------------------------------------------------

const TH_LONG: StyleData = StyleData {
    second: u(
        p1("{0} วินาทีที่ผ่านมา"),
        p1("ในอีก {0} วินาที"),
        p1("{0} วินาที"),
        "",
        "ขณะนี้",
        "",
    ),
    minute: u(
        p1("{0} นาทีที่ผ่านมา"),
        p1("ในอีก {0} นาที"),
        p1("{0} นาที"),
        "",
        "นาทีนี้",
        "",
    ),
    hour: u_half(
        u(
            p1("{0} ชั่วโมงที่ผ่านมา"),
            p1("ในอีก {0} ชั่วโมง"),
            p1("{0} ชั่วโมง"),
            "",
            "ชั่วโมงนี้",
            "",
        ),
        "ครึ่งชั่วโมง",
        "หนึ่งชั่วโมงครึ่ง",
    ),
    day: u_day(
        p1("{0} วันที่ผ่านมา"),
        p1("ในอีก {0} วัน"),
        p1("{0} วัน"),
        "เมื่อวานซืน",
        "เมื่อวาน",
        "วันนี้",
        "พรุ่งนี้",
        "มะรืนนี้",
    ),
    week: u(
        p1("{0} สัปดาห์ที่ผ่านมา"),
        p1("ในอีก {0} สัปดาห์"),
        p1("{0} สัปดาห์"),
        "สัปดาห์ที่แล้ว",
        "สัปดาห์นี้",
        "สัปดาห์หน้า",
    ),
    month: u(
        p1("{0} เดือนที่ผ่านมา"),
        p1("ในอีก {0} เดือน"),
        p1("{0} เดือน"),
        "เดือนที่แล้ว",
        "เดือนนี้",
        "เดือนหน้า",
    ),
    quarter: u(
        p1("{0} ไตรมาสที่ผ่านมา"),
        p1("ในอีก {0} ไตรมาส"),
        p1("{0} ไตรมาส"),
        "ไตรมาสที่แล้ว",
        "ไตรมาสนี้",
        "ไตรมาสหน้า",
    ),
    year: u(
        p1("{0} ปีที่ผ่านมา"),
        p1("ในอีก {0} ปี"),
        p1("{0} ปี"),
        "ปีที่แล้ว",
        "ปีนี้",
        "ปีหน้า",
    ),
};

const TH: LocaleData = LocaleData {
    tag: "th",
    long: TH_LONG,
    short: StyleData::EMPTY,
    narrow: StyleData::EMPTY,
    compact: UnitStrings::EMPTY,
    indefinite: UnitStrings::EMPTY,
    list: ListPatterns {
        standard: list_conjunction("{0} {1}", "{0} และ{1}"),
        unit: list_uniform("{0} {1}"),
        narrow: list_uniform("{0} {1}"),
    },
    approximate: ApproximatePatterns {
        exactly: "{0}",
        about: "ประมาณ {0}",
        just_over: "มากกว่า {0} เล็กน้อย",
        over: "มากกว่า {0}",
        nearly: "เกือบ {0}",
        less_than: "น้อยกว่า {0}",
        more_than: "มากกว่า {0}",
    },
    weekday: WeekdayPatterns {
        previous: "{0} ที่แล้ว",
        current: "{0} นี้",
        next: "{0} หน้า",
    },
    decimal_separator: ".",
    at_pattern: "{0} เวลา {1}",
};

// --- Turkish --------------------------------------------------------------
//
// The comparative hedges are phrased so that no case suffix lands on `{0}`:
// Turkish would want an ablative there (*3 saatten fazla*) and the suffix
// changes with vowel harmony and with the final consonant, which a static
// pattern cannot produce. *{0} ve üzeri* and *en fazla {0}* are grammatical
// for every filling.

const TR_LONG: StyleData = StyleData {
    second: u(
        p1("{0} saniye önce"),
        p1("{0} saniye sonra"),
        p1("{0} saniye"),
        "",
        "şimdi",
        "",
    ),
    minute: u(
        p1("{0} dakika önce"),
        p1("{0} dakika sonra"),
        p1("{0} dakika"),
        "",
        "bu dakika",
        "",
    ),
    hour: u_half(
        u(
            p1("{0} saat önce"),
            p1("{0} saat sonra"),
            p1("{0} saat"),
            "",
            "bu saat",
            "",
        ),
        "yarım saat",
        "bir buçuk saat",
    ),
    day: u_day(
        p1("{0} gün önce"),
        p1("{0} gün sonra"),
        p1("{0} gün"),
        "evvelsi gün",
        "dün",
        "bugün",
        "yarın",
        "öbür gün",
    ),
    week: u(
        p1("{0} hafta önce"),
        p1("{0} hafta sonra"),
        p1("{0} hafta"),
        "geçen hafta",
        "bu hafta",
        "gelecek hafta",
    ),
    month: u(
        p1("{0} ay önce"),
        p1("{0} ay sonra"),
        p1("{0} ay"),
        "geçen ay",
        "bu ay",
        "gelecek ay",
    ),
    quarter: u(
        p1("{0} çeyrek önce"),
        p1("{0} çeyrek sonra"),
        p1("{0} çeyrek"),
        "geçen çeyrek",
        "bu çeyrek",
        "gelecek çeyrek",
    ),
    year: u(
        p1("{0} yıl önce"),
        p1("{0} yıl sonra"),
        p1("{0} yıl"),
        "geçen yıl",
        "bu yıl",
        "gelecek yıl",
    ),
};

const TR: LocaleData = LocaleData {
    tag: "tr",
    long: TR_LONG,
    short: StyleData::EMPTY,
    narrow: StyleData::EMPTY,
    compact: UnitStrings {
        second: "sn",
        minute: "dk",
        hour: "sa",
        day: "g",
        week: "hf",
        month: "ay",
        quarter: "çyr",
        year: "y",
    },
    indefinite: strings(
        "bir saniye",
        "bir dakika",
        "bir saat",
        "bir gün",
        "bir hafta",
        "bir ay",
        "bir çeyrek",
        "bir yıl",
    ),
    list: lists("{0}, {1}", "{0} ve {1}", "{0} {1}"),
    approximate: ApproximatePatterns {
        exactly: "{0}",
        about: "yaklaşık {0}",
        just_over: "{0} ve biraz fazlası",
        over: "{0} ve üzeri",
        nearly: "neredeyse {0}",
        less_than: "en fazla {0}",
        more_than: "{0} ve üzeri",
    },
    weekday: WeekdayPatterns {
        previous: "geçen {0}",
        current: "bu {0}",
        next: "gelecek {0}",
    },
    decimal_separator: ",",
    at_pattern: "{0} {1}",
};

// --- Vietnamese -----------------------------------------------------------

const VI_LONG: StyleData = StyleData {
    second: u(
        p1("{0} giây trước"),
        p1("sau {0} giây nữa"),
        p1("{0} giây"),
        "",
        "bây giờ",
        "",
    ),
    minute: u(
        p1("{0} phút trước"),
        p1("sau {0} phút nữa"),
        p1("{0} phút"),
        "",
        "phút này",
        "",
    ),
    hour: u_half(
        u(
            p1("{0} giờ trước"),
            p1("sau {0} giờ nữa"),
            p1("{0} giờ"),
            "",
            "giờ này",
            "",
        ),
        "nửa giờ",
        "một giờ rưỡi",
    ),
    day: u_day(
        p1("{0} ngày trước"),
        p1("sau {0} ngày nữa"),
        p1("{0} ngày"),
        "hôm kia",
        "hôm qua",
        "hôm nay",
        "ngày mai",
        "ngày kia",
    ),
    week: u(
        p1("{0} tuần trước"),
        p1("sau {0} tuần nữa"),
        p1("{0} tuần"),
        "tuần trước",
        "tuần này",
        "tuần sau",
    ),
    month: u(
        p1("{0} tháng trước"),
        p1("sau {0} tháng nữa"),
        p1("{0} tháng"),
        "tháng trước",
        "tháng này",
        "tháng sau",
    ),
    quarter: u(
        p1("{0} quý trước"),
        p1("sau {0} quý nữa"),
        p1("{0} quý"),
        "quý trước",
        "quý này",
        "quý sau",
    ),
    year: u(
        p1("{0} năm trước"),
        p1("sau {0} năm nữa"),
        p1("{0} năm"),
        "năm ngoái",
        "năm nay",
        "năm sau",
    ),
};

const VI: LocaleData = LocaleData {
    tag: "vi",
    long: VI_LONG,
    short: StyleData::EMPTY,
    narrow: StyleData::EMPTY,
    compact: UnitStrings {
        second: "s",
        minute: "p",
        hour: "h",
        day: "ngày",
        week: "tuần",
        month: "tháng",
        quarter: "quý",
        year: "năm",
    },
    indefinite: UnitStrings::EMPTY,
    list: lists("{0}, {1}", "{0} và {1}", "{0} {1}"),
    approximate: ApproximatePatterns {
        exactly: "{0}",
        about: "khoảng {0}",
        just_over: "hơn {0} một chút",
        over: "hơn {0}",
        nearly: "gần {0}",
        less_than: "chưa đến {0}",
        more_than: "hơn {0}",
    },
    weekday: WeekdayPatterns {
        previous: "{0} tuần trước",
        current: "{0} tuần này",
        next: "{0} tuần sau",
    },
    decimal_separator: ",",
    at_pattern: "{0} lúc {1}",
};

// --- Welsh ----------------------------------------------------------------
//
// Welsh is the only language in CLDR that uses all six categories, and its
// boundaries are nothing like Arabic's: `zero` for 0, `one` for 1, `two` for
// 2, `few` for 3 and `many` for 6 alone, with everything else `other`. Three
// and six are singled out because *tri* and *chwe* mutate the following
// noun where other numerals do not. The forms below follow CLDR's `cy`
// entry; Welsh is carried here because a six-category language whose
// categories are *not* Arabic's is the only way to test that the selection
// really goes through the plural rules.

const CY_LONG: StyleData = StyleData {
    second: u(
        p6(
            "{0} eiliad yn ôl",
            "{0} eiliad yn ôl",
            "{0} eiliad yn ôl",
            "{0} eiliad yn ôl",
            "{0} eiliad yn ôl",
            "{0} o eiliadau yn ôl",
        ),
        p6(
            "ymhen {0} eiliad",
            "ymhen {0} eiliad",
            "ymhen {0} eiliad",
            "ymhen {0} eiliad",
            "ymhen {0} eiliad",
            "ymhen {0} o eiliadau",
        ),
        p6(
            "{0} eiliad",
            "{0} eiliad",
            "{0} eiliad",
            "{0} eiliad",
            "{0} eiliad",
            "{0} o eiliadau",
        ),
        "",
        "nawr",
        "",
    ),
    minute: u(
        p6(
            "{0} munud yn ôl",
            "{0} funud yn ôl",
            "{0} funud yn ôl",
            "{0} munud yn ôl",
            "{0} munud yn ôl",
            "{0} o funudau yn ôl",
        ),
        p6(
            "ymhen {0} munud",
            "ymhen {0} funud",
            "ymhen {0} funud",
            "ymhen {0} munud",
            "ymhen {0} munud",
            "ymhen {0} o funudau",
        ),
        p6(
            "{0} munud",
            "{0} funud",
            "{0} funud",
            "{0} munud",
            "{0} munud",
            "{0} o funudau",
        ),
        "",
        "y funud hon",
        "",
    ),
    hour: u_half(
        u(
            p6(
                "{0} awr yn ôl",
                "{0} awr yn ôl",
                "{0} awr yn ôl",
                "{0} awr yn ôl",
                "{0} awr yn ôl",
                "{0} o oriau yn ôl",
            ),
            p6(
                "ymhen {0} awr",
                "ymhen {0} awr",
                "ymhen {0} awr",
                "ymhen {0} awr",
                "ymhen {0} awr",
                "ymhen {0} o oriau",
            ),
            p6(
                "{0} awr",
                "{0} awr",
                "{0} awr",
                "{0} awr",
                "{0} awr",
                "{0} o oriau",
            ),
            "",
            "yr awr hon",
            "",
        ),
        "hanner awr",
        "awr a hanner",
    ),
    day: u_day(
        p6(
            "{0} diwrnod yn ôl",
            "{0} diwrnod yn ôl",
            "{0} ddiwrnod yn ôl",
            "{0} diwrnod yn ôl",
            "{0} diwrnod yn ôl",
            "{0} o ddiwrnodau yn ôl",
        ),
        p6(
            "ymhen {0} diwrnod",
            "ymhen {0} diwrnod",
            "ymhen {0} ddiwrnod",
            "ymhen {0} diwrnod",
            "ymhen {0} diwrnod",
            "ymhen {0} o ddiwrnodau",
        ),
        p6(
            "{0} diwrnod",
            "{0} diwrnod",
            "{0} ddiwrnod",
            "{0} diwrnod",
            "{0} diwrnod",
            "{0} o ddiwrnodau",
        ),
        "echdoe",
        "ddoe",
        "heddiw",
        "yfory",
        "drennydd",
    ),
    week: u(
        p6(
            "{0} wythnos yn ôl",
            "{0} wythnos yn ôl",
            "{0} wythnos yn ôl",
            "{0} wythnos yn ôl",
            "{0} wythnos yn ôl",
            "{0} o wythnosau yn ôl",
        ),
        p6(
            "ymhen {0} wythnos",
            "ymhen {0} wythnos",
            "ymhen {0} wythnos",
            "ymhen {0} wythnos",
            "ymhen {0} wythnos",
            "ymhen {0} o wythnosau",
        ),
        p6(
            "{0} wythnos",
            "{0} wythnos",
            "{0} wythnos",
            "{0} wythnos",
            "{0} wythnos",
            "{0} o wythnosau",
        ),
        "wythnos ddiwethaf",
        "yr wythnos hon",
        "wythnos nesaf",
    ),
    month: u(
        p6(
            "{0} mis yn ôl",
            "{0} mis yn ôl",
            "{0} fis yn ôl",
            "{0} mis yn ôl",
            "{0} mis yn ôl",
            "{0} o fisoedd yn ôl",
        ),
        p6(
            "ymhen {0} mis",
            "ymhen {0} mis",
            "ymhen {0} fis",
            "ymhen {0} mis",
            "ymhen {0} mis",
            "ymhen {0} o fisoedd",
        ),
        p6(
            "{0} mis",
            "{0} mis",
            "{0} fis",
            "{0} mis",
            "{0} mis",
            "{0} o fisoedd",
        ),
        "mis diwethaf",
        "y mis hwn",
        "mis nesaf",
    ),
    quarter: u(
        p6(
            "{0} chwarter yn ôl",
            "{0} chwarter yn ôl",
            "{0} chwarter yn ôl",
            "{0} chwarter yn ôl",
            "{0} chwarter yn ôl",
            "{0} o chwarteri yn ôl",
        ),
        p6(
            "ymhen {0} chwarter",
            "ymhen {0} chwarter",
            "ymhen {0} chwarter",
            "ymhen {0} chwarter",
            "ymhen {0} chwarter",
            "ymhen {0} o chwarteri",
        ),
        p6(
            "{0} chwarter",
            "{0} chwarter",
            "{0} chwarter",
            "{0} chwarter",
            "{0} chwarter",
            "{0} o chwarteri",
        ),
        "chwarter diwethaf",
        "y chwarter hwn",
        "chwarter nesaf",
    ),
    year: u(
        p6(
            "{0} mlynedd yn ôl",
            "{0} flwyddyn yn ôl",
            "{0} flynedd yn ôl",
            "{0} blynedd yn ôl",
            "{0} blynedd yn ôl",
            "{0} o flynyddoedd yn ôl",
        ),
        p6(
            "ymhen {0} mlynedd",
            "ymhen {0} flwyddyn",
            "ymhen {0} flynedd",
            "ymhen {0} blynedd",
            "ymhen {0} blynedd",
            "ymhen {0} o flynyddoedd",
        ),
        p6(
            "{0} mlynedd",
            "{0} flwyddyn",
            "{0} flynedd",
            "{0} blynedd",
            "{0} blynedd",
            "{0} o flynyddoedd",
        ),
        "llynedd",
        "eleni",
        "y flwyddyn nesaf",
    ),
};

const CY: LocaleData = LocaleData {
    tag: "cy",
    long: CY_LONG,
    short: StyleData::EMPTY,
    narrow: StyleData::EMPTY,
    compact: UnitStrings {
        second: "e",
        minute: "mun",
        hour: "awr",
        day: "d",
        week: "wth",
        month: "mis",
        quarter: "chw",
        year: "bl",
    },
    indefinite: UnitStrings::EMPTY,
    list: lists("{0}, {1}", "{0} a {1}", "{0} {1}"),
    approximate: ApproximatePatterns {
        exactly: "{0}",
        about: "tua {0}",
        just_over: "ychydig dros {0}",
        over: "dros {0}",
        nearly: "bron {0}",
        less_than: "llai na {0}",
        more_than: "mwy na {0}",
    },
    weekday: WeekdayPatterns {
        previous: "{0} diwethaf",
        current: "{0} hwn",
        next: "{0} nesaf",
    },
    decimal_separator: ".",
    at_pattern: "{0} am {1}",
};

// --- Chinese, Simplified --------------------------------------------------
//
// Tagged `zh` rather than `zh-Hans` so that `zh`, `zh-Hans` and `zh-CN` all
// reach it by truncation; `zh-Hant` below is the separate entry.

const ZH_LONG: StyleData = StyleData {
    second: u(p1("{0}秒前"), p1("{0}秒后"), p1("{0}秒"), "", "现在", ""),
    minute: u(
        p1("{0}分钟前"),
        p1("{0}分钟后"),
        p1("{0}分钟"),
        "",
        "此刻",
        "",
    ),
    hour: u_half(
        u(
            p1("{0}小时前"),
            p1("{0}小时后"),
            p1("{0}小时"),
            "",
            "这一小时",
            "",
        ),
        "半小时",
        "一个半小时",
    ),
    day: u_day(
        p1("{0}天前"),
        p1("{0}天后"),
        p1("{0}天"),
        "前天",
        "昨天",
        "今天",
        "明天",
        "后天",
    ),
    week: u(
        p1("{0}周前"),
        p1("{0}周后"),
        p1("{0}周"),
        "上周",
        "本周",
        "下周",
    ),
    month: u(
        p1("{0}个月前"),
        p1("{0}个月后"),
        p1("{0}个月"),
        "上个月",
        "本月",
        "下个月",
    ),
    quarter: u(
        p1("{0}个季度前"),
        p1("{0}个季度后"),
        p1("{0}个季度"),
        "上季度",
        "本季度",
        "下季度",
    ),
    year: u(
        p1("{0}年前"),
        p1("{0}年后"),
        p1("{0}年"),
        "去年",
        "今年",
        "明年",
    ),
};

const ZH: LocaleData = LocaleData {
    tag: "zh",
    long: ZH_LONG,
    short: StyleData::EMPTY,
    narrow: StyleData::EMPTY,
    compact: UnitStrings {
        second: "秒",
        minute: "分钟",
        hour: "小时",
        day: "天",
        week: "周",
        month: "个月",
        quarter: "个季度",
        year: "年",
    },
    indefinite: UnitStrings::EMPTY,
    list: ListPatterns {
        standard: list_conjunction("{0}、{1}", "{0}和{1}"),
        unit: list_uniform("{0}{1}"),
        narrow: list_uniform("{0}{1}"),
    },
    approximate: ApproximatePatterns {
        exactly: "{0}",
        about: "大约{0}",
        just_over: "{0}多一点",
        over: "{0}多",
        nearly: "将近{0}",
        less_than: "不到{0}",
        more_than: "超过{0}",
    },
    weekday: WeekdayPatterns {
        previous: "上{0}",
        current: "这{0}",
        next: "下{0}",
    },
    decimal_separator: ".",
    at_pattern: "{0}{1}",
};

// --- Chinese, Traditional -------------------------------------------------

const ZH_HANT_LONG: StyleData = StyleData {
    second: u(p1("{0}秒前"), p1("{0}秒後"), p1("{0}秒"), "", "現在", ""),
    minute: u(
        p1("{0}分鐘前"),
        p1("{0}分鐘後"),
        p1("{0}分鐘"),
        "",
        "這一分鐘",
        "",
    ),
    hour: u_half(
        u(
            p1("{0}小時前"),
            p1("{0}小時後"),
            p1("{0}小時"),
            "",
            "這一小時",
            "",
        ),
        "半小時",
        "一個半小時",
    ),
    day: u_day(
        p1("{0}天前"),
        p1("{0}天後"),
        p1("{0}天"),
        "前天",
        "昨天",
        "今天",
        "明天",
        "後天",
    ),
    week: u(
        p1("{0}週前"),
        p1("{0}週後"),
        p1("{0}週"),
        "上週",
        "本週",
        "下週",
    ),
    month: u(
        p1("{0}個月前"),
        p1("{0}個月後"),
        p1("{0}個月"),
        "上個月",
        "本月",
        "下個月",
    ),
    quarter: u(
        p1("{0}個季度前"),
        p1("{0}個季度後"),
        p1("{0}個季度"),
        "上一季",
        "這一季",
        "下一季",
    ),
    year: u(
        p1("{0}年前"),
        p1("{0}年後"),
        p1("{0}年"),
        "去年",
        "今年",
        "明年",
    ),
};

const ZH_HANT: LocaleData = LocaleData {
    tag: "zh-Hant",
    long: ZH_HANT_LONG,
    short: StyleData::EMPTY,
    narrow: StyleData::EMPTY,
    compact: UnitStrings {
        second: "秒",
        minute: "分鐘",
        hour: "小時",
        day: "天",
        week: "週",
        month: "個月",
        quarter: "季",
        year: "年",
    },
    indefinite: UnitStrings::EMPTY,
    list: ListPatterns {
        standard: list_conjunction("{0}、{1}", "{0}和{1}"),
        unit: list_uniform("{0}{1}"),
        narrow: list_uniform("{0}{1}"),
    },
    approximate: ApproximatePatterns {
        exactly: "{0}",
        about: "大約{0}",
        just_over: "{0}多一點",
        over: "{0}多",
        nearly: "將近{0}",
        less_than: "不到{0}",
        more_than: "超過{0}",
    },
    weekday: WeekdayPatterns {
        previous: "上{0}",
        current: "這{0}",
        next: "下{0}",
    },
    decimal_separator: ".",
    at_pattern: "{0}{1}",
};

/// Every locale this crate carries, in tag order.
///
/// Adding a language is one line here plus one `const` above.
pub static LOCALES: &[LocaleData] = &[
    AR, CS, CY, DE, EN, ES, FR, HI, ID, IT, JA, KO, NL, PL, PT, RU, TH, TR, VI, ZH, ZH_HANT,
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pattern::RelativeStyle;
    use crate::unit::TimeUnit;
    use alloc::string::ToString as _;
    use alloc::vec::Vec;
    use hc_i18n::{Locale, PluralCategory, PluralRules};

    fn every_entry() -> Vec<&'static LocaleData> {
        let mut entries: Vec<&'static LocaleData> = LOCALES.iter().collect();
        entries.push(&ROOT);
        entries
    }

    fn every_unit_patterns(
        data: &'static LocaleData,
    ) -> Vec<(&'static str, RelativeStyle, TimeUnit, &'static UnitPatterns)> {
        let mut found = Vec::new();
        for style in RelativeStyle::ALL {
            for unit in TimeUnit::ALL {
                found.push((data.tag, style, unit, style.within(data).get(unit)));
            }
        }
        found
    }

    #[test]
    fn the_table_is_sorted_by_tag_and_has_no_duplicates() {
        for pair in LOCALES.windows(2) {
            assert!(
                pair[0].tag < pair[1].tag,
                "{} !< {}",
                pair[0].tag,
                pair[1].tag
            );
        }
    }

    #[test]
    fn every_tag_is_canonical() {
        for data in every_entry() {
            let locale: Locale = data.tag.parse().expect("well-formed tag");
            assert_eq!(locale.to_string(), data.tag);
        }
    }

    #[test]
    fn every_entry_states_every_unit_in_its_long_style() {
        for data in every_entry() {
            for unit in TimeUnit::ALL {
                let patterns = data.long.get(unit);
                assert!(
                    !patterns.past.is_empty(),
                    "{} has no past pattern for {unit}",
                    data.tag
                );
                assert!(
                    !patterns.future.is_empty(),
                    "{} has no future pattern for {unit}",
                    data.tag
                );
                assert!(
                    !patterns.count.is_empty(),
                    "{} has no count pattern for {unit}",
                    data.tag
                );
            }
        }
    }

    #[test]
    fn the_other_category_can_always_take_a_number() {
        // `other` is the catch-all, so it has to carry the placeholder even
        // where a narrower category does not: Arabic's dual يومين names the
        // number in the noun and needs no `{0}`, but its `other` does.
        for data in every_entry() {
            for (tag, style, unit, patterns) in every_unit_patterns(data) {
                for (name, forms) in [
                    ("past", &patterns.past),
                    ("future", &patterns.future),
                    ("count", &patterns.count),
                ] {
                    if forms.is_empty() {
                        continue;
                    }
                    assert!(
                        forms.other.contains("{0}"),
                        "{tag} {style:?} {unit} {name} `other` cannot take a number"
                    );
                }
            }
        }
    }

    #[test]
    fn no_pattern_is_padded_with_whitespace() {
        for data in every_entry() {
            for (tag, style, unit, patterns) in every_unit_patterns(data) {
                for forms in [&patterns.past, &patterns.future, &patterns.count] {
                    for category in PluralCategory::ALL {
                        let text = forms.get(category);
                        assert_eq!(
                            text.trim(),
                            text,
                            "{tag} {style:?} {unit} {category} is padded"
                        );
                    }
                }
                for word in [
                    patterns.previous_2,
                    patterns.previous,
                    patterns.current,
                    patterns.next,
                    patterns.next_2,
                    patterns.half,
                    patterns.one_and_a_half,
                ] {
                    assert_eq!(word.trim(), word, "{tag} {style:?} {unit} word is padded");
                }
            }
        }
    }

    #[test]
    fn a_special_word_never_takes_a_number() {
        for data in every_entry() {
            for (tag, style, unit, patterns) in every_unit_patterns(data) {
                for offset in -2..=2 {
                    assert!(
                        !patterns.special(offset).contains("{0}"),
                        "{tag} {style:?} {unit} offset {offset} carries a placeholder"
                    );
                }
            }
        }
    }

    #[test]
    fn every_list_pattern_can_be_written_left_to_right() {
        // A list is built by appending to a sink that cannot be read back,
        // so every pattern must be exactly `{0}<glue>{1}`.
        for data in every_entry() {
            for (name, forms) in [
                ("standard", &data.list.standard),
                ("unit", &data.list.unit),
                ("narrow", &data.list.narrow),
            ] {
                if forms.is_empty() {
                    continue;
                }
                for pattern in [forms.two, forms.start, forms.middle, forms.end] {
                    assert!(
                        pattern.starts_with("{0}") && pattern.ends_with("{1}"),
                        "{} {name} list pattern {pattern:?} is not {{0}}<glue>{{1}}",
                        data.tag
                    );
                }
            }
        }
    }

    #[test]
    fn every_hedge_and_weekday_pattern_takes_its_argument() {
        for data in every_entry() {
            let hedges = data.approximate;
            for pattern in [
                hedges.exactly,
                hedges.about,
                hedges.just_over,
                hedges.over,
                hedges.nearly,
                hedges.less_than,
                hedges.more_than,
            ] {
                assert!(
                    pattern.contains("{0}"),
                    "{} has a hedge with no argument",
                    data.tag
                );
            }
            for pattern in [
                data.weekday.previous,
                data.weekday.current,
                data.weekday.next,
            ] {
                assert!(
                    pattern.contains("{0}"),
                    "{} has a weekday pattern with no argument",
                    data.tag
                );
            }
            assert!(data.at_pattern.contains("{0}") && data.at_pattern.contains("{1}"));
            assert!(!data.decimal_separator.is_empty());
        }
    }

    #[test]
    fn the_per_unit_string_tables_are_all_or_nothing() {
        // Half a compact table would produce `2h30` with no suffix on the
        // minutes, which reads as a different number.
        for data in every_entry() {
            for (name, strings) in [("compact", data.compact), ("indefinite", data.indefinite)] {
                let stated = TimeUnit::ALL
                    .into_iter()
                    .filter(|unit| !strings.get(*unit).is_empty())
                    .count();
                assert!(
                    stated == 0 || stated == TimeUnit::ALL.len(),
                    "{} states {stated} of 8 {name} strings",
                    data.tag
                );
            }
        }
    }

    #[test]
    fn every_language_in_the_table_has_plural_rules_to_choose_with() {
        // A locale whose language `hc-i18n` does not know would silently get
        // `other` for every number, which is exactly the bug this crate
        // exists to avoid.
        for data in LOCALES {
            let locale: Locale = data.tag.parse().expect("well-formed tag");
            let rules = PluralRules::for_locale(&locale);
            assert_ne!(rules.language(), "und", "{} has no plural rules", data.tag);
        }
    }

    #[test]
    fn the_root_entry_is_language_free() {
        // CLDR's root states `+3 d`, not "in 3 days". An unknown locale that
        // answered in English would be a bug only a speaker of the missing
        // language could see.
        for unit in TimeUnit::ALL {
            let patterns = ROOT.long.get(unit);
            assert!(patterns.past.other.starts_with('-'));
            assert!(patterns.future.other.starts_with('+'));
            for offset in -2..=2 {
                assert_eq!(patterns.special(offset), "");
            }
        }
    }
}
