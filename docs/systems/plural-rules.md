# Plural rules: the cardinal categories CLDR gives a number

Backs `hc-i18n`'s `plural` module: `PluralCategory`, `PluralOperands`,
`PluralRules` and the table `plural::RULES`; and through them
`hc-humanize`'s `PluralForms` and the phrases of the `hc_relative_time`,
`hc_relative_day`, `hc_relative_day_at` and `hc_duration` exports. No
calendar identifier is registered. The WebAssembly module and the C library
export the cardinal category of a number in a locale as `hc_plural_category`
(`hyper_calendar::i18n_lines::plural_category_line`), which reads the operands
UTS #35 takes from the number as written, so `1.0` and `1` are different
questions; the kind `ordinal` is `HC_ERR_NO_DATA`.

## What it is

A language writes the words around a number differently according to the
number. English has *1 day* and *2 days*. Russian has *1 день*, *2 дня* and
*5 дней*, and a fourth form for *1,5 дня*. Arabic has six forms. A message
such as "in 2 months" therefore cannot be one template with a number in it.
The language has to say which of its forms the number takes.

CLDR does this with six category keywords: `zero`, `one`, `two`, `few`,
`many` and `other`. A language uses a subset of them, and `other` is always
among them: it covers every number that none of the language's other rules
claims [uts35-v48]. The categories are labels for a language's own forms,
not meanings. `few` is 2 to 4 in Russian and 3 to 10 in Arabic, `many`
begins at 5 in Russian, at 11 in Arabic and at 7 in Irish, and in Czech it
means a number with a decimal fraction [cldr48-plural-chart]. A category
exists in a language because two numbers need different versions of the
same sentence: the test is the minimal pair [uts35-v48].

UTS #35 defines two kinds of rule set per language, and they are
independent. The *cardinal* rules choose the form after a count, *1 day*.
The *ordinal* rules choose the form of a position, *1st floor*, and are
defined for integers only [uts35-v48]. English has two cardinal categories
and four ordinal ones [uts35-v48]. The rules are not part of any one
calendar. They are what lets a calendar library say a duration or a date
distance in a language that is not English.

The rule for a language depends on the number as it is *written*, not only
on its value. In English *1* takes `one` and *1.0* takes `other`; in French
and Portuguese 0 and 1.5 are both `one`; in Czech every number with a
visible fraction is `many`. So the input to a rule is a source string, and
the rule reads six or eight operands off it.

## How it works

**The source number and its operands.** UTS #35 Part 3 defines the operands
on the written form of the number [uts35-v48]. A negative number is judged
by its absolute value, and leading integer zeros have no effect.

| Operand | Meaning | In this crate |
| --- | --- | --- |
| `n` | the absolute value | `PluralOperands::n`, the nearest `f64`, for display only; rules use the exact integer operands |
| `i` | the integer digits | `i`, a `u64` |
| `v` | the count of visible fraction digits, trailing zeros included | `v`, a `u32` |
| `w` | the count of visible fraction digits, trailing zeros excluded | `w`, a `u32` |
| `f` | the visible fraction digits as an integer, trailing zeros included | `f`, a `u64` |
| `t` | the visible fraction digits as an integer, trailing zeros excluded | `t`, a `u64` |
| `c` | the compact decimal exponent: for `1.2c6` (1.2 million) it is 6, and the other operands are those of 1200000 | not carried; always 0 |
| `e` | a deprecated synonym of `c`, which UTS #35 says may be redefined | not carried; always 0 |

Some operands of the sources the specification gives, as
`PluralOperands::parse` reads them:

| Source | `i` | `v` | `w` | `f` | `t` |
| --- | --- | --- | --- | --- | --- |
| `1` | 1 | 0 | 0 | 0 | 0 |
| `1.0` | 1 | 1 | 0 | 0 | 0 |
| `1.00` | 1 | 2 | 0 | 0 | 0 |
| `1.30` | 1 | 2 | 1 | 30 | 3 |
| `1.03` | 1 | 2 | 2 | 3 | 3 |
| `1.230` | 1 | 3 | 2 | 230 | 23 |
| `1200.50` | 1200 | 2 | 1 | 50 | 5 |

