# Casing and bidirectional text: the Turkic i, names that take a capital, and isolated fields

Backs `hc-i18n`'s `casing` module (`CasingStyle`, `casing_style`,
`capitalises_month_names`, `to_lowercase`, `to_uppercase`, `capitalise_first`,
`lowercase_first`, `name_at_sentence_start`, `name_in_sentence`) and its
`direction` module (`Direction`, `script_direction`, `locale_direction`,
`first_strong_direction`, `needs_isolation`, `write_isolated`,
`write_first_strong_isolated`, `write_field`, `isolate`, `strip_isolates`, and
the constants for U+2066 to U+2069, U+200E and U+200F), and the two exports of
the `calendars` layer that write them: `hc_case` and `hc_isolate`
(`hyper_calendar::i18n_lines::case_line` and `isolate_line`). The locale line
`hc_locale_info` carries the same facts about a locale in its columns 16
(direction), 17 (`standard` or `turkic`) and 18 (whether names take a capital).
No calendar identifier is registered: these are properties of text.

## What it is

**Casing.** Upper and lower case are character mappings that a language can
change. Turkish and Azerbaijani have four letters where English has two: `i`
and `İ` carry a dot in both cases, and `ı` and `I` carry none. Uppercasing the
Turkish month name *iyi* with the default mapping gives `IYI`, a different
word; the language-sensitive rows of `SpecialCasing.txt` make `i` uppercase to
`İ`, and `I` lowercase to `ı`. The file's rows for `tr` and `az` are:
`0130` lowercases to `i`; `0307` loses its dot after an `I` (`After_I`); `0049`
lowercases to `0131` unless it is before a dot above (`Not_Before_Dot`); and
`0069` uppercases to `0130`. It has rows for Lithuanian as well, which keeps
the dot of a lowercase `i` when accents follow [unicode-specialcasing-18].

A second question is not about characters: whether a month or weekday name is
a capitalised word at all. English and German write *January* and *Januar* in
the middle of a sentence; French writes *janvier*, and Russian *январь*,
though each writes a capital where the name opens a sentence. CLDR carries
this as context transforms of the names; this crate keeps one boolean per
locale.

**Direction.** A date is a run of text dropped into a sentence that may run
the other way. In "الأحد 12/3" a left-to-right month inside an Arabic
sentence can drag its neighbouring punctuation to the other side, because the
Unicode bidirectional algorithm resolves the whole paragraph at once. UAX #9
defines the isolates for it: LRI (U+2066) and RLI (U+2067) open a run of known
direction, FSI (U+2068) opens one whose direction is "the direction of its
first strong directional character that is not inside a nested isolate", and
PDI (U+2069) ends the scope of the last open one [uax9]. The text inside is
resolved on its own and does not reorder what surrounds it.

The paragraph direction of a text with none given is found by rules P2 and P3:
"find the first character of type L, AL, or R while skipping over any
characters between an isolate initiator and its matching PDI", and if it is
of type AL or R the level is one, else zero [uax9]. Digits are not strong:
European and Arabic numbers (types EN and AN, the Arabic-Indic digits among
them) and the common separators (type CS) are weak types that take the
direction of what surrounds them [uax9].

## How it works

*Casing.* `casing_style(locale)` is `Turkic` for Turkish and Azerbaijani, whatever
entry the locale resolves to (`az` has none of its own, and `hc_case("az", "upper",
"iyi")` is `İYİ` with the data entry `und`), and otherwise for the locales whose
data entry says so, and `Standard` for the rest. `to_uppercase` and `to_lowercase` map each
character by Unicode's default (`char::to_uppercase`, `char::to_lowercase`),
except that a Turkic locale maps `i`→`İ`, `ı`→`I`, `I`→`ı`, `İ`→`i`.
`capitalise_first` and `lowercase_first` recase the first character only,
because a title case of every word needs a word-break rule the crate does not
have, and it would damage `2 de enero` and `tháng 1`. `name_at_sentence_start`
is `capitalise_first`. `name_in_sentence` leaves a name that a capitalising
locale writes with a capital and lowercases the first character of any other.

