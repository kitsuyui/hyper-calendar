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
//! follow the `<unit type="duration-…">` section of the same file, with the
//! file's list patterns, decimal separator and long `relative` date-time
//! pattern, which UTS #35 Part 4 gives a relative date joined to a time.
//! They are a subset: CLDR carries roughly 600 locales and this crate
//! carries 50.
//!
//! Every one of the 50 takes that part from its CLDR 48 file, generated:
//! `scripts/humanize-cldr.py` resolves each value as CLDR resolves it and
//! writes `data/cldr48.rs`, applying the documented overrides of
//! `data/cldr48_overrides.tsv`, each a value of CLDR's that a source argues
//! against, with its reason, such as Traditional Chinese's *{0} 刻*, a
//! quarter of an hour, for a quarter of a year. `tests/cldr48_resolved.rs`
//! compares every value an entry takes from CLDR with CLDR 48's own
//! resolution, read from `cldr-json`, and holds that the two differ only
//! where an override says so.
//!
//! Five kinds of string here are **not** from CLDR, because CLDR has no
//! field for them, and are ordinary translations written in this file, so
//! that adding a language stays one entry here and one line of the script:
//!
//! * the approximation hedges (*just over*, *nearly*) used by
//!   [`crate::approximate`](mod@crate::approximate);
//! * the weekday phrases (*last Monday*);
//! * the half-unit idioms (*half an hour*, *anderthalb Stunden*), which are
//!   the long style's;
//! * the compact suffixes of *2h30m*;
//! * the indefinite units (*an hour*).
//!
//! # Deliberate deviations
//!
//! * The `zh` entry carries Simplified Chinese and is tagged `zh` rather
//!   than `zh-Hans`, so `zh-Hans` reaches it by truncation inheritance;
//!   `zh-Hant` is a separate entry. This is a deviation in the tagging
//!   only, not in what a tag resolves to: a tag that names no script first
//!   takes the one CLDR's likely subtags give it
//!   ([`hc_i18n::locale::LIKELY_SCRIPTS`]), so `zh-TW`, `zh-HK` and `zh-MO`
//!   resolve to Traditional (*3 週前*) and `zh` and `zh-CN` to Simplified
//!   (*3周前*), as CLDR's would.
//! * Weekday phrases avoid agreement wherever a language inflects the
//!   demonstrative for gender. Russian says *понедельник на прошлой неделе*
//!   rather than *в прошлый понедельник* because the latter is wrong for
//!   *среда*, and Portuguese and Italian do the same. Gendered agreement
//!   is not yet carried.

mod cldr48;

use cldr48::{
    AM, BN, FA, FIL, HA, HE, JV, ML, MN, MR, MY, NE, PA_GURU, PCM, PS, SW, SYR, TA, TE, UR, UR_IN,
    YUE_HANS, YUE_HANT,
};

use crate::pattern::{
    ApproximatePatterns, ListForms, ListPatterns, LocaleData, PluralForms, StyleData, UnitPatterns,
    UnitStrings, WeekdayPatterns,
};
use crate::unit::TimeUnit;

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

/// Attach the half-unit idioms, which CLDR has no field for, to one unit of
/// a style.
const fn idioms(
    mut style: StyleData,
    unit: TimeUnit,
    half: &'static str,
    one_and_a_half: &'static str,
) -> StyleData {
    let patterns = match unit {
        TimeUnit::Second => &mut style.second,
        TimeUnit::Minute => &mut style.minute,
        TimeUnit::Hour => &mut style.hour,
        TimeUnit::Day => &mut style.day,
        TimeUnit::Week => &mut style.week,
        TimeUnit::Month => &mut style.month,
        TimeUnit::Quarter => &mut style.quarter,
        TimeUnit::Year => &mut style.year,
    };
    patterns.half = half;
    patterns.one_and_a_half = one_and_a_half;
    style
}

/// A style with the half-unit idioms of another style of the same
/// language, unit by unit.
const fn idioms_of(mut style: StyleData, from: StyleData) -> StyleData {
    style.second = unit_idioms_of(style.second, from.second);
    style.minute = unit_idioms_of(style.minute, from.minute);
    style.hour = unit_idioms_of(style.hour, from.hour);
    style.day = unit_idioms_of(style.day, from.day);
    style.week = unit_idioms_of(style.week, from.week);
    style.month = unit_idioms_of(style.month, from.month);
    style.quarter = unit_idioms_of(style.quarter, from.quarter);
    style.year = unit_idioms_of(style.year, from.year);
    style
}

