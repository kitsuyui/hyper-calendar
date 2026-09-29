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
| `numbering` | 15 positional digit systems (`latn`, `fullwide`, `arab`, `arabext`, `deva`, `beng`, `guru`, `java`, `mlym`, `mymr`, `tamldec`, `telu`, `thai`, `tibt`, `hanidec`), 4 algorithmic Han styles (`jpan`, `jpanfin`, `hans`, `hant`), Hebrew numerals (`hebr`, by CLDR's rules, 1 to 9 999) and Greek alphabetic numerals (`grek`, `greklow`, by CLDR's `%greek-upper` and `%greek-lower`), rendered and parsed back |
| `plural` | CLDR cardinal categories and the full operand set (`n i v w f t`) for 55 languages and `pt-PT` |
| `names` | Months, weekdays, day periods, eras, quarters and the sexagenary cycle, keyed by (locale, calendar, width, context); each locale's names for the calendars, and the templates by which `hc-format` writes a year with its era, a day and a date |
| `notation` | The notations a calendar's sources write its dates in whatever the language — the Long Count's `13.0.13.17.8`, the ISO week date `2026-W39-7` — as a level of templates between a calendar's entries and a locale's general ones, each citing its source |
| `fields` | What a calendar's extra fields are called — an English label for every field a registered calendar sets, in its system document's words — and which cycle names each one's values, for the `{extra:FIELD}` placeholder and the lines that list a day's extra fields |
| `almanac` | What a locale calls the Japanese almanac's annotations — 六曜, 二十八宿, 九星, 十二直, 納音, the 暦注下段, the 選日 and their combinations — by kind and position or identifier: Japanese, as the almanacs print them, and English, in their Hepburn romanisation and the asterisms' English names; `zh-Hans` for the 納音 alone, as 『三命通會』 names them; no other locale, since no other language's names were read |
| `reckonings` | What a locale calls the terms of the other reckonings of a day and a year — the choghadiya, the Panchak kinds, the kālam, the Kumbh sites and Pushkaram rivers, the planets of the planetary hours, the night watches, and the Vietnamese, Chinese and Turkish folk days — each in the language its source writes it in, English, Chinese, Vietnamese or Turkish, with the crates' own names, and in Hindi or the other Chinese script where a source in it was read; no term is translated or converted |
| `horizons` | What a locale calls a horizon a rising is measured against, only where an observatory or almanac office names it in the language: the Hong Kong Observatory's name for the USNO in `zh-Hant` and `zh-Hans`, the IMCCE's in `fr`; every other locale and horizon is left to the English name |
| `holiday_groups` | What a locale calls a group of people a holiday is given to alone (`hc_holiday::group`), only where an instrument in the language names the group: China's Article 3 names for women, youth, children and active servicemen in `zh-Hans`, and the names Nepal's Home Ministry notices give their communities, faiths and employees in `ne`; every other locale and group is left to the English name |
| `holiday_names` | What a locale calls a day of a holiday table beside the table's own names, only where a source prints it: seven Coptic Orthodox feasts in `cop`, from Wikipedia's "Nayrouz" and the Coptic Wikipedia's test project (both secondary) |
| `dated` | Month and weekday names a government gave for a period, with the days they were in force and the days no source decides: Turkmenistan's of 2002–2008 |
| `direction` | Script direction and the bidi isolation a formatter needs to embed a date in text running the other way |
| `casing` | Turkish dotted/dotless i, and whether a language capitalises month names at all |
| `territories` | With the `territories` feature: CLDR's names for the 195 countries the workspace keeps holiday tables for, in every carried locale CLDR names them in, and CLDR's `alt="short"` names for the few it shortens |
| `exemplar_cities` | With the `exemplar-cities` feature: CLDR's English exemplar city of each of the 418 zones `hc-tz` locates; with `localized-exemplar-cities`, the city in every other carried locale CLDR names it in |
| `zone_names` | CLDR 48's time zone data for the pattern fields `z`, `v`, `V` and `O`: every carried locale's zone formats (`gmtFormat`, `hourFormat`, `regionFormat`, …), always; with the `zone-names` feature, each zone's metazones and their periods, the golden, preferred and primary zones, the BCP 47 zone identifiers, the languages' likely regions and English's names; with `localized-zone-names`, every other carried locale's metazone and zone names; generated by `scripts/zone-names-cldr.py` |
| `day_periods` | CLDR 48's day periods beyond am and pm, for the pattern fields `b` and `B`: each carried language's format rule set from `dayPeriods.xml`, found by truncating the tag, and the names of midnight, noon and the flexible periods in three widths; generated by `scripts/day-periods-cldr.py` |
| `place_names` | With the `place-names` feature: CLDR's name of every territory and every ISO 3166-2 subdivision it names, 295 and 5 503, in every carried locale, with the locale that answered and the value's draft level; generated by `scripts/place-names-cldr.py` |

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
3. If CLDR names the countries in the language at a release level, add
   its table to `src/territories.rs` and to `territories::TABLES`; if it
   names the zones' exemplar cities, add its table to
   `src/exemplar_cities.rs` and to `exemplar_cities::TABLES`.
   Add it to `LOCALES` in `scripts/place-names-cldr.py` and run the
   script, which writes its territory and subdivision names.

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