*Direction.* A locale's direction is `rtl` for a script subtag of the ten
scripts `Adlm Arab Aran Hebr Nkoo Rohg Samr Syrc Thaa Yezi`, and for a tag with
no script the direction of its data entry; an explicit script wins, so
`ar-Latn-EG` is `ltr`. `first_strong_direction(text)` returns `rtl` for the first character that is
in one of eight code point ranges (U+0590 to U+05FF, U+0600 to U+07BF,
U+0800 to U+085F, U+08A0 to U+08FF, U+FB1D to U+FDFF, U+FE70 to U+FEFF,
U+10800 to U+10FFF and U+1E800 to U+1EFFF: Hebrew, Arabic, Syriac, Thaana,
Samaritan, Mandaic, Arabic Extended-A, the presentation forms and the
right-to-left historic scripts and Adlam), `ltr` for the first that is
alphabetic otherwise, and `None` where there is none.
`needs_isolation(outer, inner)` is true when they differ. `write_field` wraps a
field in the isolate of its own direction only then, so a date inside prose of
its own direction carries no extra code points. `isolate` wraps in FSI and PDI.
`strip_isolates` removes the four isolates and the two marks, for comparing or
hashing.

**A worked example.** The calls below were made on 2026-10-04 and give these
lines (a line is tab-separated; U+2066 and the like are written out).

| Call | Result cells |
| --- | --- |
| `hc_case("tr", "upper", "iyi")` | `İYİ`, `upper`, `turkic`, `tr` |
| `hc_case("en", "upper", "iyi")` | `IYI`, `upper`, `standard`, `en` |
| `hc_case("tr", "lower", "IYI")` | `ıyı` |
| `hc_case("tr", "lower", "İYİ")` | `iyi` |
| `hc_case("tr", "capitalise-first", "iyi")` | `İyi` |
| `hc_case("de", "upper", "straße")` | `STRASSE` |
| `hc_case("fr", "sentence-start", "janvier")` | `Janvier` |
| `hc_case("fr", "in-sentence", "Janvier")` | `janvier` |
| `hc_case("de", "in-sentence", "Januar")` | `Januar` |
| `hc_case("en", "capitalise-first", "2 de enero")` | `2 de enero` |
| `hc_isolate("en", "field", "الأحد")` | U+2067 `الأحد` U+2069, `ltr`, `rtl`, `1`, `field` |
| `hc_isolate("ar-EG", "field", "الأحد")` | `الأحد` with no isolate, `rtl`, `rtl`, `0` |
| `hc_isolate("ar-EG", "field", "2024")` | `2024`, `rtl`, empty (no strong character), `0` |
| `hc_isolate("ar", "first-strong", "Sunday")` | U+2068 `Sunday` U+2069, `rtl`, `ltr`, `1` |
| `hc_isolate("ar", "strip", "\u{200F}12\u{200E}")` | `12` |

By hand: *iyi* is the Turkish word, `i`, `y`, `i`. Uppercasing `i` in a
Turkic locale gives `İ` (the row `0069; 0069; 0130; 0130; tr`), and `y` has
no language-sensitive row, so the word is `İYİ`; the default mapping gives
`IYI`. The German `ß` has no language-sensitive row and uppercases by
Unicode's default to `SS`. For direction, an English sentence is left to
right and the Arabic word is right to left: they disagree, so the word is
wrapped in RLI and PDI. An Arabic sentence and the digits `2024` disagree on
nothing, since digits are not strong and the field has no direction of its
own, so nothing is added.

## What is carried

- **Locales.** `hc-i18n` carries 65 locale entries. Of them one, `tr`, is
  Turkic; sixteen capitalise month and weekday names (`ban`, `cop`, `de`, `en`,
  `en-001`, `en-GB`, `fil`, `ha`, `id`, `jv`, `kab`, `nah`, `pcm`, `sw`, `tr`,
  `yua`) and 49 do not; ten are right to left (`ar`, `ar-EG`, `fa`, `he`,
  `mid`, `pa-Arab`, `ps`, `syr`, `ur`, `ur-IN`).
- **The six modes of `hc_case`**: `lower`, `upper`, `capitalise-first`,
  `lowercase-first`, `sentence-start` and `in-sentence`; and **the three of
  `hc_isolate`**: `field` (isolate only where the two directions disagree),
  `first-strong` (always wrap in U+2068 and U+2069) and `strip`.
  A mode not named is `HC_ERR_UNKNOWN` and a tag that does not parse
  `HC_ERR_MALFORMED`; matching is the library's, ASCII case-insensitive and
  trimmed.

Not carried:

- **The conditional Turkic rows and Lithuanian.** `I` followed by U+0307
  lowercases to `i` and the dot is removed (`Not_Before_Dot`, `After_I`), and
  the Lithuanian rows keep the dot of a lowercase `i`; the module maps single
  characters and reads no context. Not yet done.
- **Title case of words and locale-sensitive casing of other scripts**, which
  need a word-break rule. Not yet done.
- **The bidirectional algorithm.** The module does not reorder text, resolve
  neutral runs or implement the paragraph-level rules beyond a simplified
  first-strong test. Not yet done.
