# Place names: what CLDR calls each territory and each ISO 3166-2 subdivision

Backs `hc-i18n::place_names` and `hyper_calendar::place_lines`, and
through them the exports `hc_territories`, `hc_subdivisions` and
`hc_place_name` of the `places` layer. It is also the one source of a
country's name elsewhere: the holiday tables' names of their countries
(columns 3 and 8 of `hc_holiday_tables`) and the country in a zone's
location name (`VVVV`, *Italy Time*, in `hc_zone_name`) are read from its
territory half. No calendar identifier is registered: a place name labels
a code that another export writes, such as a subdivision in column 9 of
`hc_holiday_tables` or a country in columns 4 and 5 of `hc_zones`.

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
and Validity]. Those locales come from *language matching*: "The language
matching data can be used to get the closest fallback locales (of those
supported) to a given locale", as French is "the best fallback" for a
Breton reader, and "The locales in the fallback list are not used
recursively" [uts35-v48-matching].

A territory may have alternative names beside its plain one, each an
`alt` attribute: `short` (*Hong Kong* beside *Hong Kong SAR China*),
`variant` (*Czech Republic* beside *Czechia*), and for the British Indian
Ocean Territory `biot` and `chagos`. "If a variant value is absent for a
particular locale, the normal value is used" [uts35-v48-matching, Attribute
alt]. And `supplemental/subdivisions.xml` says which subdivisions lie in
which: `GB` contains England, Northern Ireland, Scotland and Wales, and
England contains Kent [cldr48-subdivision-containment].

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
   where the parent is a carried locale too. `pt-PT`'s is `pt`, and
   `subdivisions/pt_PT.xml` writes its three values as `↑↑↑`, so `JP-13`
   under `pt-PT` is `pt.xml`'s Tóquio, answered by `pt`. The regional
   tables `en-001`, `es-419`, `ur-IN` and `zh-Hant-HK` have their
   language's table as parent, and `en-GB`, whose file names nothing,
   reaches `en-001` by the locale's own chain: `KN` under `en-GB` is
   `en_001.xml`'s *St Kitts & Nevis*, where `en.xml` has *St. Kitts &
   Nevis*.
3. **The language-matching fallbacks.** The first table lists the tables
   language matching makes its fallbacks, nearest first, and each is read
   with its own parents, a table already read left out: Tibetan's is
   Simplified Chinese, so `JP-13` under `bo`, which `subdivisions/bo.xml`
   does not name, is `zh.xml`'s 東京都, answered by `zh-Hans`. Below.
4. **English.** A place no table of the chain names is named as `en.xml`
   names it, answered by `en`. `zh_Hant`'s parent is root, and
   `subdivisions/zh_Hant.xml` names only England, Scotland and Wales, so
   `JP-13` under `zh-TW` is `Tokyo`, answered by `en`, although `zh.xml`
   has 東京都.
5. **Nothing.** A place English does not name either has no name, and the
   caller holds its code, which is CLDR's own last resort. `FR-75`, which
   CLDR 48 holds as deprecated beside the regular `FR-75C` that `en.xml`
   names Paris, is named by Hausa alone among the carried locales
   (`Pariis`), so under `ja` it has none.

A locale whose file names a place with English's own spelling answers
under its own tag: `US-CA` under `es` is `California`, answered by `es`,
because `subdivisions/es.xml` names it so.

A caller may ask for CLDR's release levels only, `approved` and
`contributed`: a provisional value is then passed over as if its file had
none, and the lookup goes on. Kabyle's `IO` is provisional in `kab.xml`,
so at those levels it is English's *British Indian Ocean Territory*. The
holiday tables and the zone names ask so, as they ask for every other
CLDR value they carry.

### A fallback locale

TR35's distance between two locales, for Tibetan and Simplified Chinese:

1. **Maximize.** `likelySubtags.xml` makes `bo` `bo_Tibt_CN` and `zh`
   `zh_Hans_CN` [cldr48-language-matching].