* **Vocabulary** follows Unicode CLDR 48 (`common/main/<locale>.xml`, the
  `calendars` sections, tag `release-48`): a subset chosen for calendar
  work. The entries carried before the most-spoken languages are
  **hand-checked**; the twelve added for those languages are **generated**
  from their files by following CLDR's inheritance, as the section on them
  below says. Each entry's
  `LocaleData::sources` names the file it follows; the language's own file
  and CLDR's default values are carried, not the regional files (Arabic is
  `ar.xml`'s يناير…, not the Levantine كانون الثاني… of `ar_SY.xml`). It will
  not track a CLDR release automatically, and it is not a drop-in
  replacement for ICU.
* **Plural rules** follow CLDR 48 `supplemental/plurals.xml`, with the
  operands of UTS #35 Part 3. They are implemented from the published rule
  text, and the tests assert the published sample values, not values
  derived from this implementation.
* **First day of week** follows CLDR 48 `supplementalData.xml`
  `weekData/firstDay`, transcribed in full; only the non-Monday regions are
  tabulated, since CLDR lists Monday for `001` and every region it does not
  name otherwise. A tag's region decides, so `en-GB` starts on Monday and
  `pt-PT` on Sunday; a tag with no region takes its language entry's day,
  which follows the region CLDR 48's likely subtags give the language (for
  `nah`, which has none, the region its entry's comment names): `en` takes
  US's Sunday and `pt` BR's. `und`, which names no language, takes the
  world default, `001`, Monday, and so does a tag with no region whose
  language has no entry and falls to root.
* **Script from region**: a Chinese, Cantonese or Punjabi tag with no
  script takes the one CLDR 48's likely subtags give it, so `zh`, `zh-CN`
  and `zh-SG` resolve to the `zh-Hans` entry and `zh-TW`, `zh-HK` and
  `zh-MO` to `zh-Hant`, `yue` to `yue-Hant` and `yue-CN` to `yue-Hans`, and
  `pa` to `pa-Guru` and `pa-PK` to `pa-Arab`, instead of falling to root's
  `M01`…`M12`.
* **Calendar names** — what a locale calls a calendar,
  `names::calendar_display_name` — are CLDR 48's
  `localeDisplayNames/types/type[@key="calendar"]` at its `approved` and
  `contributed` levels, keyed to the registry, and a calendar CLDR does not
  name in a locale is left unnamed. No two registered calendars share a
  name in a locale, nor in the English names the calendars declare
  themselves (`CalendarMeta::english_name`); the facade's
  `tests/distinct_names.rs` holds every locale to it. Where CLDR's one
  name would serve two calendars, the one CLDR's identifier belongs to keeps
  it and the other carries a qualifier that needs no translation: CLDR's
  `persian` names `persian`, the equinox calendar, and `persian-arithmetic`,
  the 2 820-year cycle, is that name followed by "(2820)", in parentheses as
  the locale's own CLDR names write a qualifier (`ペルシア暦(2820)`,
  `波斯历（2820）`), with ASCII digits in every script. No qualified form is
  translated; English alone spells it out, "Persian Calendar (2820-year
  cycle)". CLDR's `iso8601` likewise names `iso8601`, and `iso8601-week`
  is that name followed by "(W)", ISO 8601's week designator, or in English
  ISO 8601's own "ISO 8601 week date". `data.rs` says so beside the tables.
