//! A locale is served by a catalogue only if the catalogue translates every
//! message the function writes, so a result is in one language.
//!
//! The groups of messages a catalogue translates are generated with the
//! catalogues (`scripts/humanize-gettext.py` reads the `.po` files of
//! `humanize` 4.16.0); this test holds them to the strings the generated
//! catalogues carry, for every catalogue and every group of every function,
//! and holds the lookup to them.

#![allow(clippy::unwrap_used)]

use hc_humanize::natural::{NaturalPhrases, NaturalWords};
use hc_i18n::Locale;

const EVERY_FUNCTION: [NaturalWords; 12] = [
    NaturalWords::Any,
    NaturalWords::Apnumber,
    NaturalWords::Intword,
    NaturalWords::Delta,
    NaturalWords::DeltaFine,
    NaturalWords::Time,
    NaturalWords::TimeFine,
    NaturalWords::Precise,
    NaturalWords::PreciseFine,
    NaturalWords::Ordinal,
    NaturalWords::Day,
    NaturalWords::Naturalsize,
];

fn locale(catalogue: &NaturalPhrases) -> Locale {
    Locale::parse(catalogue.language).unwrap()
}

/// The words of a group of messages as the catalogue holds them, and as the
/// English of the source does.
fn groups(phrases: &NaturalPhrases) -> Vec<(&'static str, Vec<String>)> {
    let plural = |plural: hc_humanize::natural::Plural| -> Vec<String> {
        let mut all = vec![plural.one.to_string(), plural.other.to_string()];
        all.extend(plural.forms.iter().map(ToString::to_string));
        all
    };
    let own = |texts: &[&str]| texts.iter().map(ToString::to_string).collect::<Vec<_>>();
    let mut powers = Vec::new();
    for power in phrases.powers {
        powers.extend(power.forms.iter().map(ToString::to_string));
    }
    vec![
        (
            "articles",
            [
                own(&[
                    phrases.a_moment,
                    phrases.a_second,
                    phrases.a_minute,
                    phrases.an_hour,
                    phrases.a_day,
                    phrases.a_month,
                    phrases.a_year,
                    phrases.one_year_one_month,
                ]),
                plural(phrases.one_year_days),
                plural(phrases.one_year_months),
            ]
            .concat(),
        ),
        (
            "units",
            [
                plural(phrases.seconds),
                plural(phrases.minutes),
                plural(phrases.hours),
                plural(phrases.days),
                plural(phrases.months),
                plural(phrases.years),
            ]
            .concat(),
        ),
        (
            "fine_units",
            [plural(phrases.microseconds), plural(phrases.milliseconds)].concat(),
        ),
        (
            "moments",
            own(&[phrases.now, phrases.ago, phrases.from_now]),
        ),
        ("list_last", own(&[phrases.list_last])),
        (
            "ordinals",
            [
                own(&phrases.ordinal_by_last_digit),
                own(&phrases.ordinal_by_last_digit_female),
            ]
            .concat(),
        ),
        ("apnumber", own(&phrases.apnumber)),
        ("powers", powers),
        (
            "days",
            own(&[phrases.today, phrases.tomorrow, phrases.yesterday]),
        ),
        (
            "sizes",
            [
                own(&[phrases.byte, phrases.bytes]),
                own(&phrases.size_decimal),
                own(&phrases.size_binary),
            ]
            .concat(),
        ),
    ]
}

fn flag(phrases: &NaturalPhrases, group: &str) -> bool {
    let t = phrases.translated;
    match group {
        "articles" => t.articles,
        "units" => t.units,
        "fine_units" => t.fine_units,
        "moments" => t.moments,
        "list_last" => t.list_last,
        "ordinals" => t.ordinals,
        "apnumber" => t.apnumber,
        "powers" => t.powers,
        "days" => t.days,
        "sizes" => t.sizes,
        _ => unreachable!(),
    }
}

/// A group the catalogue calls translated holds no raw placeholder in the
/// words that take no number, and a group it does not is the English of the
/// source or such a placeholder in at least one message.
#[test]
fn the_flags_agree_with_the_strings() {
    let english = groups(&NaturalPhrases::ENGLISH);
    for catalogue in NaturalPhrases::catalogues() {
        let own = groups(catalogue);
        for ((group, mine), (_, theirs)) in own.iter().zip(&english) {
            let placeholder = matches!(*group, "ordinals" | "apnumber" | "powers")
                && mine.iter().any(|text| text.contains('%'));
            if flag(catalogue, group) {
                assert!(
                    !placeholder,
                    "{} calls {group} translated and holds a placeholder",
                    catalogue.catalogue
                );
            } else {
                // A group is untranslated because some message is the
                // English of the source, which has no `forms`, so the
                // strings differ in count or in content from a translation.
                assert!(
                    placeholder || mine == theirs || has_english_message(catalogue, group),
                    "{} calls {group} untranslated but nothing in it is",
                    catalogue.catalogue
                );
            }
        }
    }
}