An `f` of 03 is 3, so `1.03` has `f` 3 and `v` 2. Operands are numbers,
and the zeros of a source are only kept in `v` [uts35-v48].

**The rule language.** A rule is a condition on the operands of a category.
CLDR writes it as follows [uts35-v48].

- A *condition* is relations joined by `and`, and those groups joined by
  `or`. `and` binds more tightly than `or`, so `X or Y and Z` is
  `X or (Y and Z)`.
- A *relation* is an operand, optionally followed by `%` and a modulus, then
  `=` or `!=` and a list of values and ranges: `i % 10 = 2..4`,
  `i = 0,1`, `v != 0`. The older keywords `is`, `in`, `within` and `mod`
  remain for compatibility; the preferred forms are `=`, `!=` and `%`.
- A range `a..b` is the integers from `a` to `b`. A list is an `or`. `!=`
  negates the whole relation, so `3.5 = 2..4,15` is false and `3.5 !=
  2..4,15` is true: a range never matches a number with a fraction.
- `n % 10` is the decimal remainder, where 21.33 gives 1.33, so `n % 10 = 1`
  is `i % 10 = 1 and f = 0`. `i % 10` is the remainder of the integer
  digits alone.
- Categories are tested in the order `zero`, `one`, `two`, `few`, `many`.
  The first condition that holds gives the category; if none holds the
  category is `other`, which has no condition.

Every rule also carries samples, `@integer` and `@decimal` lists with a
trailing ellipsis where the set is infinite, which a consumer can use to
check an implementation [uts35-v48].

**What this crate does with a rule.** It does not interpret the rule text.
Each rule is a Rust function from `PluralOperands` to `PluralCategory`,
transcribed from CLDR 48's text with its doc comment quoting the text, and
`plural::RULES` is a table of `(language, function)` rows kept in
ascending order. Languages whose CLDR text is identical share a function.
Five small methods of `PluralOperands` carry the relations that repeat:

| CLDR relation | Method | Matches |
| --- | --- | --- |
| `n = 1` | `n_is(1)` | 1, 1.0, 1.00; not 1.5 |
| `n = 3..6` | `n_in(3, 6)` | integers only |
| `n % 100 = 3..10` | `n_mod_in(100, 3, 10)` | false whenever `f` is not 0 |
| `i % 10 = 2..4` | `i_mod_in(10, 2, 4)` | the integer digits only |
| `e = 0 and i != 0 and i % 1000000 = 0 and v = 0` | `is_exact_million` | exact millions; the `e != 0..5` alternative is dropped |

Relations on `i`, `v`, `f` and `t` alone are written as direct comparisons.

**Finding the rules of a locale.** UTS #35 selects plural rules by the base
locale identifier and ignores any `-u-` extension [uts35-v48].
`PluralRules::for_language` finds a row by its exact key, which is a
language subtag or, for the one regional row, `pt-PT`.
`PluralRules::for_locale` walks `Locale::fallback`, and at each step drops
the extension, tries the hyphenated rows against the whole identity (so
`pt-PT` is found before `pt`), then the row of the step's language. A tag
with no row at any step takes root's rule, `other` for every number, and
reports its language as `und`. Because the language of the tag is tried
at the first step, the script and region of `zh-Hant-HK`, `ru-RU` or
`pt-BR` do not matter. The chain adds something only where the tag's own
language has no row and a parent names another language: `ht` goes to
`fr-HT`, which finds French's row, and `nb` and `nn` go to `no`, which has
none.

**Worked example.** The Russian rule is

```
one:  v = 0 and i % 10 = 1 and i % 100 != 11
few:  v = 0 and i % 10 = 2..4 and i % 100 != 12..14
many: v = 0 and i % 10 = 0 or v = 0 and i % 10 = 5..9
      or v = 0 and i % 100 = 11..14
```

and the chart's minimal pairs are *из 1 книги за 1 день*, *из 2 книг за 2
дня*, *из 5 книг за 5 дней* and *из 1,5 книги за 1,5 дня*
[cldr48-plural-chart]. Take the source numbers one at a time. An integer
has `v` = 0, so the first clause of every rule holds.

