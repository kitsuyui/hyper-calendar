# Plural rules: the cardinal and ordinal categories CLDR gives a number

Backs `hc-i18n`'s `plural` module: `PluralCategory`, `PluralType`,
`PluralOperands`, `PluralRules` and the generated tables of
`plural::cldr48`; and through them `hc-humanize`'s `PluralForms` and the
phrases of the `hc_relative_time`, `hc_relative_day`, `hc_relative_day_at`
and `hc_duration` exports. No calendar identifier is registered. The
WebAssembly module and the C library export the category of a number in a
locale as `hc_plural_category` (`hyper_calendar::i18n_lines::plural_category_line`),
which reads the operands UTS #35 takes from the number as written, so `1.0`
and `1` are different questions, and answers the kind `cardinal` or
`ordinal`.

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
independent: "The type attribute is currently either cardinal (plural) or
ordinal (1st, 2nd, …)" [uts35-v48]. The *cardinal* rules choose the form
after a count, *1 day*. The *ordinal* rules choose the form of a position,
*1st floor*, *2nd*, *3rd*, *4th*, and are defined for integers only. English
has two cardinal categories and four ordinal ones; German has one of each,
so every German position takes the same `other` form (*1.*, *2.*). The
rules are not part of any one calendar. They are what lets a calendar
library say a duration or a date distance in a language that is not
English.

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
| `n` | the absolute value | `PluralOperands::n`, the nearest `f64`, for display only; the evaluator reads `i` and whether the fraction is zero |
| `i` | the integer digits | `i`, a `u64` |
| `v` | the count of visible fraction digits, trailing zeros included | `v`, a `u32` |
| `w` | the count of visible fraction digits, trailing zeros excluded | `w`, a `u32` |
| `f` | the visible fraction digits as an integer, trailing zeros included | `f`, a `u64` |
| `t` | the visible fraction digits as an integer, trailing zeros excluded | `t`, a `u64` |
| `c` | the compact decimal exponent: for `1.2c6` (1.2 million) it is 6, and the other operands are those of 1200000 | not carried; `c()` is always 0 |
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
  `or`: `condition = and_condition ('or' and_condition)*` and
  `and_condition = relation ('and' relation)*`, so `and` binds more tightly
  than `or`.
- A *relation* is an operand, optionally followed by `%` and a modulus, then
  `=` or `!=` and a list of values and ranges: `i % 10 = 2..4`,
  `i = 0,1`, `v != 0`. The older keywords `is`, `in`, `within` and `mod`
  remain for compatibility; `=` and `!=` "work identically to `in` and
  `not in`", and the preferred forms are `=`, `!=` and `%`.
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
check an implementation [uts35-v48]. A sample list is values and
`low~high` ranges, `0.0~1.5` standing for 0.0, 0.1, … 1.5, and a value may
carry a compact exponent, `1c6`.

**What this crate does with a rule.** The rule text is parsed once, by
`scripts/plurals-cldr.py`, into data: each category's condition as a slice
of `and` groups, each group a slice of `Relation` values, and a `Relation`
one operand, a modulus (0 for none), whether it is negated, and its
inclusive ranges. The generated file `plural/cldr48.rs` holds one rule set
per block of `plurals.xml` and of `ordinals.xml`, and two sorted tables,
`CARDINAL` and `ORDINAL`, from each locale code to its block's rule set.
One evaluator, `PluralRules::select`, reads them: a relation holds when the
operand's value, or its remainder by the modulus, lies in one of the
ranges — for `n`, only when the fraction is also zero — and `!=` turns the
answer round; a group holds when every relation does, a condition when any
group does, and the categories are tried in CLDR's order. No language has
a function of its own, and a new language is a row the generator writes.
The operand `c`, and `e` with it, reads as 0, so a rule's `e = 0 and …`
branch applies and its `e != 0..5` alternative never does, which is the
correct answer for every number written without an exponent.

**Finding the rules of a locale.** UTS #35 selects plural rules by the base
locale identifier and ignores any `-u-` extension [uts35-v48].
`PluralRules::of_kind_for_language` finds a row by its exact key, which is
a language subtag or, for the regional rows, a tag such as `pt-PT`;
`for_language` is the cardinal case. `of_kind_for_locale` (`for_locale`
for the cardinal rules) walks `Locale::fallback`, and at each step drops
the extension, tries the hyphenated rows against the whole identity (so
`pt-PT` is found before `pt`), then the row of the step's language. A tag
with no row at any step takes root's rule, `other` for every number, and
reports its language as `und`. Because the language of the tag is tried
at the first step, the script and region of `zh-Hant-HK`, `ru-RU` or
`pt-BR` do not matter. The chain adds something only where the tag's own
language has no row and a parent names another language: `ht` goes to
`fr-HT`, which finds French's row, and `nb` and `nn` go to `no`, which has
one of its own. `pt-PT` has a cardinal row and no ordinal one, so its
ordinal rules are Portuguese's.

**Worked example.** The Russian cardinal rule is

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
by running it on 2026-10-04:

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