/// Whether some message of the group in the catalogue is the English of the
/// source: a plural with no `forms`, or a word that equals the source's.
fn has_english_message(catalogue: &NaturalPhrases, group: &str) -> bool {
    let english = &NaturalPhrases::ENGLISH;
    let plural_missing = |plurals: &[hc_humanize::natural::Plural]| {
        plurals.iter().any(|plural| plural.forms.is_empty())
    };
    let words_equal = |mine: &[&str], theirs: &[&str]| mine.iter().zip(theirs).any(|(a, b)| a == b);
    match group {
        "articles" => {
            [
                catalogue.a_moment,
                catalogue.a_second,
                catalogue.a_minute,
                catalogue.an_hour,
                catalogue.a_day,
                catalogue.a_month,
                catalogue.a_year,
                catalogue.one_year_one_month,
            ]
            .iter()
            .zip([
                english.a_moment,
                english.a_second,
                english.a_minute,
                english.an_hour,
                english.a_day,
                english.a_month,
                english.a_year,
                english.one_year_one_month,
            ])
            .any(|(a, b)| *a == b)
                || plural_missing(&[catalogue.one_year_days, catalogue.one_year_months])
        }
        "units" => plural_missing(&[
            catalogue.seconds,
            catalogue.minutes,
            catalogue.hours,
            catalogue.days,
            catalogue.months,
            catalogue.years,
        ]),
        "fine_units" => plural_missing(&[catalogue.microseconds, catalogue.milliseconds]),
        "moments" => words_equal(
            &[catalogue.now, catalogue.ago, catalogue.from_now],
            &[english.now, english.ago, english.from_now],
        ),
        "list_last" => catalogue.list_last == english.list_last,
        "ordinals" => {
            words_equal(
                &catalogue.ordinal_by_last_digit,
                &english.ordinal_by_last_digit,
            ) || words_equal(
                &catalogue.ordinal_by_last_digit_female,
                &english.ordinal_by_last_digit_female,
            )
        }
        "apnumber" => words_equal(&catalogue.apnumber, &english.apnumber),
        "powers" => plural_missing(&catalogue.powers),
        "days" => words_equal(
            &[catalogue.today, catalogue.tomorrow, catalogue.yesterday],
            &[english.today, english.tomorrow, english.yesterday],
        ),
        "sizes" => {
            catalogue.byte == english.byte
                || catalogue.bytes == english.bytes
                || words_equal(&catalogue.size_decimal, &english.size_decimal)
                || words_equal(&catalogue.size_binary, &english.size_binary)
        }
        _ => unreachable!(),
    }
}

/// For every catalogue and every function, the catalogue's own locale is
/// served by it when it translates every message of the function and by the
/// English of the source otherwise, never by it in part.
#[test]
fn a_catalogue_serves_its_own_locale_only_whole() {
    for catalogue in NaturalPhrases::catalogues() {
        for words in EVERY_FUNCTION {
            let served = NaturalPhrases::for_locale(&locale(catalogue), words);
            if catalogue.translates(words) {
                assert_eq!(
                    served.catalogue, catalogue.catalogue,
                    "{} translates {words:?} and must serve it",
                    catalogue.catalogue
                );
            } else {
                assert_eq!(
                    served.catalogue, "en",
                    "{} does not translate {words:?} and must not serve it",
                    catalogue.catalogue
                );
            }
        }
    }
}

/// Every message of every function a served locale writes is translated:
/// whatever `for_locale` answers translates the words asked, for every
/// locale tag the catalogues name and every region of them.
#[test]
fn whatever_serves_a_locale_translates_all_it_writes() {
    for catalogue in NaturalPhrases::catalogues() {
        let language = catalogue.language.split('-').next().unwrap();
        for tag in [
            catalogue.language.to_string(),
            language.to_string(),
            format!("{language}-001"),
            format!("{language}-AX"),
        ] {
            let locale = Locale::parse(&tag).unwrap();
            for words in EVERY_FUNCTION {
                let served = NaturalPhrases::for_locale(&locale, words);
                assert!(
                    served.translates(words),
                    "{tag} for {words:?} is served by {} which does not translate it",
                    served.catalogue
                );
            }
        }
    }
}