2. **The language.** `bo` and `zh` differ; the first rule of
   `languageInfo.xml`'s `written_new` list that matches is `desired="bo"
   supported="zh" distance="20" oneway="true"`: 20.
3. **The script.** `Tibt` and `Hans` differ; `desired="bo_Tibt"
   supported="zh_Hans" distance="10"`: 10.
4. **The region.** `CN` and `CN` are the same: 0. The distance is 30.

TR35 leaves the threshold to the implementation, "typically set to greater
than a default region difference, and less than a default script
difference" [uts35-v48-matching]: the defaults are `*_*_*`, 4, and `*_*`,
50, and a carried locale less than 50 away is a fallback. So Simplified
Chinese is Tibetan's, and Traditional Chinese's is not, since `zh_Hant`
against `zh_Hans` meets no rule but `*_*`, 50; `zh-Hant`'s is `zh-Hant-HK`
(`zh_Hant_*`, 5). A rule that is not `oneway` matches either way. A
matching variable's regions, `$americas` for `019`, are expanded through
`territoryContainment`, groupings and the regions they contain included, so
that `419` is one of them. Each list ends before English, which the lookup
takes last in any case, as TR35's example list ends in `en`. The lists,
from `scripts/place-names-cldr.py --fallbacks`:

| Table | Fallbacks (distance) |
| --- | --- |
| `bo` | `zh-Hans` (30) |
| `en` | `en-001` (5) |
| `es` | `es-419` (5) |
| `jv` | `id` (20) |
| `mn` | `ru` (34) |
| `mr`, `sa` | `hi` (30) |
| `pt` | `pt-PT` (5) |
| `ur` | `ur-IN` (4) |
| `yue-Hans` | `zh-Hans` (10) |
| `yue-Hant` | `zh-Hant-HK` (10), `zh-Hant` (14) |
| `zh-Hant` | `zh-Hant-HK` (5) |

Many languages are 30 from English and 44 in all (`am` ⇒ `en`, `am_Ethi`
⇒ `en_Latn`, and the region default), which English's place at the end
already covers. The list is the first table's, the one the requested
locale's chain reaches: `zh-TW` takes `zh-Hant`'s.

### An alternative form

`HK`'s short name under `es-419`: the lookup reads the same tables as for
the plain name, in runs, a locale's own table and its parents first, then
each fallback's, then English's. In a run, the first table that gives the
form answers, a parent's among them, as CLDR resolves each path apart:
`es_419.xml` gives no short `HK`, but `es.xml`, its parent, does. `GB`'s
short name under `es-419` is `es_419.xml`'s own *R. U.*, where `es.xml`
has *RU*. `pt_PT.xml` gives `GB` the short name `GB` beside a plain name it
inherits from `pt.xml`, and writes `PS`'s short name as `↑↑↑`, which
inherits `pt.xml`'s *Palestina*. The first run whose tables give the plain
name but not the form ends the lookup with no form, since the plain name
stands for it there: `bo.xml` names `GB` and gives it no short form, so
Tibetan has none, rather than English's *UK*. A locale with no table,
Yucatec Maya's, reaches English's run, and *UK*.

### Which lies in which

`subdivisions.xml` lists each code in a group: `<subgroup type="GB"
contains="gbeng gbnir gbsct gbwls"/>`, and `gbeng`'s group contains
`gbken`. So Kent, `GB-KEN`, is within `GB-ENG`, and `GB-ENG` within `GB`;
`JP-13` is within `JP`. The file lists 5 046 codes, 3 590 directly within
a country and 1 456 within another subdivision; 19 of them are deprecated
codes no carried locale names (`usgu`, Guam, and French and Dutch overseas
codes among them), which are left out, so that every regular subdivision,
5 027, is within something. A deprecated code the file does not list, such
as `FR-75`, is within nothing.

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

### A municipality

CLDR names ISO 3166-2 subdivisions and no smaller place. A holiday table
scopes a day to a municipality by a code under its subdivision's, the
subdivision's code, a hyphen and the code within it in the country's own
standard (ADR 0014): 川崎市 is `JP-14-130`, Kanagawa's `JP-14` and JIS X
0402's 14130 [jis-x0402-cities]. Column 9 of `hc_holiday_tables` lists such
a code beside its subdivision, and CLDR has no line for it. The names
therefore live in a second, small module, `hc-i18n::municipal_names`, in
the same shape as `holiday_names`: one table per tag, each a list of the
code and the name, in code order, and a lookup that walks the locale's
fallback chain and then English. The three tags are `ja`, the name as the
city's own instruments write it, 川崎市; `ja-Latn`, its Hepburn romanisation
with the suffix, `Kawasaki-shi`; and `en`, the English name,
`Kawasaki`. `hc_place_name` writes a municipality's line in the columns of
every place's, with no draft level, since the level is CLDR's, and the
status `municipal`; `hc_subdivisions` writes it in code order after its
subdivision, `JP-14`, `JP-14-100`, `JP-14-130`, `JP-14-150`, `JP-15`, so
that the lookup that names column 9's prefectures names its cities too. A
locale no table names, `de` or `zh-Hans`, gets `en`.

## What is carried

- **The places.** The 295 territories `en.xml` names, and the 5 503
  subdivisions a carried locale's file names: the 5 027 codes CLDR's
  validity data holds as `regular`, every one of which `en.xml` names, and
  476 it holds as `deprecated`, 104 of which English does not name. Each
  code carries its status: `regular`, `deprecated`, `macroregion` (`001`,
  `419`, `EU`), `special` (`XA`, `XB`) or `unknown` (`ZZ`).
- **The names**, from CLDR 48 at the draft levels `approved`,
  `contributed` and `provisional`, the plain value of each, with the level
  each was read at. Of the 122 577 subdivision names in the carried
  locales other than English, 122 464 are provisional, 112 approved
  (England's, Scotland's and Wales's in each file that names them) and
  one contributed. Of the 13 306 territory names, 13 297
  are approved, five contributed and four provisional. Fifty-three of the
  65 carried locales have a table, every one of which names territories,
  and 40 name subdivisions, `fil` and `zh-Hant` only England, Scotland and
  Wales. The regional tables (`en-001`, `es-419`, `pt-PT`, `ur-IN`,
  `zh-Hant-HK`) name only what their files state apart from their
  parents'; `en_GB.xml` and `ar_EG.xml` state no territory name of their
  own, so `en-GB` and `ar-EG` have none and reach `en-001` and `ar`.
  Tibetan, Kabyle, Punjabi in the Arabic script, Nigerian Pidgin, Sanskrit,
  Tachelhit in the Latin script, Syriac and Standard Moroccan Tamazight
  name no subdivision, nor do the regional tables; Coptic's every value is
  unconfirmed, and so is every one of Riffian's (`rif.xml`); Balinese,
  Middle Egyptian, Mixtec, Nahuatl, Tunisian and Libyan Arabic in the Latin
  script, Yucatec Maya and Zapotec have no CLDR file. Their places are named
  in English, or in a fallback's language where language matching gives
  one.
- **The alternative forms**, 581 of them, at the same levels: 214 `short`,
  329 `variant`, 36 `chagos` and 2 `biot` (English's and Syriac's). All
  are approved but one, contributed. A form written as the marker `↑↑↑` is
  absent, so that the lookup goes on to the table's parent. CLDR gives no
  subdivision an `alt` form, and the generator stops on one.
- **The containment** of 5 027 subdivisions: every regular one, 3 571
  directly within its country and 1 456 within another subdivision.
- **The fallbacks** of the eleven tables in the table above.
- **The municipalities**, 21: the twenty designated cities of Japan,
  `JP-01-100` 札幌市 to `JP-43-100` 熊本市, and 長崎市, `JP-42-201`, the
  codes the holiday tables of `docs/systems/japan-holidays.md` list, each
  in three tags. The `ja` names are the names of the instruments the
  table cites for the cities [jp-city-designated] and of JIS X 0402's list
  [jis-x0402-cities]. The `en` names are English Wikipedia's list of the
  designated cities [wikipedia-en-designated-cities] and, for 長崎市, its
  article's title. The `ja-Latn` names are the Hepburn forms the leads of
  the English articles print, macrons included, for nineteen
  [wikipedia-en-city-leads] — `Kyōto-shi`, `Ōsaka-shi`, `Kōbe-shi`,
  `Kitakyūshū-shi` — and, for 札幌市 and 横浜市, whose leads print none,
  the Hepburn forms of the readings the Japanese articles print,
  さっぽろし and よこはまし [wikipedia-ja-city-readings]. A test holds
  every municipality any table lists, in column 9 or in column 13 of
  `hc_holiday_tables`, to a name in all three tags: a table that lists a
  municipality `municipal_names` does not carry fails the build.
- **Two halves.** The territories, their names and forms, and each
  table's parent and fallbacks, are `hc-i18n`'s `territories` feature,
  which the facade's `holiday` feature and `hc-format`'s `zone-names` turn
  on: the holiday tables and the zone names read a country's name from it.
  The subdivisions, their names and their containment are the
  `place-names` feature on top, which only the `places` layer turns on.
- **Sizes**, measured on 2026-09-29 (the names) and 2026-10-03 (the layers). The names are 2 594 289 bytes of
  subdivision text (127 976 names, English's 5 399 included) and 232 003
  of territory text (13 306 names), with the line feeds; storing a name
  equal to English's as an empty line keeps 22 725 subdivision names and
  1 618 territory names out. The bits are 26 551 and 1 948 bytes, the
  codes 33 018 and 885, the containment 5 824 bytes of pairs and 688 of
  bits. The `places` layer of the WebAssembly module is 2 968 604 bytes,
  860 901 gzipped (`gzip -9`) and 637 841 with Brotli (`-q 11`). The
  territories, with their forms, are carried once, in `hc-i18n`'s
  `territories` feature, which the `holiday` layer (2 435 012 bytes) and
  `zone-names` (1 651 483, the 184-day rule of `docs/systems/zone-names.md`
  among it) read their countries' names from; `full` is 7 959 092 bytes.
  Sharing a string across locales was measured on 2026-09-28 and is not
  done: a line that pointed to another table's identical name for the
  same code would save 78 429 bytes of the text (2.8 %), and 31 879 of it
  gzipped, but nothing once the text is compressed as LZMA compresses it
  (594 548 bytes against 595 456 with the pointers), since such a
  compressor finds the repeats itself.
- **Not carried.** The `unconfirmed` values. The locales CLDR has and
  `hc-i18n` does not carry, and the regional files beyond the six regional
  entries `docs/i18n.md` lists. The `nonlikelyScript` parent rule beyond
  the tags `parentLocales` lists, as for the other locale data: a tag such
  as `ja-Latn` walks to `ja`, not root. A replacement for a deprecated
  code: `supplementalMetadata.xml`'s `subdivisionAlias` gives one for 147
  deprecated codes, none of which a carried file names; the aliases of the
  476 that are named are commented out, with the replacement `al?` and the
  like [cldr48-validity]. The containment of territories
  (`territoryContainment`, which says `JP` is in `030`, Eastern Asia): read
  only to expand the matching variables. ISO 3166-2 itself, which was not
  read: a code ISO lists that CLDR 48 does not is not carried.

### Beside the holiday tables and the zones

Column 9 of `hc_holiday_tables` lists the subdivisions a table's rules
are scoped to, `JP-01;JP-07;…;JP-47` for the nineteen prefectures with
days of their own, and `hc_holidays_on` writes the one an entry is for in
its column 10. Every such code has a line here, and every
country with a table has a territory line; a test in `place_lines` holds
the lists to each other. One of the codes the tables use is `deprecated`
in CLDR 48: `GB-EAW`, England and Wales, which English and most carried
locales still name. A municipality's code, `JP-14-130`, is not CLDR's and
has the line of the section "A municipality", status `municipal`; a test
holds every one a table lists to a name. Guatemala City's festivity is scoped to the department
of Guatemala as `GT-01`, which `en.xml` names Guatemala and CLDR 48 holds
as regular; its old code, `GT-GU`, is deprecated, and ISO 3166-2 replaced
it with `GT-01` on 25 November 2021, when it renumbered all twenty-two
departments (Wikipedia, "ISO 3166-2:GT", the change of 2021-11-25 on the
Online Browsing Platform, retrieved 2026-09-29). No CLDR alias links the
two codes, so a caller holding the old one must map it.

The name a holiday table's line gives its country in column 3, and the
short name in column 8, are this data's, at the release levels: the
territory half is the one source, and `hc-i18n` keeps no second list of
countries. Until 2026-09-29 a smaller module, `territories`, carried the
195 countries with a table at those levels, from a one-off script; its
every name was this data's from the same table, which a test checked, and
the unified lookup gives the same names but where the new tables and the
fallbacks answer: `mn` and `shi-Latn`, which had no table, name their
countries; `en-GB`, `es-MX`, `zh-HK` and `ur-IN` take their regional
file's names (*St Kitts & Nevis*, *Rumania*, 阿拉伯聯合酋長國); Tibetan and
Sanskrit take Simplified Chinese's and Hindi's where their files name
nothing, before English; and `pt-PT`'s short name for `GB` is `pt_PT.xml`'s
`GB`, and for `PS` the *Palestina* its marker inherits.

`hc_zones` gives each zone's countries by ISO 3166-1 code, and this data
names those, as it names the country in a zone's location name, where the
country's short form comes first, as UTS #35 has it: *Puerto Rico Time*
for `America/Puerto_Rico` and *Faroe Islands Time* for `Atlantic/Faroe`,
which read *PR Time* and *FO Time* while the smaller module named only the
195. A zone's exemplar city (column 7) is a different CLDR data set,
`timeZoneNames`, described in [zone-locations.md](zone-locations.md):
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
- **The municipal names are Wikipedia's and the instruments'**, not
  CLDR's, and carry no draft level. English Wikipedia is a secondary
  source; the page fetches were summaries of each page, not its text, so
  a macron of `ja-Latn` is as the summary gave it. JIS X 0402 itself was
  not read. Chinese and Korean names, which no source read prints for
  these cities but as a language link's title, and the kana readings of
  the other nineteen, are not carried.
- **The lookup is CLDR's**, measured two ways: the tests read every table
  in one pass and one code at a time, and get the same answers; and the
  tables' parents, read from `parentLocales`, agree with the chains
  `hc-i18n`'s `Locale::fallback` walks.
- **The fallbacks are a reading of TR35.** The distance is TR35's
  algorithm on CLDR's rules; the threshold, 50, is this library's choice
  within the range TR35 gives, and ending each list at English is its
  reading of the example list. A fallback puts a name in another language
  before English's: Tibetan's missing names are Chinese, Sanskrit's Hindi.
  That is what CLDR's matching data says a reader of the one understands
  best; a caller that wants English instead asks for `en`.
- **The countries' names are this lookup's alone.** A holiday table's
  name, in every carried locale and ten other tags, is the name this
  lookup gives its country at the release levels, with a new table or a
  fallback answering where CLDR has none, as the section above lists;
  there is no second list of the 195 countries' names.
- **The generator is checked against CLDR.**
  `python3 scripts/place-names-cldr.py --check` reads the release again
  and fails when the generated file differs.
- The test anchors: `JP-13` is 東京都 in `ja` and Tokyo in `en`; `DE-BY` is
  Bayern in `de` and Bavière in `fr`; `US-CA` is Kalifornien in `de`,
  Californie in `fr` and California, under `es`, in `es`; `JP-13` under
  `zh-TW`, Swedish and `native` falls back to English, under `pt-PT` to
  `pt`; `FR-75` has no name in `ja`; `BH` is Barém in `pt-PT` and Barein
  in `pt`. `JP` is Япон in `mn` and lyaban in `shi-Latn`; `KN` is St Kitts
  & Nevis in `en-GB`, `RO` Rumania in `es-MX`, `AE` 阿拉伯聯合酋長國 in
  `zh-HK`. `HK`'s short form is Hong Kong in `en`, 香港 in `ja`, Hongkong
  in `de-AT`; `GB`'s is R. U. in `es-419`, `GB` in `pt-PT` and none in
  `bo`; `PS`'s is Palestina in `pt-PT`, from `pt`; `IO`'s `biot` and
  `chagos` forms are English's. `AD` is 安道尔 in `bo`, from `zh-Hans`, and
  एंडोरा in `sa`, from `hi`; `JP-13` in `bo` is 東京都 from `zh-Hans`, and
  Kent in `mn` Кент from `ru`. Kabyle's provisional `IO` gives way to
  English's at the release levels. Kent is within England, England within
  `GB`, whose four are England, Northern Ireland, Scotland and Wales; Japan
  has 47; `FR-75` is within nothing. In the holiday tables, Tibetan's
  `FR` is 法国, from `zh-Hans`.

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
- [cldr48-language-matching] — `supplemental/languageInfo.xml`'s
  `languageMatching`, `likelySubtags.xml` and `supplementalData.xml`'s
  `territoryContainment`, for the fallbacks. Read, 2026-09-29.
- [cldr48-subdivision-containment] — `supplemental/subdivisions.xml`.
  Read, 2026-09-29.
- [jis-x0402-cities] — the cities' 全国地方公共団体コード, which give a
  municipality its code under its prefecture's. Read, 2026-09-29, in
  Wikipedia; JIS X 0402 itself not read.
- [jp-city-designated] — the twenty designated cities' and 長崎市's
  instruments, which spell each city's name. Read, 2026-09-29, in each
  例規集 or the archive's copy.
- [wikipedia-en-designated-cities] — the English names of the twenty
  designated cities. Read, 2026-10-03.
- [wikipedia-en-city-leads] — the Hepburn romanisation of 19 cities' names
  with their suffix. Read, 2026-10-03.
- [wikipedia-ja-city-readings] — the readings of 札幌市 and 横浜市. Read,
  2026-10-03.
- [uts35-v48-matching] — Part 1: Language Matching (the fallback locales,
  the distance, the variables and the threshold) and Attribute alt. Read,
  2026-09-29.
- [uts35-v48] — Part 1: Subdivision Codes, Attribute draft, Inheritance
  and Validity (the lookup, the marker `↑↑↑`, the identifier as the last
  resort and the fallback locales before root), and Parent Locales. Read.
- ISO 3166-1 and ISO 3166-2, the standards the codes come from. Not read:
  CLDR's validity data stands in for them, as the list of codes and their
  status.

## Code

- `scripts/place-names-cldr.py` — reads the files above and writes
  `crates/hc-i18n/src/place_names/cldr48.rs`; `--check`, `--dump`,
  `--stats` and `--fallbacks`.
- `crates/hc-i18n/src/place_names.rs` — the lists, the tables and the
  lookup, the territory half behind the `territories` feature and the
  subdivisions behind `place-names`; the tests
  `tokyo_is_tokyo_to_in_japanese_and_tokyo_in_english`,
  `a_german_and_an_american_subdivision_in_two_locales`,
  `a_name_falls_back_along_cldrs_chain_then_to_english`,
  `territories_follow_the_same_chain`,
  `a_deprecated_code_only_one_locale_names`,
  `a_pass_and_a_lookup_agree`, `every_table_is_consistent`,
  `a_tables_parent_is_the_next_table_of_its_chain`,
  `mongolian_and_tachelhit_in_the_latin_script_name_the_territories`,
  `the_regional_files_name_what_differs_from_their_parents`,
  `an_alt_form_is_resolved_as_cldr_resolves_a_path`,
  `the_release_levels_pass_over_a_provisional_name`,
  `language_matching_gives_fallbacks_before_english`,
  `a_subdivision_falls_back_by_language_matching_too` and
  `subdivisions_lie_within_what_cldr_says`.
- `crates/hyper-calendar/src/holiday_lines.rs` and
  `crates/hc-format/src/patterns/zone.rs` — the holiday tables' and the
  zone names' use of the territory half, at the release levels.
- `crates/hc-i18n/src/municipal_names.rs` — the municipalities' names and
  their lookup; the tests `every_table_names_the_same_codes_in_order`,
  `kawasaki_is_named_in_japanese_romanised_and_english` and
  `the_names_are_cities`.
- `crates/hyper-calendar/src/place_lines.rs` — the lines of the three
  exports; the tests `a_line_is_six_cells`,
  `every_place_a_holiday_table_names_has_a_line`,
  `every_municipality_a_holiday_table_lists_has_a_name` and
  `a_municipality_is_named_and_stands_beside_its_prefecture`.
- `crates/hyper-calendar-wasm/src/places.rs`,
  `crates/hyper-calendar-ffi/src/places.rs` — the `places` layer of each
  boundary, and their tests `tests/places.rs`; the JavaScript binding's
  `territories`, `subdivisions` and `placeName`.
