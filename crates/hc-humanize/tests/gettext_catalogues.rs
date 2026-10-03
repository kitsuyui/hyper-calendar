//! The vocabularies of `humanize` 4.16.0's gettext catalogues, which
//! `NaturalPhrases::by_catalogue` loads as `humanize.i18n.activate` does.
//!
//! The expected texts are the answers of Python's `gettext.GNUTranslations`
//! over the `.mo` that GNU `msgfmt --check` compiles from each `.po` of the
//! tag `4.16.0`, which is how `humanize` builds its catalogues
//! (`scripts/generate-translation-binaries.sh`), driven by a transcription of
//! `humanize`'s `time.py` and `number.py` of that tag; the project's README
//! gives `3 секунды назад` for `activate("ru_RU")`. The generator,
//! `scripts/humanize-gettext.py`, reads the same `.po` files, so these values
//! are an independent check of it.

#![allow(clippy::unwrap_used)]

use hc_core::Duration;
use hc_humanize::natural::{DeltaOptions, Gender, Natural, NaturalPhrases, PreciseUnit};

fn natural(catalogue: &str) -> Natural {
    Natural::for_catalogue(catalogue).unwrap_or_else(|| panic!("no catalogue {catalogue}"))
}

fn ago(catalogue: &str, seconds: i128) -> String {
    natural(catalogue)
        .naturaltime_delta(Duration::from_secs(seconds), DeltaOptions::default())
        .unwrap()
}

/// The README's example, and the plural forms the catalogues' own
/// `Plural-Forms` expressions choose: Russian's three, Polish's three,
/// Arabic's six, Latvian's, Slovene's four and the French and Japanese ones.
#[test]
fn the_plural_form_is_chosen_by_the_catalogues_expression() {
    assert_eq!(ago("ru_RU", 3), "3 секунды назад");
    assert_eq!(ago("ru_RU", 21), "21 секунда назад");
    assert_eq!(ago("ru_RU", 25), "25 секунд назад");
    assert_eq!(ago("pl_PL", 3), "3 sekundy temu");
    assert_eq!(ago("pl_PL", 21), "21 sekund temu");
    assert_eq!(ago("ar", 3), "قبل 3 ثوانٍ");
    assert_eq!(ago("ar", 21), "قبل 21 ثانية");
    assert_eq!(ago("lv", 3), "pirms 3 sekundēm");
    assert_eq!(ago("lv", 21), "pirms 21 sekundes");
    assert_eq!(ago("sl_SI", 3), "3 sekunde nazaj");
    assert_eq!(ago("sl_SI", 25), "25 sekund nazaj");
    assert_eq!(ago("fr_FR", 3), "il y a 3 secondes");
    assert_eq!(ago("ja_JP", 3), "3秒前");
    assert_eq!(ago("de_DE", 3), "vor 3 Sekunden");
    assert_eq!(ago("hu_HU", 3), "azóta eltelt 3 másodperc");
    // `now` and the tense of a span that is ahead.
    let ahead = Duration::from_secs(-2 * 3_600);
    assert_eq!(
        natural("ru_RU")
            .naturaltime_delta(ahead, DeltaOptions::default())
            .unwrap(),
        "через 2 часа"
    );
    assert_eq!(
        natural("ja_JP")
            .naturaltime_delta(ahead, DeltaOptions::default())
            .unwrap(),
        "2時間後"
    );
}

/// `precisedelta`'s list joiner is the catalogue's `%s and %s`, with its
/// text before and after the halves (Klingon ends with *je*), and its years
/// are grouped with the language's own separator.
#[test]
fn precisedelta_joins_with_the_catalogues_list_pattern() {
    let span = Duration::from_days(2) + Duration::from_secs(3_633) + Duration::from_millis(123);
    let precise = |catalogue: &str| {
        natural(catalogue)
            .precisedelta(span, PreciseUnit::Seconds, &[], 2)
            .unwrap()
    };
    assert_eq!(precise("ru_RU"), "2 дня, 1 час и 33.12 секунды");
    assert_eq!(precise("fr_FR"), "2 jours, 1 heure et 33.12 secondes");
    assert_eq!(precise("de_DE"), "2 Tage, 1 Stunde und 33.12 Sekunden");
    assert_eq!(precise("tlh"), "2 jaj, 1 rep 33.12 lup je");
    // An untranslated `%s and %s` is the English one, as in Python.
    assert_eq!(precise("ja_JP"), "2日, 1時間 and 33.12秒");
    assert_eq!(
        natural("fr_FR")
            .naturaldelta(Duration::from_days(400_000), DeltaOptions::default())
            .unwrap(),
        "1 095 ans"
    );
}

/// A message the catalogue leaves fuzzy or untranslated, which `msgfmt`
/// does not compile, is the English of the source, whatever the language.
#[test]
fn what_msgfmt_leaves_out_is_english() {
    // `ja_JP` leaves `thousand`, `zero` and `%s and %s` untranslated and
    // `billion` fuzzy.
    assert_eq!(natural("ja_JP").apnumber(0).unwrap(), "zero");
    assert_eq!(natural("ja_JP").apnumber(7).unwrap(), "七");
    assert_eq!(
        natural("ja_JP").intword(5_000_000_000, 1).unwrap(),
        "5.0 billion"
    );
    assert_eq!(natural("ja_JP").intword(1_234_567, 1).unwrap(), "1.2 百万");
    // `ko_KR`'s English-looking ordinals are fuzzy entries.
    assert_eq!(natural("ko_KR").ordinal(2).unwrap(), "2nd");
}

