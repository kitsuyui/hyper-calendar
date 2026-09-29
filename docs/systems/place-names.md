# Place names: what CLDR calls each territory and each ISO 3166-2 subdivision

Backs `hc-i18n::place_names` and `hyper_calendar::place_lines`, and
through them the exports `hc_territories`, `hc_subdivisions` and
`hc_place_name` of the `places` layer. No calendar identifier is
registered: a place name labels a code that another export writes, such
as a subdivision in column 9 of `hc_holiday_tables` or a country in
columns 4 and 5 of `hc_zones`.

## What it is

ISO 3166-1 gives each country a two-letter code, `JP`, and ISO 3166-2
each of a country's subdivisions a code of the country's two letters, a
hyphen and a suffix: "The ISO codes have a region code followed by a
hyphen, then a suffix consisting of 1..3 ASCII letters or digits", so
Tokyo is `JP-13` [uts35-v48, Part 1, Subdivision Codes]. The Unicode
Common Locale Data Repository (CLDR) names both kinds of place in every
language it covers, in two sets of files:

- `common/main/<locale>.xml`, `localeDisplayNames/territories`, names the
  *territories*: the countries, the UN M.49 areas such as `001` (the
  world) and `419` (Latin America), and a few groupings of CLDR's own,
  `EU`, `EZ`, `UN`, `QO`, the pseudo-locales' `XA` and `XB`, and `ZZ`, the
  unknown region [cldr48-territory-names].
- `common/subdivisions/<locale>.xml` names the *subdivisions*, one
  `<subdivision type="jp13">` element each [cldr48-subdivision-names].
  "The CLDR codes are designed to work in a unicode_locale_id (BCP 47),
  and are thus all lowercase, with no hyphen" [uts35-v48, Part 1,
  Subdivision Codes].

CLDR keeps codes ISO has withdrawn: "If an ISO 3166-2 code is removed, it
remains valid in CLDR, though marked as deprecated", and each code's
status is in `common/validity/region.xml` and `subdivision.xml`
[cldr48-validity]. CLDR also mints codes of its own, which "may start with
a 3-digit region code or use a suffix of 4 ASCII letters or digits, so
they will not collide with the ISO codes", `uszzzz` for an unknown
subdivision of the United States among them [uts35-v48, Part 1,
Subdivision Codes].

The names are of uneven standing, and CLDR says so. "CLDR provides vetted
name data for only a few of the thousands of ISO subdivisions; other names
are extracted from various sources, including Wikidata" [uts35-v48,
Part 1, Subdivision Codes]. Each value carries a `draft` attribute:
`approved`, "fully approved by the technical committee"; `contributed`,
"partially approved"; `provisional`, "partially confirmed.
Implementations may choose to accept the provisional data, especially if
there is no translated alternative"; and `unconfirmed`, "no confirmation
available" [uts35-v48, Part 1, Attribute draft].

A locale that has no value of its own inherits one. The lookup of one item
runs from the locale through its parents to root, `nb-NO → nb → root`,
and "if there is no value for P there, we return the value for P in root
(or a code value, if there is nothing there)"; for identifiers such as
region codes, "the default value is the identifier itself whenever no
value is found in the root". A locale's parent is the locale with its last
subtag cut, unless `parentLocales` in `supplementalData.xml` names another
[cldr48-supplemental]; there `zh_Hant`, `yue_Hans` and `pa_Arab` have root
as their parent, since their script is not their language's usual one. A
value written as the marker `↑↑↑` is the parent's: implementations
"produce the inherited value whether the element is present with a value
of ↑↑↑, or is completely absent". And before root, an implementation may
try fallback locales from language matching; the example's list for
`nb-NO` ends in English, `[nn da sv en]`, looked up in turn "returning the
first value that is not found in root" [uts35-v48, Part 1, Inheritance
and Validity].

## How it works

### A code