The English ordinal rule is `one: n % 10 = 1 and n % 100 != 11`, `two: n %
10 = 2 and n % 100 != 12`, `few: n % 10 = 3 and n % 100 != 13`
[cldr48-supplemental]: 1, 21 and 101 are `one` (*1st*), 2 `two` (*2nd*), 3
`few` (*3rd*), 4, 11, 12, 13 and 111 `other` (*4th*, *11th*). Welsh uses
all six for positions, `zero: n = 0,7,8,9`, `one: n = 1`, `two: n = 2`,
`few: n = 3,4`, `many: n = 5,6`.

## What is carried

Every rule set of CLDR 48's `plurals.xml` and `ordinals.xml`, as data
[cldr48-supplemental]:

| Table | Locales | Blocks | Categories per locale |
| --- | ---: | ---: | --- |
| `CARDINAL` | 226, every code of `plurals.xml` but `root`; `pt-PT` among them | 40 | 1 for 34 (`other` only: `ja ko th vi zh yue id jv bo my` and 24 more), 2 for 139, 3 for 33, 4 for 11, 5 for 5 (`br ga gv mt sgs`), 6 for 4 (`ar ars cy kw`) |
| `ORDINAL` | 109, every code of `ordinals.xml` but `root` | 25 | 1 for 67, 2 for 24, 3 for 3, 4 for 9 (`az blo ca en gd kok kok-Latn mk mr`), 5 for 5 (`as bn gu hi or`), 6 for 1 (`cy`) |

A locale `plurals.xml` lists shares its block's one rule set; the legacy
codes `iw`, `in` and `jw` are rows like any other. The 65 locales of
`hc-i18n`'s `LOCALES` fall as follows: 54 of their languages have a
cardinal row; `aeb`, `ayl`, `ban`, `cop`, `mid`, `mix`, `rif`, `sa`,
`yua`, `zap` and `zgh` have none and take root's `other`. 36 of them have
an ordinal row, and 9 an ordinal rule beyond `other`: `bn en fil fr hi it
mr ne vi`; the other 27 (`de ja ru zh` among them) have one ordinal form.
Of the 42 locales of `ordinals.xml` with a rule beyond `other`, those
without an entry in `hc-i18n` (`ca gd hu ka sq sv uk` …) answer through
`hc_plural_category` all the same, since the rules are keyed by language.

The crate's own tests keep, beside the generated samples, hand-written
expectations for English, Danish, Arabic, Russian and Ukrainian, Polish,
Czech, Welsh, Irish, Slovenian, Latvian, Lithuanian, Romanian, Hebrew,
Hindi, French, Spanish, Italian, Portuguese and Turkish, and for Greek,
Hungarian, Icelandic, Maltese, Scottish Gaelic and Breton, each from the
chart's samples.

**Not carried.**

- Not carried: the compact decimal exponent `c` and its synonym `e` (not
  yet done). `PluralOperands::parse` rejects `1.2c6` and `1e3`, and the
  evaluator reads the operand as 0, so the `e != 0..5` alternative of the
  `many` rule of Spanish, French, Italian and Portuguese never holds. For
  every source number written without an exponent that alternative is
  false, so the answers are exact for them, and the differential test
  skips the `@integer` samples written with an exponent (`1c6`). The crate's
  own documentation gives the reason as compact forms such as "1M", which
  nothing in the workspace produces.
- Not carried: the plural categories of a range, "1-2 meters" (not yet
  done). CLDR 48's chart shows every range row as not available
  [cldr48-plural-chart]; UTS #35 has the section [uts35-v48] and its
  contents were not read.
- Not carried: the explicit cases 0 and 1 of UTS #35, which take precedence
  over `zero` and `one` where a pattern states them [uts35-v48] (not yet
  done). `hc-humanize`'s `PluralForms` has the six categories and nothing
  else.
- Not carried: the `within` relation, which matches non-integers in a
  range. No rule of CLDR 48 uses it, so the generator refuses it rather
  than carry an untested branch; `is`, `in`, `not` and `mod` it reads as
  their modern equivalents, and no rule of CLDR 48 uses those either.

## Accuracy

The rules are exact integer comparisons on operands, so the answer is the
same on every platform and in every build; the only floating-point operand,
`n`, is never read by the evaluator. The question is whether the generated
data is CLDR 48's rule and whether the evaluator reads it as UTS #35 says.
Checks made on 2026-10-04:

| Check | Source | Result |
| --- | --- | --- |
| Every block's `@integer` and `@decimal` samples, ranges expanded, against `select_decimal`: 3 767 samples over the 40 cardinal and 25 ordinal blocks (`every_block_answers_cldrs_own_samples`), the samples written with an exponent skipped | [cldr48-supplemental] | 0 mismatches |
| The crate's hand-written tests, 31 in `plural::tests`, whose expectations are the chart's samples: the twenty languages above, the six added, the English and Welsh ordinals | [cldr48-plural-chart], [cldr48-supplemental] | 31 passed |
| The operands of seven sources against the specification's table | [uts35-v48] | agree (the first table above) |
| The tables' row counts, 226 and 109, and their sort order | [cldr48-supplemental] | hold |