* **Territory names** — what a locale calls a country,
  `territories::territory_name`, behind the `territories` feature — are
  CLDR 48's `localeDisplayNames/territories`, the plain value (no `alt`
  form) at the `approved` and `contributed` levels, for the 195 regions
  `hc-holiday` keeps a country's table for and no others. Forty-seven
  locales have a table; Coptic, whose every value is unconfirmed, and the
  five locales without a CLDR file have none, and a region a locale does
  not name is left unnamed. Each table is one string of names in the order
  of `territories::REGIONS`, so that about seven thousand names cost their
  text and a line feed rather than a slice each; a macro checks each
  table's codes against `REGIONS` at compile time. The feature is off by
  default, and the facade's `holiday` feature turns it on, because the
  text is about 135 kilobytes that a build rendering only dates
  does not need. The `alt="short"` values, `territories::short_name`, come
  from the same files at the same levels, as (code, name) pairs for the
  few regions each locale shortens — `Hong Kong` for `HK`, `UK` for `GB` —
  and not where the file writes the inheritance marker `↑↑↑`.
* **Exemplar cities** — what a locale calls a time zone's city,
  `exemplar_cities::exemplar_city` — are CLDR 48's
  `dates/timeZoneNames/zone/exemplarCity` at the `approved` and
  `contributed` levels, for the 418 names `hc_tz::location` gives a row,
  each read under CLDR's own identifier for the zone from
  `common/bcp47/timezone.xml` (`Asia/Calcutta` for `Asia/Kolkata`). English
  is `en.xml`'s value, else `root.xml`'s, else the name UTS #35 derives
  from the zone's identifier, its last field with underscores as spaces;
  it is the `exemplar-cities` feature, about 2 kB, which the facade's `tz`
  feature turns on. The forty-one other locales with values are the
  `localized-exemplar-cities` feature, about 230 kB of text. Where a file
  writes the inheritance marker `↑↑↑` at a release level, the table keeps
  it, and the locale answers with the root name under its own tag. Each
  table is one string of lines in the order of `exemplar_cities::ZONES`,
  checked at compile time as the territory tables are.
  `docs/systems/zone-locations.md` explains the lookup.
* **Zone names** — what a locale calls a time zone, `zone_names` — are
  CLDR 48's `dates/timeZoneNames` at the `approved` and `contributed`
  levels, with `supplemental/metaZones.xml`, `bcp47/timezone.xml` and
  `supplemental/likelySubtags.xml`; `scripts/zone-names-cldr.py` generates
  `src/zone_names/cldr48.rs` and checks it (`--check`). The formats are
  always carried, a few hundred bytes a locale, since the localized GMT
  format needs them. The `zone-names` feature, about 60 kB, adds the
  supplemental data and English's names, and turns on `exemplar-cities`
  and `territories`, which a zone's location name is written with;
  `hc-format`'s `zone-names` feature turns it on. `localized-zone-names`,
  some 600 kB of text, adds every other carried locale's names and turns
  on `localized-exemplar-cities`; `hc-format`'s feature of the same name
  turns it on. A name a file writes as CLDR's empty override `∅∅∅` is
  carried as "no name", which stops the lookup rather than inherit the
  parent's. `docs/systems/zone-names.md` explains the composition.
* **Day periods** — `day_periods` — are the format rule set of CLDR 48's
  `supplemental/dayPeriods.xml` for every carried language, the rule sets
  it keys by a language's script or region among them (`hi_Latn`,
  `es_CO`), and each carried locale's format names of midnight, noon and
  the flexible periods; `scripts/day-periods-cldr.py` generates
  `src/day_periods/cldr48.rs` and checks it (`--check`). They are always
  carried. The rules are found by truncating the tag, as the file keys
  them by language: `zh-Hant` takes `zh`'s, which `parentLocales` would
  not reach. `docs/systems/zone-names.md` explains them.
* **Place names** — what a locale calls any territory or ISO 3166-2
  subdivision, `place_names`, behind the `place-names` feature — are
  CLDR 48's `localeDisplayNames/territories` and
  `common/subdivisions/<locale>.xml`, the plain values at the `approved`,
  `contributed` and `provisional` levels, since nearly every subdivision
  name outside English is provisional. `scripts/place-names-cldr.py`
  generates `src/place_names/cldr48.rs` and checks it (`--check`). A
  table keeps a bit per code and a line per name, empty where the name is
  English's; the lookup walks the locale's chain and its CLDR parents,
  then English. Some 2.8 MB of text, so nothing turns the feature on but
  the facade's `place-names` and the boundary crates' `places` layer.
  `docs/systems/place-names.md` explains the lookup.