CLDR's id `jp13` is written as ISO writes it by putting the first two
letters in upper case, a hyphen, and the rest in upper case: `jp13` →
`JP-13`, `gbsct` → `GB-SCT`, `fr75c` → `FR-75C`. Every id CLDR 48 names
fits the pattern the rule needs, two letters and one to three letters or
digits; an id of CLDR's own, with four characters after the region or a
numeric region, does not, and the generator stops on one. None of CLDR
48's names is keyed by one.

### A name

The name of a place in a locale, `JP-13` under `ja-JP`:

1. **The first table.** The locale's chain is walked until a carried
   locale has a table: `ja-JP` → `ja`. `zh-TW` walks `zh-Hant-TW`, then
   `zh-Hant`; `de-AT` finds `de`.
2. **The table, then its parents.** If the table's files name the code at
   the draft levels carried, that is the answer, under the table's tag:
   `subdivisions/ja.xml` has `<subdivision type="jp13"
   draft="provisional">東京都</subdivision>`, so the answer is 東京都, `ja`,
   `provisional`. Otherwise the lookup goes on to the table's CLDR parent,
   where the parent is a carried locale too. Only `pt-PT` has one: its
   parent is `pt`, and `subdivisions/pt_PT.xml` writes its three values as
   `↑↑↑`, so `JP-13` under `pt-PT` is `pt.xml`'s Tóquio, answered by `pt`.
3. **English.** A place no table of the chain names is named as `en.xml`
   names it, answered by `en`. `zh_Hant`'s parent is root, and
   `subdivisions/zh_Hant.xml` names only England, Scotland and Wales, so
   `JP-13` under `zh-TW` is `Tokyo`, answered by `en`, although `zh.xml`
   has 東京都.
4. **Nothing.** A place English does not name either has no name, and the
   caller holds its code, which is CLDR's own last resort. `FR-75`, which
   CLDR 48 holds as deprecated beside the regular `FR-75C` that `en.xml`
   names Paris, is named by Hausa alone among the carried locales
   (`Pariis`), so under `ja` it has none.

A locale whose file names a place with English's own spelling answers
under its own tag: `US-CA` under `es` is `California`, answered by `es`,
because `subdivisions/es.xml` names it so.

### A table

A table stores only what differs from English, in two parts. The
subdivisions are one list of 5 503 codes in code order, `JP-13` at index
2 546. A table has one bit per code, set where its files name the code,
and a line of text per set bit, in code order, holding the name, or
nothing where the name is English's. The line of the code at index *i* is
the number of bits set below *i*: the `ja` table names 2 033 codes before
`JP-13`, so `JP-13`'s line is its 2 034th, 東京都; the `es` table names
2 218, and the 2 219th line is `Tokio`. `es`'s line for `US-CA` is empty,
and English's `California` fills it. A table's usual draft level is stored
once and the others by index: every name of `ja` is provisional but
England's, Scotland's and Wales's, which are approved.

## What is carried

- **The places.** The 295 territories `en.xml` names, and the 5 503
  subdivisions a carried locale's file names: the 5 027 codes CLDR's
  validity data holds as `regular`, every one of which `en.xml` names, and
  476 it holds as `deprecated`, 104 of which English does not name. Each
  code carries its status: `regular`, `deprecated`, `macroregion` (`001`,
  `419`, `EU`), `special` (`XA`, `XB`) or `unknown` (`ZZ`).
- **The names**, from CLDR 48 at the draft levels `approved`,
  `contributed` and `provisional`, the plain value of each (no `alt`
  form), with the level each was read at. Of the 121 765 subdivision names
  in the carried locales other than English, 121 655 are provisional, 109
  approved (England's, Scotland's and Wales's in each file that names
  them) and one contributed. Nearly every territory name is approved.
  Forty-seven of the 65 carried locales have a table (the regional and
  added entries of `docs/i18n.md` not yet): 47 name territories
  and 39 name subdivisions, `fil` and `zh-Hant` only England, Scotland and
  Wales. Tibetan, Kabyle, Punjabi in the Arabic script, Nigerian Pidgin,
  European Portuguese (whose three values are the marker), Sanskrit,
  Syriac and Standard Moroccan Tamazight name no subdivision; Coptic's every
  value is unconfirmed; Balinese, Middle Egyptian, Nahuatl, Yucatec Maya
  and Zapotec have no CLDR file. Their places are named in English.
