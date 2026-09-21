# hc-i18n

Internationalisation (i18n), multilingualisation (m17n) and localisation
(L10n) for calendar data: BCP 47 locales, CLDR plural rules, numbering
systems, localised calendar vocabulary, script direction and locale-dependent
casing.

A calendar computes. A locale decides what the computation is *called*.
`hc-calendar` can say that a day is month 9 of year 2024 in the Gregorian
calendar; only a locale can say whether that prints as "September",
"сентября", "сентябрь", "eylül" or "９月". This crate owns that half, and
nothing else in the workspace hard-codes a localised string.

## What it covers

| Module | What it does |
|---|---|
| `locale` | `language[-Script][-REGION][-variant]` plus the `-u-ca`, `-u-nu`, `-u-fw` and `-u-hc` keys; parse, render, and the CLDR inheritance chain as an iterator |
| `numbering` | 9 positional digit systems (`latn`, `arab`, `arabext`, `deva`, `beng`, `thai`, `mymr`, `hanidec`, `fullwide`) and 4 algorithmic Han styles (`jpan`, `jpanfin`, `hans`, `hant`), rendered and parsed back |
| `plural` | CLDR cardinal categories and the full operand set (`n i v w f t`) for 30 languages |
| `names` | Months, weekdays, day periods, eras, quarters and the sexagenary cycle, keyed by (locale, calendar, width, context) |
| `direction` | Script direction and the bidi isolation a formatter needs to embed a date in text running the other way |
| `casing` | Turkish dotted/dotless i, and whether a language capitalises month names at all |

`Locale` is `Copy` and allocation-free: subtags live in inline ASCII buffers,
and rendering goes through `core::fmt::Write`. Everything works with
`--no-default-features` (no allocator at all); the `alloc` feature only adds
the `String`-returning conveniences.

## Data is not code

This is the point of the crate's layout, so it is worth stating plainly.

Every locale is **one `LocaleData` value of `&'static` slices** in
`data::LOCALES`. Lookup (`names::resolve`) walks `Locale::fallback()` and
takes the first entry that actually carries the field asked for — not merely
the first entry that matches the locale. Nothing in `names.rs` knows which
languages exist, and no function grows a branch when a language is added.

The same split holds elsewhere: `numbering::ALL` is a table of digit arrays
and Han styles over two shared algorithms, and `plural::RULES` is a table of
`(language, fn)` pairs where languages with identical CLDR rule text share
one function.

### Adding a locale

1. Write a `const` `LocaleData` in `src/data.rs`, copying the nearest
   existing entry for shape.
2. Add its name to the `LOCALES` array, keeping the array in tag order.

That is all. The consistency test suite will then check the new entry for
you: 12 or 13 months, 7 weekdays, 2 day periods, 4 quarters, era codes and
era names the same length, no empty or padded strings, no duplicate calendar
entries, a numbering system that exists, and a canonical tag.

Leaving a field empty is meaningful: an empty slice means *inherit*, so a
locale states only what differs from its parent. Japanese never abbreviates
`1月`, so its abbreviated months are empty and the wide form answers for
every width.

### Adding a plural language

Add one `(subtag, rule_fn)` row to `plural::RULES`, reusing an existing rule
function if CLDR's rule text for the language is identical to one already
there. The table is checked for sortedness and uniqueness by a test.

## Reference data and accuracy

* **Vocabulary** follows the Unicode CLDR common locale data (the
  `main/<locale>.xml` `calendars` sections). It is **hand-checked, not
  generated**: a subset chosen for calendar work, with the most widely used
  alternative taken where CLDR offers several. It will not track a CLDR
  release automatically, and it is not a drop-in replacement for ICU.
* **Plural rules** follow `supplemental/plurals.xml`. They are implemented
  from the published rule text, and the tests assert the published sample
  values, not values derived from this implementation.
* **First day of week** follows `supplementalData.xml` `weekData/firstDay`;
  only the non-Monday exceptions are tabulated.
* **Bidi** follows UAX 9 §2.4 (isolates) and §P2–P3 (first-strong).
* **Casing** follows UAX 21 plus the Turkic tailoring that Unicode itself
  specifies for `tr` and `az`.

Locales shipped: `ar cs de en es fa fr he hi id it ja ko nl pl pt ru th tr vi
zh-Hans zh-Hant`, plus the `und` root. Non-Gregorian vocabulary: Hijri months
(Arabic, English), Hebrew months (Hebrew, English), Persian months (Persian),
the Chinese lunisolar months in both scripts, their Japanese traditional names
(睦月 … 師走), Japanese era names, and the stems, branches and zodiac animals
in Chinese, Japanese, Korean and romanised English.

Plural languages: `ar cs cy da de en es fi fr ga he hi id it ja ko lt lv nl pl
pt ro ru sl sv th tr uk vi zh`.

## What it deliberately does not do

* **No collation, no message formatting, no date patterns.** Assembling
  `{month} {day}, {year}` is `hc-format`'s job; this crate supplies the
  pieces.
* **No number formatting beyond integers.** No grouping separators, no
  decimal separator, no currency, no sign other than an ASCII hyphen.
* **No compact-notation plural operands (`c`/`e`).** Where a CLDR rule has an
  `e = 0 and …` branch the `e = 0` case is implemented and the `e != 0`
  alternatives are dropped, which only affects compact forms such as "1M".
* **No bidirectional algorithm.** `direction` emits isolates and answers
  first-strong questions; it does not reorder text.
* **No word-level title casing.** Only the first character is ever recased,
  because a per-word rule would mangle *2 de enero* and *tháng 1*.
* **No transliteration**, no Hebrew or Greek alphabetic numerals, no
  counting-rod numerals.
* **Lossless or nothing in tags.** A `-u-` key this crate does not model, or
  a `-t-`/`-x-` extension, is an error rather than something silently
  dropped: a formatter that ignored `-u-co-phonebk` would answer for a tag it
  was not given.

## Accuracy claims

The crate makes no numerical claims — there is no physics here. What it does
claim:

* Tag parse → render is **lossless** for every tag it accepts, and case is
  normalised to the BCP 47 canonical form.
* Numbering systems **round-trip**: `parse_integer(format_integer(n)) == n`
  for every `n` in `i64` for positional systems, and for every `|n| < 10^16`
  for the Han styles (bigger values need 京 and are refused).
* Plural categories match the CLDR sample values for the 30 languages
  implemented.
* The vocabulary is as accurate as CLDR and a careful hand-check; where a
  language has forms this crate does not model (finer day periods, Dutch
  `IJ`, Arabic month name variants outside the Levant), it returns the common
  form rather than guessing.