- `1`: `i % 10` is 1 and `i % 100` is 1, not 11. `one`.
- `2`: `one` fails (`i % 10` is 2). `i % 10` is in 2..4 and `i % 100` is 2,
  outside 12..14. `few`.
- `5`: `i % 10` is 5, so neither `one` nor `few`. `i % 10` is in 5..9.
  `many`.
- `21`: `i % 10` is 1 and `i % 100` is 21, not 11. `one`. So `21 день`.
- `22`: `i % 10` is 2, `i % 100` is 22. `few`.
- `25`: `i % 10` is 5. `many`.
- `101`: `i % 10` is 1 and `i % 100` is 1. `one`.
- `111`: `i % 100` is 11, so `one` fails; `i % 10` is 1, so `few` fails;
  `i % 100` is in 11..14. `many`. `112` is `many` for the same reason.
- `1.5`: `v` is 1, so every clause fails. `other`.

The same numbers through this crate's `PluralRules::for_language`, checked
by running it on 2026-10-03:

| Source | `ru` | `pl` | `ar` |
| --- | --- | --- | --- |
| `0` | many | many | zero |
| `1` | one | one | one |
| `2` | few | few | two |
| `5` | many | many | few |
| `11` | many | many | many |
| `21` | one | many | many |
| `22` | few | few | many |
| `25` | many | many | many |
| `100` | many | many | other |
| `101` | one | many | other |
| `102` | few | few | other |
| `111` | many | many | many |
| `1.0` | other | other | one |
| `1.5` | other | other | other |
| `2.0` | other | other | two |

Polish differs from Russian where `one` is only `i = 1`, so 21 and 101
fall to `many` by `i != 1 and i % 10 = 0..1`. Arabic reads `n` itself:
`n = 0`, `1` and `2` are `zero`, `one` and `two`, `n % 100 = 3..10` is
`few`, `n % 100 = 11..99` is `many`, and 100, 101 and 102 are `other`
because the remainder is 0, 1 and 2. Since `n = 2` holds for `2.0`, Arabic
calls `2.0` `two` where Russian and Polish, which test `v = 0`, call it
`other`.

## What is carried

`plural::RULES` holds 56 rows: 55 languages and the regional row `pt-PT`.
Each row of the table below is one rule function; the rule text is CLDR
48's cardinal rule for the languages listed, and a category a rule does not
state is `other`.