const fn unit_idioms_of(mut patterns: UnitPatterns, from: UnitPatterns) -> UnitPatterns {
    patterns.half = from.half;
    patterns.one_and_a_half = from.one_and_a_half;
    patterns
}

/// A regional entry: its own CLDR part, and the parts CLDR has no field
/// for from its language's entry — the half-unit idioms, the compact
/// suffixes, the indefinite units, the hedges and the weekday phrases —
/// which are the same language's.
const fn regional(cldr: LocaleData, language: &LocaleData) -> LocaleData {
    LocaleData {
        long: idioms_of(cldr.long, language.long),
        compact: language.compact,
        indefinite: language.indefinite,
        approximate: language.approximate,
        weekday: language.weekday,
        ..cldr
    }
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
    approximate: LANGUAGE_FREE_HEDGES,
    weekday: LANGUAGE_FREE_WEEKDAYS,
    decimal_separator: ".",
    at_pattern: "{0}, {1}",
};

/// Root's hedges, which say nothing in any language: `~5`, `>5`, `<5`.
/// The entries whose language no source read gives hedges for take them
/// too, rather than a translation.
const LANGUAGE_FREE_HEDGES: ApproximatePatterns = ApproximatePatterns {
    exactly: "{0}",
    about: "~{0}",
    just_over: ">{0}",
    over: ">{0}",
    nearly: "<{0}",
    less_than: "<{0}",
    more_than: ">{0}",
};

/// Root's weekday phrases, which name the weekday and nothing else.
const LANGUAGE_FREE_WEEKDAYS: WeekdayPatterns = WeekdayPatterns {
    previous: "{0}",
    current: "{0}",
    next: "{0}",
};

// --- Arabic ---------------------------------------------------------------
//
// Arabic is the reason the six-category path exists: ٠ takes `zero`, ١
// `one`, ٢ the dual `two`, ٣–١٠ `few`, ١١–٩٩ `many` and ١٠٠ `other`, and
// each of those six is a different sentence. The `one` and `two` patterns
// carry no `{0}` at all, because the noun's own form already says how many
// — يومين *is* "two days".

const AR: LocaleData = LocaleData {
    long: idioms(cldr48::AR.long, TimeUnit::Hour, "نصف ساعة", "ساعة ونصف"),
    // Left empty on purpose: Arabic has no `2h30m` convention, and mixing
    // the root's Latin suffixes into a right-to-left string would need bidi
    // isolation the compact form does not carry. Use `Narrow` instead.
    compact: UnitStrings::EMPTY,
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
    ..cldr48::AR
};

// --- Czech ----------------------------------------------------------------
//
// Czech puts the past under a preposition that governs the instrumental —
// *před třemi dny* — so its past patterns share no stem with its undirected
// ones: *3 dny* but *před 3 dny*, *3 roky* but *před 3 lety*. Its `many`
// category is the fraction category, so an integer never reaches it.

const CS: LocaleData = LocaleData {
    long: idioms(
        cldr48::CS.long,
        TimeUnit::Hour,
        "půl hodiny",
        "hodina a půl",
    ),
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
    ..cldr48::CS
};

// --- German ---------------------------------------------------------------
//
// German is the counterexample that makes `UnitPatterns::count` a separate
// field rather than a prefix stripped off the past form: the preposition
// *vor* takes the dative, so it is *3 Tage* but *vor 3 Tagen*, *3 Monate*
// but *vor 3 Monaten*, *3 Jahre* but *vor 3 Jahren*. Minutes, hours and
// weeks happen to coincide, which is exactly why the mistake is easy to
// miss.

const DE: LocaleData = LocaleData {
    long: idioms(
        cldr48::DE.long,
        TimeUnit::Hour,
        "eine halbe Stunde",
        "anderthalb Stunden",
    ),
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
    ..cldr48::DE
};

// --- English --------------------------------------------------------------

const EN: LocaleData = LocaleData {
    long: idioms(
        idioms(
            cldr48::EN.long,
            TimeUnit::Hour,
            "half an hour",
            "an hour and a half",
        ),
        TimeUnit::Day,
        "half a day",
        "a day and a half",
    ),
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
    ..cldr48::EN
};

// --- Spanish --------------------------------------------------------------