* **Bidi** follows UAX 9 §2.4 (isolates) and §P2–P3 (first-strong).
* **Casing** follows the default case algorithms of The Unicode Standard 17.0
  §3.13 plus the Turkic tailoring of `SpecialCasing-17.0.0.txt` for `tr` and
  `az`.

Locales shipped: `aeb-Latn am ar ar-EG ayl-Latn ban bn bo cop cs de en en-001
en-GB es es-419 fa fil fr ha he hi id it ja jv kab ko mid mix ml mn mr my nah ne
nl pa-Arab pa-Guru pcm pl ps pt pt-PT rif ru sa shi-Latn sw syr ta te th tr ur
ur-IN vi yua yue-Hans yue-Hant zap zgh zh-Hans zh-Hant zh-Hant-HK`, plus the
`und` root. The regional entries `en-001`, `en-GB`, `es-419`, `zh-Hant-HK`,
`ur-IN` and `ar-EG`, and the languages `mn` and `shi-Latn`, are generated
by `scripts/locales-cldr.py` (`--check`) into `src/data/cldr48_locales.rs`
with CLDR 48's `parentLocales`, which `Locale::parent` follows, and
the default numbering systems (`data::DEFAULT_NUMBERING`): each entry's,
and each regional file's that differs from its language's, so that `ar`
writes Latin digits as `ar.xml` does and `ar-SA`, like `ar-EG` and
nineteen other Arabic regional files, Arabic-Indic ones:
`docs/i18n.md`, "Regional locales", says what each carries. Non-Gregorian vocabulary: Hijri months
(Arabic, English), Hebrew months (Hebrew, English), Babylonian months
(English), the Chinese calendar's months in both Chinese scripts, in
Japanese (正月, 二月 … 十二月) and in Korean, the Tibetan months in Tibetan,
the numbered lunisolar months in English, the Japanese lunisolar calendars'
traditional names (睦月 … 師走, which serve those five calendars and no
other), the Japanese era names 大化 to 令和 in every locale whose CLDR 48
file states them (`ar cs en fa he hi id ja ko nl ru th yue-Hans yue-Hant
zh-Hans zh-Hant`, generated into `src/data/japanese_eras.rs` by
`scripts/japanese-eras-cldr.py`: each file's own names as it writes them,
with the years where it gives them, 만엔 (1860 ~ 1861)), the Ethiopic
months in Amharic, the Coptic months in Coptic and in Egyptian Arabic, the
Burmese months in Burmese, the
Bikram Sambat and Nepal Sambat months in Devanagari, the Hindu lunisolar and
Vikrami solar months in Devanagari (Sanskrit and Hindi), the Indian national
calendar's months in Hindi, Tamil, Malayalam and Bengali, the Tamil,
Malayalam and Bengali solar months in their own scripts, the Assyrian months
in Syriac, the Berber months in Kabyle and in Tifinagh, the Solar Hijri
months in Pashto, the Maya day-signs
and haabʼ months in Yucatec, the Aztec day-signs and months in Nahuatl, the
Zapotec *yza*'s months and year bearers in Zapotec, the
Pawukon cycles in Balinese, the pasaran and dina in Javanese, the Mandaean
weekdays in Mandaic, romanisations of the Coptic, Ethiopic, Burmese,
Khmer, Armenian and Persian months and of Afghanistan's Dari ones (whose own
scripts the calendars carry themselves), and the zodiac animals in Chinese, Japanese, Korean, Vietnamese
and English. The stems and branches are not spelled here: each locale names
one of the readings `hc_calendar::cycle::readings` catalogues.

Twenty-five of the locales exist for a calendar's own language, and they cover
what their sources cover and no more:

| Locale | Calendar | Gregorian vocabulary | Calendar vocabulary | Not carried |
|---|---|---|---|---|
| `am` Amharic | `ethiopic`, `coptic` | CLDR 48 `am.xml` | the thirteen Ethiopic months (CLDR `ethiopic`), the Coptic era abbreviation ዓ/ም (CLDR `coptic`) | Ethiopic era names: CLDR's `am` inherits root's Latin `AA`/`AM` |
| `cop` Coptic | `coptic` | none: CLDR has no `cop`, so it inherits | the thirteen Bohairic months (Wikipedia, "Coptic calendar") | weekdays, day periods, the era: no source read names them |
| `my` Burmese | `burmese` | CLDR 48 `my.xml` | the twelve months (Wikipedia, "Burmese calendar"); First Waso, Second Waso and the late Tagu in Burmese script ([burmese.md](../../docs/systems/burmese.md)) | — |
| `bo` Tibetan | `tibetan`, `tibetan-tsurphu`, `tibetan-lochen`, `tibetan-tsurphu-karana` | CLDR 48 `bo.xml` | the numbered months, CLDR's own ordinal month names keyed to the calendar that numbers its months; ཟླ་ཤོལ་ before a leap month, Henning's *zla shol*; the sixty-year cycle by element, sex and animal, ས་ཕོ་བྱི for 2008, the reading `hc_calendar::cycle::readings::TIBETAN` (Wikipedia, "Tibetan calendar", secondary) | a mark for a doubled day, which no source read writes in a date; the Tibetan calendars' dates do not yet carry the sexagenary field their year names would be written from |
| `ne` Nepali | `bikram-sambat`, `nepal-sambat` | CLDR 48 `ne.xml` | the Bikram Sambat months as the Nepal Rajpatra spells them, the Nepal Sambat months in Devanagari and its intercalary month, अनला (Wikipedia, "Nepal Sambat"); the eras विक्रम संवत् (विसं) and नेपाल सम्वत् (नेसं), as Nepali Wikipedia writes them (secondary) | a `new` (Newar) locale, which CLDR does not have |
| `sa` Sanskrit | `hindu-lunar`, `hindu-lunar-purnimanta`, `hindu-solar-vikrami` | CLDR 48 `sa.xml`, with the ASCII colon it prints for a visarga in the abbreviated months and in Thursday's wide form transcribed as printed | the twelve lunar months in Devanagari as the amānta calendar declares them (Rashtriya Panchang, Sanskrit edition; Wikipedia, "Hindu calendar"), the prefix अधिक (Wikipedia, "Adhik Maas"), the Vikrami solar months from Vaiśākha | the eras (CLDR's default forms are Latin); the rāśi names in Devanagari and the nakṣatras: no source read prints the former, no calendar declares the latter |
| `hi` Hindi | `indian`, `hindu-lunar`, `hindu-lunar-purnimanta`, `hindu-solar-vikrami` | CLDR 48 `hi.xml` | the national calendar's months and era abbreviation शक (CLDR `indian`), which names the Śaka years of the lunisolar calendars too through `data::SHARED_ERAS`; the same twelve names keyed to the lunisolar calendars with the prefix अधिक, and from Vaiśākha to the Vikrami solar calendar | the rāśi names in Devanagari; the nakṣatras |
| `ta` Tamil | `indian`, `hindu-solar-tamil` | CLDR 48 `ta.xml`; the *Tamil Lexicon* | the Tamil months சித்திரை … (CLDR `indian`), serving both calendars, with the era abbreviation சாகா; the sixty year names பிரபவ … அட்சய of `hindu-solar-tamil`, from the Lexicon's entry வருஷம் | day periods: CLDR's `ta` inherits root's |
| `ml` Malayalam | `indian`, `hindu-solar-malayalam` | CLDR 48 `ml.xml` | the national calendar's months in Malayalam and the era abbreviation ശക (CLDR `indian`); the Kollam months ചിങ്ങം … (Wikipedia, "Malayalam calendar") | day periods: CLDR's `ml` inherits root's |
| `bn` Bengali | `indian`, `hindu-solar-bengali`, `bangladeshi` | CLDR 48 `bn.xml` | the twelve months (CLDR `indian`), Chaitra first for the national calendar and Boishakh first for the Bengali year, the same spellings Wikipedia's "Bangladeshi national calendar" prints; CLDR's era abbreviation সাল | day periods: CLDR's `bn` inherits root's |
| `yua` Yucatec Maya | `maya-tzolkin`, `maya-haab`, `maya-round` and their `-gmt2` twins | none: CLDR has no `yua`, so it inherits | the twenty day-signs and nineteen haabʼ months in the sixteenth-century Yucatec spelling the calendar declares (Reingold and Dershowitz; Wikipedia, "Tzolkʼin", "Maya calendar") | the revised orthography, which Wikipedia's "Tzolkʼin" and "Haabʼ" give (after Kettunen and Helmke) but which no BCP 47 subtag tells from the colonial one; weekdays, day periods, eras |
| `nah` Nahuatl | `aztec-tonalpohualli`, `aztec-xiuhpohualli` | none: CLDR has no `nah`, so it inherits | the twenty day-signs and nineteen months as the calendar declares them (Wikipedia, "Tonalpohualli"; Reingold and Dershowitz) | weekdays, day periods, eras |
| `zap` Zapotec | `zapotec-yza` | none: CLDR has no `zap`, so it inherits | the nineteen months of Manuscript 85 and the four year bearers as the calendar declares them (Urcid, *Zapotec Hieroglyphic Writing*, Table 3.5; Tavárez and Justeson 2008, Table 1) | the day-signs of the 260-day count, whose names change with their number; weekdays, day periods, eras |
| `ban` Balinese | `balinese-pawukon` | none: CLDR has no `ban`, so it inherits | all ten Pawukon cycles as the calendar declares them (Reingold and Dershowitz, §10.6); the Saptawara as weekdays | Balinese script: no source read prints it; day periods, eras |
| `jv` Javanese | `javanese-pasaran` | CLDR 48 `jv.xml` | the five pasaran and the seven dina (Wikipedia, "Javanese calendar"), with Monday in the Javanese form Senen the module declares (Javanese Wikipedia and Wiktionary, "Senèn") where that page prints the Indonesian Senin | the Javanese-script forms that page prints, `jv` being a Latin-script locale |
| `syr` Syriac | `assyrian`, `seleucid-syrian` | CLDR 48 `syr.xml` | the twelve Assyrian months in vocalised East Syriac (Wikipedia, "Assyrian calendar"), Neesan first, ܛܲܒܵܚ for Tabakh; the Seleucid year's months, `syr.xml`'s Julian ones, as the Zabad inscription's "Illul" 823 dates by them, and its era ܕܝܲܘܢܵܝܹ̈ܐ, "of the Greeks" (Wikipedia, "Zabad inscription" and "Assyrian calendar", secondary) | the era AY in Syriac: no source read writes it |
| `kab` Kabyle | `berber` | CLDR 48 `kab.xml` | the Gregorian months, which are the agrarian calendar's under the same Latin-derived names (Encyclopédie berbère, "Calendrier"); CLDR spells Fuṛar and Nunembeṛ where the calendar has Furar and Wambeṛ | — |
| `zgh` Standard Moroccan Tamazight | `berber` | CLDR 48 `zgh.xml` | the Gregorian months in Tifinagh, ⵉⵏⵏⴰⵢⵔ …, keyed to the agrarian calendar for the same reason | Kabyle forms in Tifinagh: no source read prints them |
| `ps` Pashto | `persian-afghan`, `persian`, `persian-arithmetic` | CLDR 48 `ps.xml`, less the narrow weekdays and stand-alone narrow months, which resolve to root's Latin letters and numerals | the twelve Solar Hijri months وری … کب (CLDR `persian`, which keys them to its one Solar Hijri calendar and so to all three here), in CLDR's spelling where Wikipedia's "Solar Hijri calendar" has ګ, ي and ك in four | the Solar Hijri era in Pashto: CLDR's `ps` inherits root's; date templates, which `ps.xml` inherits |
| `shi-Latn` Tachelhit | `berber` | CLDR 48 `shi_Latn.xml`, generated | the Gregorian months, innayr …, keyed to the agrarian calendar as Kabyle's are, the Shilha column of Wikipedia's "Berber calendar" | — |
| `rif` Riffian, `aeb-Latn` Tunisian Arabic, `ayl-Latn` Libyan Arabic | `berber` | none: `rif.xml` is unconfirmed throughout and CLDR has no `aeb` or `ayl` | the twelve months as Wikipedia's "Berber calendar" spells them for each (secondary), in its romanisation | the Tunisian and Libyan names in Arabic script, which no source read prints; the Shawiya and Mozabite spellings |
| `mix` Mixtec | `mixtec-year` | none: CLDR has no `mix` | the four year bearers Huiyo, Si, Cuau and Sayu, the first forms of Wikipedia's "Mesoamerican calendars" Mixtec day names, after Caso (secondary) | the language's own name, which no source read gives |
| `mid` Mandaic | `mandaean` | none: CLDR has no `mid`, so it inherits | the seven weekdays in Mandaic script (Wikipedia, "Mandaean calendar") | the months: that page prints the twelve zodiacal names but no Mandaic Parwanaia, and the calendar's month cycle has thirteen positions; day periods, eras |