| Function | Rule | Categories | Rows |
| --- | --- | --- | --- |
| `rule_other_only` | no condition | other | `bo id ja jv ko my th vi yue zh` |
| `rule_one_if_i_is_one_and_v_is_zero` | one: `i = 1 and v = 0` | one, other | `de en fi nl sv sw ur` |
| `rule_one_if_n_is_one` | one: `n = 1` | one, other | `ha ml mn mr nah ne ps syr ta te tr` |
| `rule_one_if_n_is_zero_to_one` | one: `n = 0..1` | one, other | `pa` |
| `rule_hindi` | one: `i = 0 or n = 1` | one, other | `am bn fa hi pcm` |
| `rule_one_if_i_is_zero_or_one` | one: `i = 0,1` | one, other | `kab` |
| `rule_danish` | one: `n = 1 or t != 0 and i = 0,1` | one, other | `da` |
| `rule_filipino` | one: `v = 0 and i = 1,2,3 or v = 0 and i % 10 != 4,6,9 or v != 0 and f % 10 != 4,6,9` | one, other | `fil tl` |
| `rule_hebrew` | one: `i = 1 and v = 0 or i = 0 and v != 0`; two: `i = 2 and v = 0` | one, two, other | `he` |
| `rule_spanish` | one: `n = 1`; many: `e = 0 and i != 0 and i % 1000000 = 0 and v = 0 or e != 0..5` | one, many, other | `es` |
| `rule_french` | one: `i = 0,1`; many: as Spanish | one, many, other | `fr` |
| `rule_portuguese` | one: `i = 0..1`; many: as Spanish | one, many, other | `pt` |
| `rule_italian` | one: `i = 1 and v = 0`; many: as Spanish | one, many, other | `it pt-PT` |
| `rule_tachelhit` | one: `i = 0 or n = 1`; few: `n = 2..10` | one, few, other | `shi` |
| `rule_czech` | one: `i = 1 and v = 0`; few: `i = 2..4 and v = 0`; many: `v != 0` | one, few, many, other | `cs` |
| `rule_romanian` | one: `i = 1 and v = 0`; few: `v != 0 or n = 0 or n != 1 and n % 100 = 1..19` | one, few, other | `ro` |
| `rule_lithuanian` | one: `n % 10 = 1 and n % 100 != 11..19`; few: `n % 10 = 2..9 and n % 100 != 11..19`; many: `f != 0` | one, few, many, other | `lt` |
| `rule_latvian` | zero: `n % 10 = 0 or n % 100 = 11..19 or v = 2 and f % 100 = 11..19`; one: `n % 10 = 1 and n % 100 != 11 or v = 2 and f % 10 = 1 and f % 100 != 11 or v != 2 and f % 10 = 1` | zero, one, other | `lv` |
| `rule_slovenian` | one: `v = 0 and i % 100 = 1`; two: `v = 0 and i % 100 = 2`; few: `v = 0 and i % 100 = 3..4 or v != 0` | one, two, few, other | `sl` |
| `rule_polish` | one: `i = 1 and v = 0`; few: `v = 0 and i % 10 = 2..4 and i % 100 != 12..14`; many: `v = 0 and i != 1 and i % 10 = 0..1 or v = 0 and i % 10 = 5..9 or v = 0 and i % 100 = 12..14` | one, few, many, other | `pl` |
| `rule_east_slavic` | the Russian rule above | one, few, many, other | `ru uk` |
| `rule_irish` | one: `n = 1`; two: `n = 2`; few: `n = 3..6`; many: `n = 7..10` | one, two, few, many, other | `ga` |
| `rule_welsh` | zero: `n = 0`; one: `n = 1`; two: `n = 2`; few: `n = 3`; many: `n = 6` | zero, one, two, few, many, other | `cy` |
| `rule_arabic` | zero: `n = 0`; one: `n = 1`; two: `n = 2`; few: `n % 100 = 3..10`; many: `n % 100 = 11..99` | zero, one, two, few, many, other | `ar` |

**Which locales share a rule.** In CLDR 48's `plurals.xml` the cardinal
rules are 40 blocks, each listing the locales that have exactly those rules,
227 codes in all [cldr48-supplemental]. The 56 rows fall in 24 of the 40
blocks, which is one function each; the 16 blocks with no row are those of
`blo cv ksh`, `be`, `bs hr sr`, `br`, `tzm`, `kw`, `is`, `smn iu smj naq se
sat sms sma`, `lag`, `dsb hsb`, `mk`, `mt`, `gv`, `sgs`, `gd` and `si`.
Families in the 24:

- Germanic and others of the English shape: `de en fi nl sv sw ur`; Danish
  alone, because `t != 0` makes 0.1 and 1.6 singular.
- Romance: Spanish (`n = 1`), French (`i = 0,1`), Portuguese (`i = 0..1`)
  and Italian (`i = 1 and v = 0`) each have their own rule and all four add
  `many` for exact millions. Portugal takes the Italian rule, so `pt-PT` is
  the one regional row, and Brazil takes `pt`. The Italian block also holds
  `ca`, `lld`, `scn` and `vec`, which this crate has no row for.
- Slavic: `ru` and `uk` share a function; Polish, Czech and Slovenian each
  have their own. Czech's `many` is for fractions, not for large integers.
- Baltic: Latvian has `zero` for every multiple of ten and 11 to 19;
  Lithuanian has `many` for fractions.
- Celtic: Irish and Welsh. Welsh uses all six categories, with `many` for
  six only.
- Indo-Aryan and neighbours: `am bn fa hi pcm` take `i = 0 or n = 1`;
  Punjabi takes `n = 0..1`. `ha ml mn mr nah ne ps syr ta te tr` take
  `n = 1`, the largest block of `plurals.xml`, which also lists languages
  this crate has no row for, among them `el`, `hu` and `no`.
- Berber: Kabyle takes `i = 0,1`, and Tachelhit takes `i = 0 or n = 1` with
  `few` for 2 to 10.