- **Sizes**, measured on 2026-09-28. The names are 2 576 858 bytes of
  subdivision text (127 164 names, English's 5 399 included) and 221 116
  of territory text (12 636 names), with the line feeds; storing a name
  equal to English's as an empty line keeps 22 725 subdivision names and
  1 617 territory names out. The bits are 25 867 and 1 730 bytes, the
  codes 33 018 and 885. The `places` layer of the WebAssembly module is
  2 905 669 bytes, 830 575 gzipped (`gzip -9`) and 619 938 with Brotli
  (`-q 11`); every other layer is unchanged, and `full` grows by it.
  Sharing a string across locales was measured and is not done: a line
  that pointed to another table's identical name for the same code would
  save 78 429 bytes of the text (2.8 %), and 31 879 of it gzipped, but
  nothing once the text is compressed as LZMA compresses it (594 548 bytes
  against 595 456 with the pointers), since such a compressor finds the
  repeats itself.
- **Not carried.** The `alt="short"` and `alt="variant"` territory names:
  `hc_holiday_tables` gives the short names of its 195 countries from
  `hc-i18n`'s `territories` module, and this data does not repeat them yet.
  The `unconfirmed` values. The fallback locales that language matching
  would add between a locale and English, such as Simplified Chinese for
  Traditional: TR35 allows them, and only English is taken so far. The
  locales CLDR has and `hc-i18n` does not carry, among them CLDR's regional
  English (`en_GB`, `en_001`). CLDR's subdivision containment
  (`supplemental/subdivisions.xml`), which says which subdivisions lie in
  which. A replacement for a deprecated code: `supplementalMetadata.xml`'s
  `subdivisionAlias` gives one for 147 deprecated codes, none of which a
  carried file names; the aliases of the 476 that are named are commented
  out, with the replacement `al?` and the like [cldr48-validity]. ISO
  3166-2 itself, which was not read: a code ISO lists that CLDR 48 does
  not is not carried.

### Beside the holiday tables and the zones

Column 9 of `hc_holiday_tables` lists the subdivisions a table's rules
are scoped to, `JP-01;JP-07;…;JP-47` for the nineteen prefectures with
days of their own, and `hc_holidays_on` writes the one an entry is for in
its column 10. Every such code has a line here, and every
country with a table has a territory line; a test in `place_lines` holds
the lists to each other. Two of the codes the tables use are `deprecated`
in CLDR 48: `GB-EAW`, England and Wales, which English and most carried
locales still name, and `GT-GU`, the department of Guatemala, to which
the table scopes Guatemala City's festivity, and which only `fr.xml` names
among the carried locales (département de Guatemala). CLDR 48 holds
`GT-01`, which `en.xml` names Guatemala, as regular; no alias links the
two. A page shows `GT-GU` by its code until the holiday table's code is
revisited.

The countries a holiday table's line names in column 3 come from
`hc-i18n`'s smaller `territories` module, which the `holiday` feature
carries: the 195 countries with a table, at the `approved` and
`contributed` levels only. Where it names a country, this data names it
the same, which a test checks; this data also names the other territories,
and the provisional names the smaller module leaves out (Kabyle's and
Tamazight's name for the British Indian Ocean Territory).

`hc_zones` gives each zone's countries by ISO 3166-1 code, and this data
names those. A zone's exemplar city (column 7) is a different CLDR data
set, `timeZoneNames`, described in [zone-locations.md](zone-locations.md):
Tokyo the city of `Asia/Tokyo` is not the subdivision `JP-13`, though
their names agree in some languages.

## Accuracy

- **The names are CLDR's as published.** Nothing is corrected. The
  subdivision names outside English are provisional in CLDR's own terms,
  drawn "from various sources, including Wikidata", and the draft column
  says so on every line. They disagree with each other where the sources
  do: CLDR notes that "In November 2020 almost all subdivisions of Iran
  were renumbered. For example, IR-25, which used to represent the Yazd
  province, now represents the Qom province" [uts35-v48, Part 1,
  Subdivision Codes], and CLDR 48 names `IR-25` Yazd in English, German
  and Japanese (ヤズド州) but 古姆省, Qom, in Cantonese. A page that labels an
  Iranian code should expect such a mismatch until CLDR's planned stable
  codes for Iran arrive.
- **The lookup is CLDR's**, measured two ways: the tests read every table
  in one pass and one code at a time, and get the same answers; and the
  tables' parents, read from `parentLocales`, agree with the chains
  `hc-i18n`'s `Locale::fallback` walks.
- **The generator is checked against CLDR.**
  `python3 scripts/place-names-cldr.py --check` reads the release again
  and fails when the generated file differs.
- The test anchors: `JP-13` is 東京都 in `ja` and Tokyo in `en`; `DE-BY` is
  Bayern in `de` and Bavière in `fr`; `US-CA` is Kalifornien in `de`,
  Californie in `fr` and California, under `es`, in `es`; `JP-13` under
  `zh-TW`, Swedish and `native` falls back to English, under `pt-PT` to
  `pt`; `FR-75` has no name in `ja`; `BH` is Barém in `pt-PT` and Barein
  in `pt`.

## Sources

- [cldr48-territory-names] — `common/main/<locale>.xml`,
  `localeDisplayNames/territories`, for the carried locales and English.
  Read.
- [cldr48-subdivision-names] — `common/subdivisions/<locale>.xml` for the
  carried locales and English, and `root.xml`, which names nothing. Read.
- [cldr48-validity] — `common/validity/region.xml` and `subdivision.xml`
  for each code's status, and `supplemental/supplementalMetadata.xml`'s
  `territoryAlias` and `subdivisionAlias`. Read.
- [cldr48-supplemental] — `supplementalData.xml`'s `parentLocales`. Read.
- [uts35-v48] — Part 1: Subdivision Codes, Attribute draft, Inheritance
  and Validity (the lookup, the marker `↑↑↑`, the identifier as the last
  resort and the fallback locales before root), and Parent Locales. Read.
- ISO 3166-1 and ISO 3166-2, the standards the codes come from. Not read:
  CLDR's validity data stands in for them, as the list of codes and their
  status.

## Code

- `scripts/place-names-cldr.py` — reads the files above and writes
  `crates/hc-i18n/src/place_names/cldr48.rs`; `--check`, `--dump` and
  `--stats`.
- `crates/hc-i18n/src/place_names.rs` — the lists, the tables and the
  lookup, behind the `place-names` feature; the tests
  `tokyo_is_tokyo_to_in_japanese_and_tokyo_in_english`,
  `a_german_and_an_american_subdivision_in_two_locales`,
  `a_name_falls_back_along_cldrs_chain_then_to_english`,
  `territories_follow_the_same_chain`,
  `a_deprecated_code_only_one_locale_names`,
  `a_pass_and_a_lookup_agree`, `every_table_is_consistent`,
  `a_tables_parent_is_the_next_table_of_its_chain` and
  `the_holiday_territory_names_are_these`.
- `crates/hyper-calendar/src/place_lines.rs` — the lines of the three
  exports; the tests `a_line_is_six_cells` and
  `every_place_a_holiday_table_names_has_a_line`.
- `crates/hyper-calendar-wasm/src/places.rs`,
  `crates/hyper-calendar-ffi/src/places.rs` — the `places` layer of each
  boundary, and their tests `tests/places.rs`; the JavaScript binding's
  `territories`, `subdivisions` and `placeName`.
