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
| `names` | Months, weekdays, day periods, eras, quarters and the sexagenary cycle, keyed by (locale, calendar, width, context); each locale's names for the calendars, and the templates by which `hc-format` writes a year with its era, a day and a date |
| `direction` | Script direction and the bidi isolation a formatter needs to embed a date in text running the other way |
| `casing` | Turkish dotted/dotless i, and whether a language capitalises month names at all |

`Locale` is `Copy` and allocation-free: subtags live in inline ASCII buffers,
and rendering goes through `core::fmt::Write`. Everything works with
`--no-default-features` (no allocator at all), given the floating-point math
every `no_std` build of the workspace needs from the `libm` feature, which
passes through to `hc-core`; the `alloc` feature only adds the
`String`-returning conveniences.

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
you: every cycle's names as many as the calendar declares for that cycle
(nineteen Badíʿ months, ten décade days — the calendar says, not the test),
7 weekdays, 2 day periods, 4 quarters, era codes and era names the same
length, no empty or padded strings, no duplicate calendar entries, a
numbering system that exists, and a canonical tag.

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

Locales shipped: `am ar ban bn bo cop cs de en es fa fr he hi id it ja jv
kab ko mid ml my nah ne nl pl pt ru sa syr ta th tr vi yua zap zgh zh-Hans
zh-Hant`, plus the `und` root. Non-Gregorian vocabulary: Hijri months
(Arabic, English), Hebrew months (Hebrew, English), Babylonian months
(English), the Chinese calendar's months in both Chinese scripts, in
Japanese (正月, 二月 … 十二月) and in Korean, the Tibetan months in Tibetan,
the numbered lunisolar months in English, the Japanese lunisolar calendars'
traditional names (睦月 … 師走, which serve those five calendars and no
other), Japanese era names, the Ethiopic months in Amharic, the Coptic
months in Coptic and in Egyptian Arabic, the Burmese months in Burmese, the
Bikram Sambat and Nepal Sambat months in Devanagari, the Hindu lunisolar and
Vikrami solar months in Devanagari (Sanskrit and Hindi), the Indian national
calendar's months in Hindi, Tamil, Malayalam and Bengali, the Tamil,
Malayalam and Bengali solar months in their own scripts, the Assyrian months
in Syriac, the Berber months in Kabyle and in Tifinagh, the Maya day-signs
and haabʼ months in Yucatec, the Aztec day-signs and months in Nahuatl, the
Zapotec *yza*'s months and year bearers in Zapotec, the
Pawukon cycles in Balinese, the pasaran and dina in Javanese, the Mandaean
weekdays in Mandaic, romanisations of the Coptic, Ethiopic, Burmese,
Armenian and Persian months (whose own scripts the calendars carry
themselves), and the zodiac animals in Chinese, Japanese, Korean, Vietnamese
and English. The stems and branches are not spelled here: each locale names
one of the readings `hc_calendar::cycle::readings` catalogues.

Nineteen of the locales exist for a calendar's own language, and they cover
what their sources cover and no more:

| Locale | Calendar | Gregorian vocabulary | Calendar vocabulary | Not carried |
|---|---|---|---|---|
| `am` Amharic | `ethiopic`, `coptic` | CLDR 48 `am.xml` | the thirteen Ethiopic months (CLDR `ethiopic`), the Coptic era abbreviation ዓ/ም (CLDR `coptic`) | Ethiopic era names: CLDR's `am` inherits root's Latin `AA`/`AM` |
| `cop` Coptic | `coptic` | none: CLDR has no `cop`, so it inherits | the thirteen Bohairic months (Wikipedia, "Coptic calendar") | weekdays, day periods, the era: no source read names them |
| `my` Burmese | `burmese` | CLDR 48 `my.xml` | the twelve months (Wikipedia, "Burmese calendar") | a "Second Waso" prefix in Burmese script |
| `bo` Tibetan | `tibetan` | CLDR 48 `bo.xml` | the numbered months, CLDR's own ordinal month names keyed to the calendar that numbers its months | the doubled-month prefix; the sixty-year cycle, which the crate's cycle model cannot hold |
| `ne` Nepali | `bikram-sambat`, `nepal-sambat` | CLDR 48 `ne.xml` | the Bikram Sambat months as the Nepal Rajpatra spells them, the Nepal Sambat months in Devanagari (Wikipedia, "Nepal Sambat") | either era in Devanagari; a `new` (Newar) locale, which CLDR does not have |
| `sa` Sanskrit | `hindu-lunar`, `hindu-lunar-purnimanta`, `hindu-solar-vikrami` | CLDR 48 `sa.xml`, less the abbreviated months and the weekdays, which inherit because CLDR prints an ASCII colon for a visarga in the former and in Thursday's wide form | the twelve lunar months in Devanagari as the amānta calendar declares them (Rashtriya Panchang, Sanskrit edition; Wikipedia, "Hindu calendar"), the prefix अधिक (Wikipedia, "Adhik Maas"), the Vikrami solar months from Vaiśākha | the eras (CLDR's default forms are Latin); the rāśi names in Devanagari and the nakṣatras: no source read prints the former, no calendar declares the latter |
| `hi` Hindi | `indian`, `hindu-lunar`, `hindu-lunar-purnimanta`, `hindu-solar-vikrami` | CLDR 48 `hi.xml` | the national calendar's months and era abbreviation शक (CLDR `indian`), the same twelve names keyed to the lunisolar calendars with the prefix अधिक, and from Vaiśākha to the Vikrami solar calendar | the rāśi names in Devanagari; the nakṣatras |
| `ta` Tamil | `indian`, `hindu-solar-tamil` | CLDR 48 `ta.xml` | the Tamil months சித்திரை … (CLDR `indian`), serving both calendars, with the era abbreviation சாகா | day periods: CLDR's `ta` inherits root's |
| `ml` Malayalam | `indian`, `hindu-solar-malayalam` | CLDR 48 `ml.xml` | the national calendar's months in Malayalam and the era abbreviation ശക (CLDR `indian`); the Kollam months ചിങ്ങം … (Wikipedia, "Malayalam calendar") | day periods: CLDR's `ml` inherits root's |
| `bn` Bengali | `indian`, `hindu-solar-bengali`, `bangladeshi` | CLDR 48 `bn.xml` | the twelve months (CLDR `indian`), Chaitra first for the national calendar and Boishakh first for the Bengali year, the same spellings Wikipedia's "Bangladeshi national calendar" prints; CLDR's era abbreviation সাল | day periods: CLDR's `bn` inherits root's |
| `yua` Yucatec Maya | `maya-tzolkin`, `maya-haab`, `maya-round` and their `-gmt2` twins | none: CLDR has no `yua`, so it inherits | the twenty day-signs and nineteen haabʼ months in the sixteenth-century Yucatec spelling the calendar declares (Reingold and Dershowitz; Wikipedia, "Tzolkʼin", "Maya calendar") | the revised orthography; weekdays, day periods, eras |
| `nah` Nahuatl | `aztec-tonalpohualli`, `aztec-xiuhpohualli` | none: CLDR has no `nah`, so it inherits | the twenty day-signs and nineteen months as the calendar declares them (Wikipedia, "Tonalpohualli"; Reingold and Dershowitz) | weekdays, day periods, eras |
| `zap` Zapotec | `zapotec-yza` | none: CLDR has no `zap`, so it inherits | the nineteen months of Manuscript 85 and the four year bearers as the calendar declares them (Urcid, *Zapotec Hieroglyphic Writing*, Table 3.5; Tavárez and Justeson 2008, Table 1) | the day-signs of the 260-day count, whose names change with their number; weekdays, day periods, eras |
| `ban` Balinese | `balinese-pawukon` | none: CLDR has no `ban`, so it inherits | all ten Pawukon cycles as the calendar declares them (Reingold and Dershowitz, §10.6); the Saptawara as weekdays | Balinese script: no source read prints it; day periods, eras |
| `jv` Javanese | `javanese-pasaran` | CLDR 48 `jv.xml` | the five pasaran and the seven dina (Wikipedia, "Javanese calendar"), with Monday in the Javanese form Senen the module declares (Javanese Wikipedia and Wiktionary, "Senèn") where that page prints the Indonesian Senin | the Javanese-script forms that page prints, `jv` being a Latin-script locale |
| `syr` Syriac | `assyrian` | CLDR 48 `syr.xml` | the twelve Assyrian months in vocalised East Syriac (Wikipedia, "Assyrian calendar"), Neesan first, ܛܲܒܵܚ for Tabakh | the era AY in Syriac |
| `kab` Kabyle | `berber` | CLDR 48 `kab.xml` | the Gregorian months, which are the agrarian calendar's under the same Latin-derived names (Encyclopédie berbère, "Calendrier"); CLDR spells Fuṛar and Nunembeṛ where the calendar has Furar and Wambeṛ | — |
| `zgh` Standard Moroccan Tamazight | `berber` | CLDR 48 `zgh.xml` | the Gregorian months in Tifinagh, ⵉⵏⵏⴰⵢⵔ …, keyed to the agrarian calendar for the same reason | Kabyle forms in Tifinagh: no source read prints them |
| `mid` Mandaic | `mandaean` | none: CLDR has no `mid`, so it inherits | the seven weekdays in Mandaic script (Wikipedia, "Mandaean calendar") | the months: that page prints the twelve zodiacal names but no Mandaic Parwanaia, and the calendar's month cycle has thirteen positions; day periods, eras |

Plural languages: `ar cs cy da de en es fi fr ga he hi id it ja ko lt lv nl pl
pt ro ru sl sv th tr uk vi zh`.

## What it deliberately does not do

* **No collation, no message formatting, no general date patterns.** The
  templates here are a locale's way of writing a year with its era, a day
  and one whole date — `{month} {day}, {year}` — read from CLDR's `Gy`,
  `d` and `yMMMMd` items and rendered by `hc-format`; skeletons, interval
  patterns and time patterns are not here.
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