const ES: LocaleData = LocaleData {
    long: idioms(
        cldr48::ES.long,
        TimeUnit::Hour,
        "media hora",
        "una hora y media",
    ),
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
    ..cldr48::ES
};

// --- French ---------------------------------------------------------------
//
// French puts 0 in the `one` category, so *dans 0 jour* is singular. That is
// the CLDR rule `one: i = 0,1`, not an oversight.

const FR: LocaleData = LocaleData {
    long: idioms(
        cldr48::FR.long,
        TimeUnit::Hour,
        "une demi-heure",
        "une heure et demie",
    ),
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
    ..cldr48::FR
};

// --- Hindi ----------------------------------------------------------------
//
// Hindi has one word, कल, for both yesterday and tomorrow, and one word,
// परसों, for both the day before and the day after; direction comes from the
// verb. So CLDR's long ±1 and ±2 words are identical, and `Numeric::Auto`
// on a Hindi day offset in the long style produces a phrase that is only
// unambiguous inside a sentence; the short and narrow styles say बीता कल and
// आने वाला कल, the past and the coming कल.

const HI: LocaleData = LocaleData {
    long: idioms(cldr48::HI.long, TimeUnit::Hour, "आधा घंटा", "डेढ़ घंटा"),
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
    ..cldr48::HI
};

// --- Indonesian -----------------------------------------------------------

const ID: LocaleData = LocaleData {
    long: idioms(
        cldr48::ID.long,
        TimeUnit::Hour,
        "setengah jam",
        "satu setengah jam",
    ),
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
    ..cldr48::ID
};

// --- Italian --------------------------------------------------------------

const IT: LocaleData = LocaleData {
    long: idioms(
        cldr48::IT.long,
        TimeUnit::Hour,
        "mezz’ora",
        "un’ora e mezza",
    ),
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
    ..cldr48::IT
};

// --- Japanese -------------------------------------------------------------

const JA: LocaleData = LocaleData {
    long: idioms(cldr48::JA.long, TimeUnit::Hour, "30 分", "1 時間半"),
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
    ..cldr48::JA
};

// --- Korean ---------------------------------------------------------------

const KO: LocaleData = LocaleData {
    long: idioms(cldr48::KO.long, TimeUnit::Hour, "30분", "1시간 30분"),
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
    ..cldr48::KO
};

// --- Dutch ----------------------------------------------------------------

const NL: LocaleData = LocaleData {
    long: idioms(
        cldr48::NL.long,
        TimeUnit::Hour,
        "een half uur",
        "anderhalf uur",
    ),
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
    ..cldr48::NL
};

// --- Polish ---------------------------------------------------------------
//
// Polish differs from Russian in exactly the place a naive implementation
// gets wrong: 22 is `few` in both, but 12 is `many` in both while 2 is `few`
// and 5 is `many`, and Polish puts `i = 1, v = 0` alone in `one` where
// Russian admits 21, 31, 101. Both are in the tests.

const PL: LocaleData = LocaleData {
    long: idioms(
        cldr48::PL.long,
        TimeUnit::Hour,
        "pół godziny",
        "półtorej godziny",
    ),
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
    ..cldr48::PL
};

// --- Portuguese -----------------------------------------------------------

const PT: LocaleData = LocaleData {
    long: idioms(
        cldr48::PT.long,
        TimeUnit::Hour,
        "meia hora",
        "uma hora e meia",
    ),
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
    ..cldr48::PT
};

// --- Russian --------------------------------------------------------------
//
// The canonical demonstration: 1 день, 2 дня, 5 дней, 21 день, 22 дня,
// 25 дней. The `other` column is reached only by decimals, which is how a
// half hour comes out as *1,5 часа*.

const RU: LocaleData = LocaleData {
    long: idioms(cldr48::RU.long, TimeUnit::Hour, "полчаса", "полтора часа"),
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
    ..cldr48::RU
};

// --- Thai -----------------------------------------------------------------

const TH: LocaleData = LocaleData {
    long: idioms(cldr48::TH.long, TimeUnit::Hour, "ครึ่งชั่วโมง", "หนึ่งชั่วโมงครึ่ง"),
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
    ..cldr48::TH
};

// --- Turkish --------------------------------------------------------------
//
// The comparative hedges are phrased so that no case suffix lands on `{0}`:
// Turkish would want an ablative there (*3 saatten fazla*) and the suffix
// changes with vowel harmony and with the final consonant, which a static
// pattern cannot produce. *{0} ve üzeri* and *en fazla {0}* are grammatical
// for every filling.