Ten of the locales carry the languages of Ethnologue's thirty most-spoken
that the others did not, Cantonese in both scripts and Western Punjabi in
Shahmukhi among them; two more are Punjabi in Gurmukhi, whose Eastern
Punjabi is not among the thirty, and European Portuguese (`docs/i18n.md`
lists the thirty and why Egyptian Arabic and Wu are not
among them). Each is its CLDR 48 file, read by following CLDR's own
inheritance: every group of names the file states at a release level, with
its widths resolved through `root.xml`'s aliases, and the file's templates,
calendar names, country names, exemplar cities and, in `hc-humanize`,
relative-time phrases.

| Locale | CLDR 48 file | Calendars named beyond the Gregorian | Not carried |
|---|---|---|---|
| `fil` Filipino (Tagalog) | `fil.xml` | the Minguo eras | — |
| `ha` Hausa | `ha.xml` | the Hijri months | — |
| `mr` Marathi | `mr.xml` | the Buddhist and Minguo eras; the Hijri, Hebrew, Coptic, Ethiopic, Persian and Indian national months; the Hijri, Hebrew and Śaka eras | — |
| `pa-Arab` Punjabi (Shahmukhi) | `pa_Arab.xml` | none | the day periods, calendar names and cities, which the file does not state; its parent is root, not `pa` |
| `pa-Guru` Punjabi (Gurmukhi) | `pa.xml` | the Buddhist and Minguo eras; the Hijri, Hebrew, Coptic, Ethiopic, Persian and Indian national months; the Śaka era | — |
| `pcm` Nigerian Pidgin | `pcm.xml` | none | — |
| `pt-PT` European Portuguese | `pt_PT.xml` over `pt.xml` | the Buddhist era | everything the file does not state itself, which `pt` answers |
| `sw` Swahili | `sw.xml` | none | — |
| `te` Telugu | `te.xml` | the Minguo eras; the Hebrew, Coptic, Ethiopic, Persian and Indian national months; the Śaka era | the Hijri months, whose format names the file leaves to root's Latin ones |
| `ur` Urdu | `ur.xml` | the Minguo eras; the Hijri, Hebrew, Coptic, Ethiopic, Persian and Indian national months; the Hijri and Śaka eras | — |
| `yue-Hans`, `yue-Hant` Cantonese | `yue_Hans.xml`, `yue.xml` | the Buddhist, Japanese, Minguo and Persian eras; the Hijri, Hebrew and Indian national months and eras; the Persian, Coptic and Ethiopic months, which the files number, 1月 to 12月 or 13月; the Chinese and Dangi months and zodiac | the Coptic and Ethiopic eras, which the files do not state, so that those dates write the calendar's own or the English era, apart from the year as the files' `generic` calendar, to which `root.xml` aliases these calendars' date formats, writes it ("G y年"): `Anno Martyrum 1743年1月17日` |

Plural languages: `am ar bn bo cs cy da de en es fa fi fil fr ga ha he hi id it
ja jv kab ko lt lv ml mn mr my nah ne nl pa pcm pl ps pt pt-PT ro ru shi sl sv
sw syr ta te th tl tr uk ur vi yue zh`.
`aeb ayl ban cop mid mix rif sa yua zap zgh` are not in CLDR 48's `plurals.xml` and take
root's rule, `other` for everything.

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
* **No transliteration**, no counting-rod numerals. Hebrew numerals are
  carried, as `hebr`, and Greek alphabetic numerals as `grek` and
  `greklow`, CLDR 48's `%greek-upper` and `%greek-lower`: 2026 is ͵βκϝ´.
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
* Plural categories match the CLDR 48 sample values for the languages
  implemented.
* The vocabulary is as accurate as CLDR and a careful hand-check; where a
  language has forms this crate does not model (finer day periods, Dutch
  `IJ`, Arabic month name variants outside the Levant), it returns the common
  form rather than guessing.