- Hebrew has `one` and `two`; `one` also covers 0.1 and 0.5.
- No agreement with the numeral: `bo id ja jv ko my th vi yue zh`.

**Rows that take root's rule.** Eleven locale entries of `hc-i18n` name
languages that `plurals.xml` does not list, and resolve to root's `other`:
`aeb`, `ayl`, `ban`, `cop`, `mid`, `mix`, `rif`, `sa`, `yua`, `zap` and
`zgh` [cldr48-supplemental]. `tl` shares Filipino's row, as `plurals.xml`
lists `ceb fil tl` together. `nah` is in the `n = 1` block of
`plurals.xml`.

**Not carried.**

- Not carried: ordinal plural rules (not yet done). CLDR 48 gives them for
  40 languages in 24 rule sets [cldr48-plural-chart]; 14 of this crate's
  rows have one: `bn cy en fil fr ga hi it mr ne ro sv uk vi`. `select`
  answers the cardinal question only, and nothing in `PluralRules` names a
  type. The chart's ordinal rows were read; the ordinal rules file was
  opened for its header only.
- Not carried: the compact decimal exponent `c` and its synonym `e` (not
  yet done). `PluralOperands::parse` rejects `1.2c6` and `1e3`, and the
  `e != 0..5` alternative of the `many` rule of Spanish, French, Italian and
  Portuguese is dropped. For every source number written without an
  exponent that alternative is false, so the answers are exact for them.
  The crate's own documentation gives the reason as compact forms such as
  "1M", which nothing in the workspace produces.
- Not carried: the plural categories of a range, "1-2 meters" (not yet
  done). CLDR 48's chart shows every range row as not available
  [cldr48-plural-chart]; UTS #35 has the section [uts35-v48] and its
  contents were not read.
- Not carried: the explicit cases 0 and 1 of UTS #35, which take precedence
  over `zero` and `one` where a pattern states them [uts35-v48] (not yet
  done). `hc-humanize`'s `PluralForms` has the six categories and nothing
  else.
- Not carried: the rules of the other CLDR 48 languages (not yet done). Of
  the 227 codes of the cardinal section, 171 have no row, root and the
  legacy codes `iw`, `in` and `jw` of carried languages among them. A row is
  added with a locale entry, as `hc-i18n`'s README describes.
- Not carried: the ordinal rules of `ordinals.xml` and the compact-notation
  operands `c` and `e`, at the boundary or anywhere else (not yet done);
  `hc_plural_category` answers `HC_ERR_NO_DATA` for the kind `ordinal`.

## Accuracy

The rules are exact integer comparisons on operands, so the answer is the
same on every platform and in every build; the only floating-point operand,
`n`, is never read by a rule. The question is whether each function is
CLDR 48's rule. Three checks were made on 2026-10-03.

| Check | Source | Result |
| --- | --- | --- |
| The crate's own 29 tests in `plural::tests`, whose expectations are published sample values; `cargo test -p hc-i18n --all-features --lib plural` | [cldr48-supplemental] | 29 passed |
| Every row's category for 18,249 source numbers, 1,021,944 comparisons, against an independent evaluator of the chart's rule text. The evaluator parses UTS #35's condition syntax with exact fractions. The numbers are the integers 0 to 1299, 999999, 1000000, 1000001, 1999999, 2000000, 3000000, 20000000, 10^9 and 10^9+1, and for 140 integers (0 to 130, 200, 1000, 1001, 1011, 1012, 1021, 1111, 1000000, 1000001) every fraction of one digit and of two digits and the three-digit and four-digit fractions `000`, `001`, `011`, `031`, `100`, `110`, `111`, `400`, `999`, `1230` and `0000` | [cldr48-plural-chart], [uts35-v48] | 0 mismatches; `tl` against Filipino's rule, `nah` against the `n = 1` rule and `pt-PT` against the `pt_PT` rule |
| Every row against the chart's comparison tables, the category of each integer 0 to 127 and of `x.5` for `x` = 0, 1 and 2 or more: 7,336 cells | [cldr48-plural-chart] | 0 mismatches |
| The operands of seven sources against the specification's table | [uts35-v48] | agree (the first table above) |