The limits of these checks: CLDR's samples are the publisher's own choice
and short, so a rule's behaviour between samples rests on the evaluator
reading the relation as the specification writes it; `c` and `e` were 0
throughout; and the ranges' step is read off the endpoints' last digit, as
the chart's expansions show, since UTS #35's text does not define it.

Known disagreements:

- `PluralRules::for_locale` gives `ht` the French rule, because the chain
  of `ht` is `ht`, `fr-HT`, `fr`, `und`. CLDR 48's `plurals.xml` lists no
  `ht` [cldr48-supplemental], and UTS #35 selects by the base locale
  identifier [uts35-v48], which would give Haitian Creole root's `other`.
  No test pins either. No `ht` entry is carried in `hc-i18n`'s `LOCALES`,
  so no carried locale is affected.

## Sources

| Key | What it gives | Read |
| --- | --- | --- |
| [uts35-v48] | Part 3, "Language Plural Rules" (Version 48.2): the six categories and `other`, cardinal and ordinal (the `type` attribute), the source number, the operands table and examples, the condition syntax (`condition`, `and_condition`, `relation`, `range_list`) and precedence, the relation semantics, the evaluation order, selection by base locale identifier, the samples' syntax (`@integer`, `@decimal`, `~`, `…`, the `c`/`e` exponent), "Explicit 0 and 1 rules"; the title of "Plural Ranges" | Yes, HTML, 2026-10-03 and 2026-10-04. "Plural Ranges" not read beyond its title |
| [cldr48-plural-chart] | Every language's cardinal and ordinal categories, samples, minimal pairs and rule text; the comparison tables by integer and by fraction; the range rows | Yes, HTML, 2026-10-03 |
| [cldr48-supplemental] | `plurals.xml`: the 40 cardinal blocks, their 227 locale codes, rule text and samples | Yes, 2026-10-03 and, by the generator, 2026-10-04 |
| [cldr48-ordinals] | `ordinals.xml`: the 25 ordinal blocks, their 110 locale codes, rule text and samples | Yes, by the generator, 2026-10-04 |

## Code

- `scripts/plurals-cldr.py`: the generator, which parses each rule's
  condition and writes `crates/hc-i18n/src/plural/cldr48.rs` (`--check`
  fails where the file is stale; `--dump` prints every rule).
- `crates/hc-i18n/src/plural.rs`: `PluralCategory`, `PluralType`,
  `PluralOperands` (`parse`, `from_parts`, `from_integer`, `i`, `v`, `w`,
  `f`, `t`, `c`, `n`), `Relation`, `Rule` and the evaluator, `PluralRules`
  (`for_language`, `for_locale`, `of_kind_for_language`,
  `of_kind_for_locale`, `languages`, `language`, `kind`, `categories`,
  `select`, `select_integer`, `select_decimal`).
- `PluralRules::category_examples` and `PROBE_OPERANDS`, in the same file:
  the categories a rule set answers, each with the first probe — the written
  numbers 0 to 1 100 and the first millions, as integers and with one and
  two fraction digits — that falls in it, a sample for each form; and at
  the boundary `hc_plural_categories` in
  `crates/hyper-calendar/src/i18n_lines.rs`, which writes them for a
  locale's cardinal or ordinal rules
  (`the_categories_a_rule_answers_are_its_rules`).
- Tests in the same file, `plural::tests`. They hold the operands of
  written forms (`operands_come_from_the_written_form_not_the_value`), the
  rejected strings, one test per rule shape with the published samples, the
  ordinals (`the_ordinal_rules_choose_the_form_of_a_position`), the
  regional row (`portugal_takes_its_own_row_and_brazil_the_language_one`),
  the fallback (`a_locale_finds_its_rules_through_the_fallback_chain`,
  `an_unknown_language_gets_the_root_rule`), the tables' order and counts
  (`the_rule_tables_are_sorted_and_free_of_duplicates`), the sample
  expansion (`samples_expand_as_the_specification_says`) and the
  differential test (`every_block_answers_cldrs_own_samples`).
- `crates/hc-i18n/src/locale.rs`: `Locale::fallback`, which `for_locale`
  walks; see [locale-fallback.md](locale-fallback.md).
- `crates/hc-humanize/src/render.rs`: `operands_of` and `category_of`, which
  present a half to the rules as the written decimal `n.5`, tested by
  `a_half_is_presented_to_the_plural_rules_as_a_written_decimal`.
  `crates/hc-humanize/src/pattern.rs`: `PluralForms`, one pattern per
  category. `crates/hc-humanize/tests/cldr48_resolved.rs` holds every
  carried locale's phrases against CLDR's own resolution of them, each
  plural category included.
- `crates/hyper-calendar/src/i18n_lines.rs`: `plural_category_line`, the
  line of `hc_plural_category`, which takes the kind `cardinal` or
  `ordinal` (`a_number_has_a_plural_category`).
- `docs/i18n.md`, "Plural rules", and the `hc-i18n` README, "Adding a plural
  language".