// --- every function over every locale ------------------------------------

use hc_core::Duration;
use hc_humanize::natural::{DeltaOptions, Gender, Natural, PreciseUnit, SizeStyle};

/// The words of the English of the source that a result in another language
/// must not hold, unless that language's catalogue has the same word.
const ENGLISH_WORDS: [&str; 33] = [
    "a",
    "an",
    "and",
    "ago",
    "from",
    "now",
    "moment",
    "second",
    "seconds",
    "minute",
    "minutes",
    "hour",
    "hours",
    "day",
    "days",
    "month",
    "months",
    "year",
    "years",
    "millisecond",
    "milliseconds",
    "microsecond",
    "microseconds",
    "today",
    "tomorrow",
    "yesterday",
    "thousand",
    "million",
    "billion",
    "trillion",
    "googol",
    "byte",
    "bytes",
];

/// Every tag the sweep tries: the languages of the catalogues, with and
/// without a region, and every locale `hc-i18n` carries.
fn tags() -> Vec<String> {
    let mut tags: Vec<String> = Vec::new();
    for catalogue in NaturalPhrases::catalogues() {
        let language = catalogue.language;
        tags.push(language.to_string());
        tags.push(language.split('-').next().unwrap().to_string());
    }
    for data in hc_i18n::data::LOCALES {
        tags.push(data.tag.to_string());
    }
    tags.extend(["de-AT", "pt-AO", "zh-Hant-TW", "no", "en-GB", "und"].map(String::from));
    tags.sort();
    tags.dedup();
    tags
}

fn words_of(texts: impl IntoIterator<Item = String>) -> Vec<String> {
    let mut words = Vec::new();
    for text in texts {
        words.extend(
            text.split(|c: char| !c.is_alphabetic())
                .filter(|word| !word.is_empty())
                .map(str::to_lowercase),
        );
    }
    words
}

/// Every function of `humanize` that writes words, over spans and counts
/// that reach each of its messages: the results, each with the words that
/// must have served it.
fn results(tag: &str) -> Vec<(NaturalWords, String)> {
    let locale = Locale::parse(tag).unwrap();
    let with = |words| Natural::with_phrases(NaturalPhrases::for_locale(&locale, words));
    let mut out = Vec::new();
    let default = DeltaOptions::default();
    let fine = DeltaOptions {
        months: true,
        minimum_unit: PreciseUnit::Microseconds,
    };
    let seconds = |s: i128| Duration::from_secs(s);
    let spans = [
        seconds(0),
        seconds(1),
        seconds(45),
        seconds(61),
        seconds(3_600),
        seconds(2 * 86_400 + 3_600),
        seconds(40 * 86_400),
        seconds(400 * 86_400),
        seconds(800 * 86_400),
        seconds(-7_200),
    ];
    for span in spans {
        out.push((
            NaturalWords::Delta,
            with(NaturalWords::Delta)
                .naturaldelta(span, default)
                .unwrap(),
        ));
        out.push((
            NaturalWords::Time,
            with(NaturalWords::Time)
                .naturaltime_delta(span, default)
                .unwrap(),
        ));
    }
    for span in [Duration::from_micros(5), Duration::from_micros(123_000)] {
        out.push((
            NaturalWords::DeltaFine,
            with(NaturalWords::DeltaFine)
                .naturaldelta(span, fine)
                .unwrap(),
        ));
        out.push((
            NaturalWords::TimeFine,
            with(NaturalWords::TimeFine)
                .naturaltime_delta(span, fine)
                .unwrap(),
        ));
    }
    let long = Duration::from_micros((2 * 86_400 + 3_600) * 1_000_000 + 33_120_000);
    out.push((
        NaturalWords::Precise,
        with(NaturalWords::Precise)
            .precisedelta(long, PreciseUnit::Seconds, &[], 2)
            .unwrap(),
    ));
    out.push((
        NaturalWords::PreciseFine,
        with(NaturalWords::PreciseFine)
            .precisedelta(
                Duration::from_micros(123_000),
                PreciseUnit::Microseconds,
                &[],
                2,
            )
            .unwrap(),
    ));
    for value in 0..=10 {
        out.push((
            NaturalWords::Apnumber,
            with(NaturalWords::Apnumber).apnumber(value).unwrap(),
        ));
    }
    for digits in [
        "1000",
        "12345678",
        "1234567890",
        "1000000000000",
        "1000000000000000000000000000000000",
        "1000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
    ] {
        out.push((
            NaturalWords::Intword,
            with(NaturalWords::Intword)
                .intword_digits(digits, 1)
                .unwrap(),
        ));
    }
    for value in 1..=14 {
        for gender in [Gender::Male, Gender::Female] {
            let natural = with(NaturalWords::Ordinal);
            let mut text = String::new();
            natural.write_ordinal_of(&mut text, value, gender).unwrap();
            out.push((NaturalWords::Ordinal, text));
        }
    }
    for value in [1.0, 300.0, 3000.0, 3_000_000_000.0] {
        for style in [SizeStyle::Decimal, SizeStyle::Binary, SizeStyle::Gnu] {
            out.push((
                NaturalWords::Naturalsize,
                with(NaturalWords::Naturalsize)
                    .naturalsize(value, style, 1)
                    .unwrap(),
            ));
        }
    }
    #[cfg(feature = "format")]
    for offset in [-1, 0, 1] {
        let today = hc_calendar::Rd(739_000);
        out.push((
            NaturalWords::Day,
            with(NaturalWords::Day)
                .naturalday(hc_calendar::Rd(today.0 + offset), today, "%b %d")
                .unwrap(),
        ));
    }
    out
}