The limits of these checks: the evaluator and the probe were scratch code
and are not in the repository, `c` and `e` were 0 throughout, and
integers above 1299 were sampled, not swept. The comparison with the chart
is with a rendering of `plurals.xml`; the file itself was read for its block
and locale lists, and its rule text is the chart's.

Known disagreements:

- `PluralRules::for_locale` gives `ht` the French rule, because the chain
  of `ht` is `ht`, `fr-HT`, `fr`, `und`. CLDR 48's `plurals.xml` lists no
  `ht` [cldr48-supplemental], and UTS #35 selects by the base locale
  identifier [uts35-v48], which would give Haitian Creole root's `other`.
  No test pins either. No `ht` entry is carried in `hc-i18n`'s `LOCALES`,
  so no carried locale is affected.
- The rustdoc of `plural::RULES` (`plural.rs`) names seven languages that
  take root's rule; the README names the eleven above.
- `docs/systems/locale-fallback.md` says plural rules are not looked up
  along the fallback chain; `for_locale` walks it.
- Policy §2 gives Welsh plural rules as an example of "a data entry, not
  editing control flow". Welsh is the function `rule_welsh`; a language whose
  rule has a shape no function has needs a new function.

## Sources

| Key | What it gives | Read |
| --- | --- | --- |
| [uts35-v48] | Part 3, "Language Plural Rules" (Version 48.2): the six categories and `other`, cardinal and ordinal, the source number, the operands table and examples, the condition syntax and precedence, the relation semantics, the evaluation order, selection by base locale identifier, the samples, "Explicit 0 and 1 rules"; the title of "Plural Ranges" | Yes, HTML, 2026-10-03. "Plural Ranges" not read beyond its title |
| [cldr48-plural-chart] | Every language's cardinal and ordinal categories, samples, minimal pairs and rule text; the comparison tables by integer and by fraction; the range rows | Yes, HTML, 2026-10-03 |
| [cldr48-supplemental] | `plurals.xml`: the 40 cardinal blocks and their 227 locale codes | Yes, 2026-10-03, the cardinal section; it has no ordinal section. The ordinal rules file, `ordinals.xml`, was opened for its header only |

## Code

- `crates/hc-i18n/src/plural.rs`: `PluralCategory`, `PluralOperands`
  (`parse`, `from_parts`, `from_integer`, `i`, `v`, `w`, `f`, `t`, `n`),
  `PluralRules` (`for_language`, `for_locale`, `select`, `select_integer`,
  `select_decimal`), `RULES` and the 24 `rule_*` functions.
- Tests in the same file, `plural::tests`. They hold the operands of
  written forms (`operands_come_from_the_written_form_not_the_value`), the
  rejected strings, one test per rule shape with the published samples
  (English, Danish, Arabic, Russian and Ukrainian, Polish, Czech, Welsh,
  Irish, Slovenian, Latvian, Lithuanian, Romanian, Hebrew, Hindi, French,
  Spanish, Italian, Portuguese, Turkish and the languages added for the
  most-spoken thirty), the regional row
  (`portugal_takes_its_own_row_and_brazil_the_language_one`), the fallback
  (`a_locale_finds_its_rules_through_the_fallback_chain`,
  `an_unknown_language_gets_the_root_rule`) and the table's order and its
  56 rows (`the_rule_table_is_sorted_and_free_of_duplicates`).
- `crates/hc-i18n/src/locale.rs`: `Locale::fallback`, which `for_locale`
  walks; see [locale-fallback.md](locale-fallback.md).
- `crates/hc-humanize/src/render.rs`: `operands_of` and `category_of`, which
  present a half to the rules as the written decimal `n.5`, tested by
  `a_half_is_presented_to_the_plural_rules_as_a_written_decimal`.
  `crates/hc-humanize/src/pattern.rs`: `PluralForms`, one pattern per
  category. `crates/hc-humanize/tests/cldr48_resolved.rs` holds every
  carried locale's phrases against CLDR's own resolution of them, each
  plural category included.
- `docs/i18n.md`, "Plural rules", and the `hc-i18n` README, "Adding a plural
  language".