const TR: LocaleData = LocaleData {
    long: idioms(
        cldr48::TR.long,
        TimeUnit::Hour,
        "yarım saat",
        "bir buçuk saat",
    ),
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
    ..cldr48::TR
};

// --- Vietnamese -----------------------------------------------------------

const VI: LocaleData = LocaleData {
    long: idioms(cldr48::VI.long, TimeUnit::Hour, "nửa giờ", "một giờ rưỡi"),
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
    ..cldr48::VI
};

// --- Welsh ----------------------------------------------------------------
//
// Welsh is the only language in CLDR that uses all six categories, and its
// boundaries are nothing like Arabic's: `zero` for 0, `one` for 1, `two` for
// 2, `few` for 3 and `many` for 6 alone, with everything else `other`. Three
// and six are singled out because *tri* and *chwe* mutate the following
// noun where other numerals do not. The phrases are CLDR's `cy` entry's
// (the future years state all six); Welsh is carried here because a six-category language whose
// categories are *not* Arabic's is the only way to test that the selection
// really goes through the plural rules.

const CY: LocaleData = LocaleData {
    long: idioms(
        cldr48::CY.long,
        TimeUnit::Hour,
        "hanner awr",
        "awr a hanner",
    ),
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
    ..cldr48::CY
};

// --- Chinese, Simplified --------------------------------------------------
//
// Tagged `zh` rather than `zh-Hans` so that `zh`, `zh-Hans` and `zh-CN` all
// reach it by truncation; `zh-Hant` below is the separate entry.

const ZH: LocaleData = LocaleData {
    long: idioms(cldr48::ZH.long, TimeUnit::Hour, "半小时", "一个半小时"),
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
    ..cldr48::ZH
};

// --- Chinese, Traditional -------------------------------------------------

const ZH_HANT: LocaleData = LocaleData {
    long: idioms(cldr48::ZH_HANT.long, TimeUnit::Hour, "半小時", "一個半小時"),
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
    ..cldr48::ZH_HANT
};

// --- the locales of the most-spoken languages -----------------------------
//
// The languages `hc-i18n` added for Ethnologue's thirty most-spoken (see
// `docs/i18n.md`) carry nothing CLDR does not give: their entries are
// `cldr48`'s as they stand, with root's language-free hedges and weekday
// phrases, and no compact suffixes or indefinite units, so that a count is
// written with its numeral. `pt-PT` takes `pt`'s hedges and weekday
// phrases, which are the same language's. `pa_Arab.xml` states no fields,
// so Punjabi in the Arabic script has no entry and falls to root.

const PT_PT: LocaleData = LocaleData {
    approximate: PT.approximate,
    weekday: PT.weekday,
    ..cldr48::PT_PT
};

// --- the regional entries ---------------------------------------------------
//
// The CLDR 48 regional files `hc-i18n` carries (`scripts/locales-cldr.py`):
// each takes its CLDR part from its own file and its parents', so that
// `en-001` and `en-GB` close a list without the serial comma, *1 hour,
// 2 minutes and 3 seconds*, and write *3 mo ago*, as `en_001.xml` has them,
// and everything CLDR has no field for from its language's entry.
// Mongolian, a language of its own, carries nothing CLDR does not give, as
// the entries of the most-spoken languages above.

const AR_EG: LocaleData = regional(cldr48::AR_EG, &AR);
const EN_001: LocaleData = regional(cldr48::EN_001, &EN);
const EN_GB: LocaleData = regional(cldr48::EN_GB, &EN);
const ES_419: LocaleData = regional(cldr48::ES_419, &ES);
const ZH_HANT_HK: LocaleData = regional(cldr48::ZH_HANT_HK, &ZH_HANT);

/// Every locale this crate carries, in tag order.
///
/// Adding a language is one line here plus one `const` above.
pub static LOCALES: &[LocaleData] = &[
    AM, AR, AR_EG, BN, CS, CY, DE, EN, EN_001, EN_GB, ES, ES_419, FA, FIL, FR, HA, HE, HI, ID, IT,
    JA, JV, KO, ML, MN, MR, MY, NE, NL, PA_GURU, PCM, PL, PS, PT, PT_PT, RU, SW, SYR, TA, TE, TH,
    TR, UR, UR_IN, VI, YUE_HANS, YUE_HANT, ZH, ZH_HANT, ZH_HANT_HK,
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