/// Over every function and every locale a result is in one language: the
/// English of the source whole, or the words of one catalogue only, with no
/// English word of the source left in it and no placeholder.
#[test]
fn no_function_writes_two_languages_in_one_result() {
    for tag in tags() {
        let locale = Locale::parse(&tag).unwrap();
        let english = results("en");
        let ours = results(&tag);
        assert_eq!(english.len(), ours.len());
        for (index, (words, text)) in ours.iter().enumerate() {
            assert!(
                !text.contains('%'),
                "{tag} {words:?}: a raw placeholder in {text:?}"
            );
            let served = NaturalPhrases::for_locale(&locale, *words);
            if served.catalogue == "en" {
                assert_eq!(
                    text, &english[index].1,
                    "{tag} {words:?} is English but differs from English"
                );
                continue;
            }
            assert!(served.translates(*words), "{tag} {words:?}");
            // The words the catalogue itself has are its own.
            let own = words_of(
                groups(served)
                    .into_iter()
                    .flat_map(|(_, texts)| texts)
                    .chain([
                        served.list_separator.to_string(),
                        served.a_moment.to_string(),
                    ]),
            );
            for word in words_of([text.clone()]) {
                assert!(
                    !ENGLISH_WORDS.contains(&word.as_str()) || own.contains(&word),
                    "{tag} {words:?}: {word:?} of the English source in {text:?}, \
                     served by {}",
                    served.catalogue
                );
            }
        }
    }
}

/// A catalogue that leaves the *and* of its list, or any word of its powers,
/// untranslated, or writes a raw placeholder for one, does not serve the
/// function that writes it: the result is the English of the source.
#[test]
fn partly_translated_catalogues_answer_in_english() {
    let precise = |tag: &str| {
        let locale = Locale::parse(tag).unwrap();
        Natural::with_phrases(NaturalPhrases::for_locale(&locale, NaturalWords::Precise))
            .precisedelta(
                Duration::from_micros((2 * 86_400 + 3_600) * 1_000_000 + 33_120_000),
                PreciseUnit::Seconds,
                &[],
                2,
            )
            .unwrap()
    };
    for tag in ["ja", "ko", "zh-CN", "sk"] {
        assert_eq!(precise(tag), "2 days, 1 hour and 33.12 seconds", "{tag}");
    }
    assert_eq!(precise("de"), "2 Tage, 1 Stunde und 33.12 Sekunden");
    let intword = |tag: &str| {
        let locale = Locale::parse(tag).unwrap();
        Natural::with_phrases(NaturalPhrases::for_locale(&locale, NaturalWords::Intword))
            .intword(1_000_000, 1)
            .unwrap()
    };
    // `bn`, `ko` and `vi` leave words untranslated or write a raw
    // placeholder; `de` and `ru` translate all of them.
    for tag in ["bn-BD", "ko", "vi", "ca", "id", "nl", "sk", "ja"] {
        assert_eq!(intword(tag), "1.0 million", "{tag}");
    }
    assert_eq!(intword("de"), "1,0 Million");
    assert_eq!(intword("ru"), "1.0 миллион");
}