- **The remaining CLDR context transforms** (the capital at the start of a
  sentence or in a list, per field, per width). The one boolean per locale is
  not read from CLDR's `contextTransforms`; where the boolean was taken from
  was not recorded. Not yet done.

## Accuracy

The casing mappings were checked on 2026-10-04 against the rows of
`SpecialCasing-18.0.0.txt` (dated 2026-05-19, read again on 2026-10-05; its
Turkish and Azeri rows are those of 17.0.0) that apply to single characters: the
examples above reproduce `0069→0130`, `0049→0131` and `0130→0069` for `tr`, and the tests hold
the twelve Turkish month names to a round trip through upper and lower case.

The direction is checked against the property values of the Unicode Character
Database, and the module's test is its own. The module's description of its
first-strong test as "exact for every script this crate ships data for" does
not hold in three respects, each reproduced on 2026-10-04:

- **Digits and separators are read as strong.** The right-to-left blocks
  include the Arabic-Indic digits U+0660 to U+0669 (type AN), the extended
  Arabic-Indic digits U+06F0 to U+06F9 (type EN) and the Arabic comma U+060C
  (type CS), which UAX #9 does not count as strong. `first_strong_direction`
  of `٢٠٢٤` is `rtl`, where P2 finds no strong character, so
  `hc_isolate("en", "field", "٢٠٢٤")` wraps the digits in U+2067 and U+2069
  and says the text's own direction is `rtl`.
- **An isolate's content is read.** P2 skips what is between an isolate
  initiator and its matching PDI. `hc_isolate("en", "field", "<RLI>שלום<PDI>
  Sunday")` says `rtl` and isolates again, where P2 finds the `S` of `Sunday`,
  `ltr`, and adds nothing.
- **Blocks are approximate.** The first range stops at U+07BF, so NKo
  (U+07C0 to U+07FF, type R) and the Syriac Supplement (U+0860 to U+086F, type
  AL) are in none of the eight, and an alphabetic character outside them reads
  as left to right: `first_strong_direction` of NKo letter A (U+07CA) is
  `ltr`. The module's documentation lists NKo among the blocks.

These are departures of the implementation from UAX #9's rules P2 and P3, not
a convention. No test asserts the behaviour the algorithm asks for in any of
them. For the scripts and locales above the dates this crate writes have no
such characters at their start.

## Sources

Read directly as HTML or plain text on 2026-10-04.

- [uax9]: UAX #9, the Unicode Bidirectional Algorithm, revision 52 (Unicode
  18.0.0, dated 2026-09-01): sections 2.4 and 2.5 (the isolates), 3.2 (the
  character types) and 3.3.1 (P2 and P3).
- [unicode-specialcasing-18]: `SpecialCasing-18.0.0.txt` (dated 2026-05-19), the
  Turkish and Azeri and the Lithuanian sections, read as text through the
  Unicode site.
- The Unicode Standard 17.0, section 3.13, "Default Case Algorithms", and
  CLDR's `contextTransforms` and `scriptMetadata.xml`: cited by the module
  and not read for this document.
- The Unicode Character Database's bidirectional class of U+0660, U+06F0,
  U+060C, U+05D0 and U+0061: read from Python 3.9.6's `unicodedata` (Unicode
  13.0.0), which is a comparison and not the source for this crate's data.

## Code

`crates/hc-i18n/src/casing.rs` and `direction.rs`, with the locale fields
`LocaleData::casing`, `capitalises_month_names` and `direction` in
`crates/hc-i18n/src/data.rs`, and the boundary lines in
`crates/hyper-calendar/src/i18n_lines.rs` (`case_line`, `isolate_line`).

The tests that anchor them, in `casing.rs`: `turkish_keeps_the_dot_on_its_capital_i`,
`english_uses_the_default_mappings_for_the_same_letters`,
`the_turkish_month_names_survive_a_case_round_trip`,
`german_month_names_are_nouns_and_stay_capitalised`,
`french_month_names_are_lowercase_except_at_a_sentence_start`,
`russian_month_names_stay_lowercase_inside_a_sentence` and
`recasing_an_empty_or_non_cased_string_is_a_no_op`; in `direction.rs`:
`scripts_carry_their_direction`,
`locales_inherit_direction_from_data_or_from_an_explicit_script`,
`the_first_strong_character_decides_a_runs_direction`,
`isolation_is_added_only_when_the_directions_disagree`,
`an_embedded_field_is_wrapped_in_the_isolate_of_its_own_direction`,
`a_first_strong_isolate_lets_the_renderer_decide` and
`isolates_can_be_stripped_back_out`; and in `i18n_lines.rs`
`a_locale_cases_a_text` and `a_text_is_isolated_for_its_locale`.