#[test]
fn the_number_functions_follow_the_catalogue() {
    let ru = natural("ru_RU");
    assert_eq!(ru.ordinal(2).unwrap(), "2ой");
    assert_eq!(ru.apnumber(7).unwrap(), "семь");
    assert_eq!(ru.intword(1_234_567, 1).unwrap(), "1.2 миллиона");
    assert_eq!(ru.intword(5_000_000_000, 1).unwrap(), "5.0 миллиардов");
    // The plural of the name of a power is chosen by `math.ceil` of the
    // number written before it, of any size.
    let huge = "5609745574825432564937787935285045626403434613616873560163168842756683252082940590137380007234662778";
    assert_eq!(
        ru.intword_digits(huge, 2).unwrap(),
        "5609745574825432307524351481905751650244483713970631286380554092544.00 децилиона"
    );
    // The feminine ordinals of a catalogue that has them.
    let mut feminine = String::new();
    natural("fr_FR")
        .write_ordinal_of(&mut feminine, 2, Gender::Female)
        .unwrap();
    assert_eq!(feminine, "2e");
    // The thousands and decimal separators `i18n.py` keeps in code.
    let intcomma = |catalogue: &str| {
        let phrases = NaturalPhrases::by_catalogue(catalogue).unwrap();
        natural(catalogue)
            .intcomma(1_234_567, phrases.grouping)
            .unwrap()
    };
    assert_eq!(intcomma("de_DE"), "1.234.567");
    assert_eq!(intcomma("fr_FR"), "1 234 567");
    assert_eq!(intcomma("ru_RU"), "1,234,567");
    assert_eq!(
        natural("de_DE").intword(1_234_567, 1).unwrap(),
        "1,2 Millionen"
    );
    assert_eq!(natural("lv").intword(1_234_567, 1).unwrap(), "1,2 miljoni");
    // 4.16.0 keeps a full stop as French's decimal separator; the project's
    // unreleased code corrected it to a comma.
    assert_eq!(
        NaturalPhrases::by_catalogue("fr_FR")
            .unwrap()
            .grouping
            .decimal,
        "."
    );
}

#[test]
fn the_catalogues_are_found_as_humanize_finds_them() {
    assert_eq!(NaturalPhrases::catalogues().len(), 35);
    assert_eq!(
        NaturalPhrases::by_catalogue("ru_RU").map(|p| p.catalogue),
        Some("ru_RU")
    );
    assert_eq!(
        NaturalPhrases::by_catalogue("pt-BR").map(|p| p.catalogue),
        Some("pt_BR")
    );
    // `activate` takes any name that starts with `en` as no translation.
    assert_eq!(
        NaturalPhrases::by_catalogue("en_GB").map(|p| p.catalogue),
        Some("en")
    );
    // No directory is called `ru`: Python raises, and a language is a
    // different lookup.
    assert_eq!(NaturalPhrases::by_catalogue("ru"), None);
    assert_eq!(
        NaturalPhrases::by_language("ru").map(|p| p.catalogue),
        Some("ru_RU")
    );
    assert_eq!(
        NaturalPhrases::by_language("ru-RU").map(|p| p.catalogue),
        Some("ru_RU")
    );
    // Two Portuguese and two Chinese catalogues: a bare language names none.
    assert_eq!(NaturalPhrases::by_language("pt"), None);
    assert_eq!(NaturalPhrases::by_language("zh"), None);
    assert_eq!(
        NaturalPhrases::by_language("zh-CN").map(|p| p.catalogue),
        Some("zh_CN")
    );
    assert_eq!(NaturalPhrases::by_language("sw"), None);
    assert!(Natural::for_language("de").is_some());
}

/// Each catalogue's expression must name a form its messages have, for
/// every count: otherwise Python falls back to English and this crate
/// would be reproducing a hole.
#[test]
fn every_translated_plural_has_the_form_its_expression_selects() {
    use hc_humanize::natural::gettext::plural_index;
    for phrases in NaturalPhrases::catalogues() {
        let messages = [
            phrases.seconds,
            phrases.minutes,
            phrases.hours,
            phrases.days,
            phrases.months,
            phrases.years,
        ];
        for message in messages {
            if message.forms.is_empty() {
                continue;
            }
            for n in 0..2_000 {
                let index = plural_index(phrases.plural_expression, n)
                    .unwrap_or_else(|| panic!("{}: no form for {n}", phrases.catalogue));
                assert!(
                    index < message.forms.len(),
                    "{} selects form {index} of {} for {n}",
                    phrases.catalogue,
                    message.forms.len()
                );
            }
            for form in message.forms {
                assert_eq!(
                    form.matches("{0}").count(),
                    1,
                    "{}: {form}",
                    phrases.catalogue
                );
            }
        }
    }
}
