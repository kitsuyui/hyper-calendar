# Natural time: `humanize`'s thresholds and CLDR's relative-time patterns

Backs `hc-humanize`'s `natural` module (`Natural`, `NaturalPhrases`,
`DeltaOptions`, `PreciseUnit`, `gettext`) for the time functions
`naturaldelta`, `naturaltime`, `naturalday`, `naturaldate` and
`precisedelta`, and `hc-humanize`'s CLDR side: `relative`
(`RelativeTimeFormatter`, `Numeric`), `unit` (`TimeUnit`, `UnitAmount`),
`unit_choice` (`Thresholds`, `RoundingPolicy`), `approximate`,
`calendar_relative`, `duration` and the `lookup` and `data` modules under
them. The boundary exports `hc_relative_time`, `hc_relative_day`,
`hc_relative_day_at` and `hc_duration` are the CLDR side's; the exports of the
`natural` layer are `humanize`'s functions (below). No calendar
identifier is registered: these phrase spans and days, they do not count
them.

[python-compatibility.md](python-compatibility.md) already explains
`humanize` 4.16.0's `precisedelta` arithmetic and its 35 gettext
catalogues, and gives the differential runs that measured them. This
document does not repeat that. It writes down what that one leaves out: the
shape of `Natural`, the thresholds and the rounding of `naturaldelta`,
`naturaltime`, `naturalday` and `naturaldate`, and the other convention it
stands beside, the CLDR relative-time path (what CLDR's `relativeTime` and
unit patterns are, how a plural category picks one, how a span is turned
into a count), with the choices each side makes named (policy §5).

## What it is

Two conventions answer "how do I say this span or day to a person?", and
they disagree on most spans.

**Python `humanize`.** The package's time functions turn a `timedelta`
into *a moment*, *an hour*, *1 year, 3 months*, with a ladder of
thresholds and roundings of its own, and a day into *today*, *tomorrow* or
*yesterday* [humanize-github-readme, humanize-docs]. Its words come from
gettext catalogues, activated process-wide with `humanize.i18n.activate`
[humanize-github-readme]. Its behaviour is the package's source, and the
documentation gives only examples [humanize-docs]. The release this library
follows is 4.16.0 [humanize-4-16-0-source].

**CLDR's relative time.** The Unicode CLDR gives each field of a date
(year, quarter, month, week, day, hour, minute, second, and each weekday)
two things in each language and each of three widths, long, short and
narrow [uts35-dates-48]. One is the *relative words* for an offset from the
present: *yesterday*, *today*, *tomorrow* for the day field, and in some
languages two before and two after. The other is the *relative time
patterns*, a `past` set and a `future` set, each a message with `{0}` for
the number and one entry for each plural category the language has: for
English `{0} day ago` and `{0} days ago`, `in {0} day` and `in {0} days`
[cldr48-humanize]. A separate part of the same data, the unit patterns,
gives a count with no direction (`{0} day`, `{0} days`), under identifiers
such as `duration-day` [uts35-general-48]. CLDR gives patterns and words.
No part read gives a rule for which unit to say a span in or how to round
into it; `Intl.RelativeTimeFormat` takes the unit and the count as
arguments [mdn-relativetimeformat].

The conventions differ in kind. `humanize`'s result is a function of the
span alone, with a fixed ladder, in the source's English or a catalogue's
translation. CLDR's phrases are a function of a count in a named unit; a
span reaches them only through a rule the caller chooses.

## How it works

### `humanize`: the ladder of `naturaldelta`

Let `d` be the whole days of the absolute span, `s` the seconds left over
in that day, `y = d // 365`, `r = d % 365` and `m = round(r / 30.5)`, with
Python's `round`, half to even. The phrase is the first line that applies
[humanize-4-16-0-source]:

| Condition | Phrase |
| --- | --- |
| `y = 0`, `d = 0`, `s = 0` | *a moment* |
| `d = 0`, `s = 1` | *a second* |
| `d = 0`, `s < 60` | *s seconds* |
| `d = 0`, `s < 3600`, `n = round(s / 60)` | *a minute* if `n = 1`, *an hour* if `n = 60`, else *n minutes* |
| `d = 0`, `n = round(s / 3600)` | *an hour* if `n = 1`, *a day* if `n = 24`, else *n hours* |
| `y = 0`, `r = 1` | *a day* |
| `y = 0`, `m = 0` or months off | *r days* |
| `y = 0` | *a month* if `m = 1`, *a year* if `m = 12`, else *m months* |
| `y = 1`, `r = 0` | *a year* |
| `y = 1`, `m = 0` or months off | *1 year, r days* |
| `y = 1` | *1 year, 1 month* if `m = 1`, *2 years* if `m = 12`, else *1 year, m months* |
| `y ≥ 2` | *y years*, with thousands separators |

Four consequences follow, and `Natural` reproduces each (the probe values
below were run in this library):

- Once a span reaches a day the seconds are ignored: 36 hours is *a day*,
  and 86 399 seconds is *a day* too, because `round(86399 / 3600)` is 24.
- Ties go to the even count: 150 seconds is `round(2.5) = 2`, *2
  minutes*; 12 600 seconds is `round(3.5) = 4`, *4 hours*; 3 570 seconds is
  `round(59.5) = 60`, *an hour*.
- A month is 30.5 days for the rounding only: 15 days is *15 days* and 16
  days is *a month*; 46 days is *2 months*; 351 days is *a year* and 716
  days is *2 years*.
- Years are whole: 1 000 days is `1000 // 365 = 2` years in 4.16.0 (the
  project's later code rounds instead; see
  [../python-parity.md](../python-parity.md)).

`naturaltime` is the same phrase with a tense: *now* where `naturaldelta`
says *a moment*, else *«phrase» ago* for a span in the past and *«phrase»
from now* for one ahead. A `timedelta` is how long *ago*, so a positive one
is the past (as `natural.rs` documents it; the description of the source
read does not say). `naturalday` of a date is *today*,
*tomorrow* or *yesterday* by the difference in days, and otherwise the day
written with `strftime` (default `%b %d`); `naturaldate` adds the year,
`%b %d %Y`, once the day is `5 × 365 / 12 ≈ 152.08` days or more from today:
152 days away is *Sep 23*, 153 days away is *Sep 24 2024* in the probe
[humanize-4-16-0-source]. `precisedelta` divides into years (365 days),
months (30.5 days) and the smaller units, truncates what a division leaves,
rounds the smallest unit with its format before testing it against zero, and
carries 31 days, not 30.5, into a month [humanize-4-16-0-source]; the
worked case is in python-compatibility.md.

### How `Natural` is shaped

`Natural` holds one `&'static NaturalPhrases`, a table of every string a
`humanize` function writes in one language: the fixed phrases (*a moment*),
the count messages as a `Plural` (the English singular and plural, and the
catalogue's `msgstr[n]` forms), the catalogue's `plural=` expression, the
number separators, the ordinal suffixes and the size units. The language is
the value you hold, not a global (policy §13): `NaturalPhrases::ENGLISH` is
the source's English, `by_catalogue("ru_RU")` is what
`humanize.i18n.activate("ru_RU")` loads, and `by_language("ru")` finds the
one catalogue a language has. A count message is chosen by evaluating the
catalogue's own expression (`natural::gettext`), because a translator's
`msgstr[1]` is the form *that* expression selects. No function branches on a
language (policy §2): a catalogue is one `const` in a generated file.

A `Duration` is read as Python's `timedelta` would hold it: the absolute
value, truncated to the microsecond, in `i128` microseconds. The arithmetic
is integer except where Python's is float (`precisedelta`'s `secs * 1e6 +
usecs`), which is reproduced as a float.

### CLDR: from a count to a phrase

`RelativeTimeFormatter::write` takes a signed count and a `TimeUnit` (the
eight units of the `relativeTime` fields) and does four things:

1. **Words first, if asked.** With `Numeric::Auto` and an offset the
   language has a word for (−1 day is *yesterday*), the word is written
   and nothing else. With `Numeric::Always`, the default, the numeric
   pattern is always used. The names and the choice are those of
   `Intl.RelativeTimeFormat`'s `numeric` option, `always` and `auto`
   [mdn-relativetimeformat, ecma402-relativetimeformat]. Auto does not
   mean "clever": a language with no word for −2 days gets *2 days ago*, and
   a half-unit never matches a word.
2. **Direction.** A count below zero takes the `past` set and any other,
   zero included, the `future` set (`in 0 days`). The crate has no negative
   zero. Whether `Intl.RelativeTimeFormat` treats zero the same way was not
   settled by what was read (see Accuracy).
3. **Plural category.** The category of the *magnitude*, from
   `hc_i18n::PluralRules`, picks one entry of the set [uts35-v48]. A category
   is not a grammatical number: Russian's `one` takes 1, 21, 31, and
   English's `one` needs no visible fraction, so 1.5 hours is `other`
   [uts35-v48]. A half-unit is therefore given to the rules as the written
   decimal `n.5` (`v = 1`, `f = 5`), which makes Czech call it `many`.
4. **The number.** The magnitude in the locale's digits, without a sign,
   substituted for `{0}`. A pattern that is empty for the category is the
   error `NoPattern`, not a guess.

Ordinary phrases come from data: one `LocaleData` for each of 50 locales,
generated from CLDR 48's files by `scripts/humanize-cldr.py`. Lookup walks
`Locale::fallback()`, and at the root it stops: root's patterns are signed
abbreviations (`-3 d`, `+3 d`, count `other` only), with its relative
words in English, and `day-short` and `day-narrow` are aliases of `day`
and of `day-short` [cldr48-humanize]. UTS #35 resolves such an alias by
restarting the lookup from the *requested* locale, so a narrow value is
sought in every parent before the short one is [uts35-v48]; the generator
resolves values that way. At run time `lookup` takes the other order, the
style fallback (narrow, short, long) inside each entry before the next
parent. No shipped entry makes the two differ: of the 936 combinations of
entry, width and unit, 96 are empty, and in none does a parent state the
value (a probe over `data::LOCALES`).

### CLDR: choosing the unit

A bare span has no calendar, so the crate gives each unit a mean length
(`unit`): second 1, minute 60, hour 3 600, day 86 400, week 604 800, month
2 629 746, quarter 7 889 238 and year 31 556 952 seconds, the year being
365.2425 days, so that a month and a quarter are exact integers. A
`Thresholds` table then says *stay in a unit while the count is below this
many of it*, and a `RoundingPolicy` says how to round into it. The default
table, `Thresholds::DEFAULT`, is:

| Unit | Stay while below | In seconds |
| --- | --- | --- |
| second | 45 seconds | 45 |
| minute | 45 minutes | 2 700 |
| hour | 22 hours | 79 200 |
| day | 6 days | 518 400 |
| week | 4 weeks | 2 419 200 |
| month | 11 months | 28 927 206 |
| year | no limit | |

`Thresholds::EXACT` promotes a unit only once a whole one fits (60, 60, 24,
7, 4 and 12), and `WITH_QUARTERS` is `DEFAULT` with the months row cut to
3 and a quarter row of 4. `RoundingPolicy` has `Ceil`, `Floor`, `Nearest`
(halves away from zero), `Truncate` (toward zero, the conversational
default), and `NearestHalf`, which turns 90 minutes into *an hour and a
half*. A span promoted out of the table's first unit never comes back as
zero: 28 days is 2 419 200 seconds, which is not below the week row's limit,
so the unit is the month, the count 0.92 truncates to 0, and the answer is
*1 month ago*, not *0 months*.

`approximate` takes the whole part in the chosen unit and reads the leftover
fraction against a table: below 0.02 no hedge, below 0.08 *about*, below
0.35 *just over*, below 0.70 *over*, otherwise *nearly* the next count up.
`calendar_relative` never divides a span for a day: the offset is the
difference of two `Rd` day numbers, so 23:30 on a Monday and 00:30 on the
Tuesday are *yesterday* and not *1 hour ago*, and a week is counted between
the weeks the locale starts, from `first_day_of_week`. `duration` lists a
span as components (days, hours, minutes, seconds by default), dropping the
remainder instead of rounding it. The exports `hc_relative_time` and
`hc_relative_day` choose with `DEFAULT` and `Truncate`.

### The competing conventions, and what names them

| Convention | Named by |
| --- | --- |
| `humanize` 4.16.0's ladder, half-to-even rounding, whole years | the `natural` module, `DeltaOptions` |
| `humanize`'s later, unreleased behaviour | not carried; listed in [../python-parity.md](../python-parity.md) |
| `humanize`'s plural choice, a catalogue's `plural=` expression | `NaturalPhrases::plural_expression`, `natural::gettext` |
| CLDR's plural choice, categories from the language's rules | `hc_i18n::PluralRules`, used by every CLDR formatter |
| the conversational ladder, 45 s to 11 months | `Thresholds::DEFAULT` |
| a unit promoted only when a whole one fits | `Thresholds::EXACT` |
| a ladder with quarters | `Thresholds::WITH_QUARTERS` |
| ceiling, floor, half away from zero, toward zero, nearest half | `RoundingPolicy` |
| the word or the number for an offset | `Numeric::{Always, Auto}` |
| the three widths | `RelativeStyle::{Long, Short, Narrow}` |
| a mean month against a calendar day | `unit` against `calendar_relative` |

A threshold table is caller data (`Thresholds::new` takes any), and
policy §5 treats such open-ended variation as a parameter. The three named
tables are the ones this library ships and tests.

**Worked example.** Take a span of 400 days, 34 560 000 seconds, in the
past, and one of 36 hours, 129 600 seconds.

*400 days, `humanize`.* `d = 400`, `y = 1`, `r = 35`, `m = round(35 /
30.5) = round(1.148) = 1`. The row for `y = 1` gives *1 year, 1 month*, and
`naturaltime` *1 year, 1 month ago*. With months off the row gives *1 year,
35 days*. `precisedelta` divides 35 days by 30.5 into 1 month and 4.5 days
and truncates the half: *1 year, 1 month and 4 days*.

*400 days, CLDR.* The month row's limit is `11 × 2 629 746 = 28 927 206`
seconds, which 34 560 000 exceeds, so the unit is the year. The count is
`34 560 000 / 31 556 952 = 1.095`, truncated to 1: *1 year ago*. The
fraction 0.095 is at least 0.08 and below 0.35, so `approximate` says *just
over a year*.

*36 hours, `humanize`.* `d = 1`, `r = 1`: *a day*, and *a day ago*; the 12
hours are not looked at.

*36 hours, CLDR.* 129 600 is not below the hour row's 79 200 and is below
the day row's 518 400: the unit is the day, the count 1.5. `Truncate` says
*1 day ago*, `Nearest` *2 days ago*, `Numeric::Auto` on the amount of −1
day *yesterday*, and `approximate` (fraction 0.5) *over a day*.

*Where the two part.* 150 seconds is *2 minutes* in `humanize` (the tie
goes to 2) and *3 minutes ago* under `Nearest`, *2 minutes ago* under
`Truncate`. A calendar day is a third answer: Monday 23:30 seen from
Tuesday 00:30 is *yesterday* from `CalendarRelativeFormatter`, though the
span is one hour.

*A plural, both ways.* For 21 seconds in Russian, `humanize`'s catalogue
expression `n%10==1 && n%100!=11 ? 0 : …` selects form 0, *21 секунда
назад*. For 21 days, the CLDR path's rules give category `one`, *21 день
назад*. For Latvian the two do not agree on the groups. The catalogue has form 0
for counts ending in 1 but not 11, form 2 for 0 alone, and form 1 for every
other count; `hc-i18n`'s CLDR rules have `one` as form 0 does, `zero` for 0,
10, 11 to 19, 20 and the like, and `other` for the rest, 2 to 9 and 22 among
them (a probe of both).

## What is carried

- **`humanize` time functions** (`Natural`): `naturaldelta`, `naturaltime`
  (from a span or from two civil date-times, every day counted 86 400
  seconds), `naturalday`, `naturaldate` and `precisedelta`, with the 35
  catalogues and English. Options are typed (`DeltaOptions`, `PreciseUnit`,
  a number of decimals for the format) and `now` is an argument (policy §13).
  The number and list functions are in
  [python-compatibility.md](python-compatibility.md). `naturalday` writes a
  day with `hc-format`'s C-locale `strftime`, in English whatever the
  catalogue, as Python's does under the C locale, and needs the `format`
  feature.
- **CLDR 48 relative time** in 50 locales: the eight units in the three
  widths, every plural category the language has, the relative words from
  −2 to 2, the unit patterns, list patterns, the decimal separator and the
  relative date-time pattern (*yesterday at 15:05*; `es` *ayer, 15:05*), with
  76 documented overrides of CLDR's own values in
  `src/data/cldr48_overrides.tsv`. A locale outside the 50 reaches the
  nearest of them by fallback, else the root; a locale `hc-i18n` carries whose
  chain reaches none of them (`aeb-Latn`, `ayl-Latn`, `ban`, `bo`, `cop`, `kab`,
  `mid`, `mix`, `nah`, `pa-Arab`, `rif`, `sa`, `shi-Latn`, `yua`, `zap`, `zgh`) is `NoPattern` at the
  crate and `NoData` at the boundary, never the root's `-5 h`: CLDR 48 states
  no phrases for it that its release level accepts.
- **Five strings that CLDR is not the source of**, hand-written in
  `data.rs`: the approximation hedges, the weekday phrases (*last Monday*),
  the half-unit idioms, the compact suffixes of *2h30m* and the indefinite
  units (*an hour*).
- **The boundary**: `hc_relative_time`, `hc_relative_day`,
  `hc_relative_day_at` and `hc_duration` in every locale carried, and the
  `natural` layer's `hc_naturaldelta`, `hc_naturaltime`, `hc_precisedelta`,
  `hc_naturalday`, `hc_naturaldate`, `hc_ordinal`, `hc_intword`,
  `hc_intcomma`, `hc_intcomma_float`, `hc_apnumber`, `hc_fractional`,
  `hc_scientific`, `hc_metric`, `hc_naturalsize` and `hc_naturallist`, which
  write Python's `humanize` functions in the language of the gettext
  catalogue that serves the locale.

Not carried, each a follow-up and not a decision:

- `clamp`, `intword` of an `i64`, `naturaltime` of two readings and a
  threshold table of the caller's own rows at the boundary: the WebAssembly
  README says why each has no export shape.
- The behaviour `humanize`'s `main` changed after 4.16.0 (the release is
  what `pip install humanize` gives; [../python-parity.md](../python-parity.md)).
- CLDR's own weekday words. `en.xml` and `ru.xml` of CLDR 48 state, for each
  weekday, `last`, `this` and `next` phrases, in Russian agreeing in gender
  (*в прошлый понедельник*, *в прошлую среду*) [cldr48-humanize]. The crate
  writes *last Tuesday* from its own patterns and avoids agreement by
  periphrasis (*понедельник на прошлой неделе*), on the stated ground that
  an agreed form is wrong for another weekday. CLDR's per-weekday fields
  carry the agreement, and were not read when the crate was written. What
  CLDR means by *last Tuesday* (the latest past Tuesday, or the Tuesday of
  the previous week, as `week_offset` counts) was not read either.
- CLDR's `approximately`, `atLeast` and `atMost` patterns, which several
  files state (`ja` *約 {0}*, root `~{0}`, `≥{0}`, `≤{0}`) [cldr48-humanize].
  The hedges *just over* and *nearly* have no CLDR field; *about* may.
- `relativePeriod` (*the week of {0}*) [uts35-dates-48], and the
  `hc-i18n` plural operands `c` and `e`, and gendered agreement (README: not
  yet done).
- Locales beyond the 50, and `humanize` catalogues for the CLDR path.
- Compact suffixes (*2h30m*) for `ar`, `hi` and `th`: not yet written.

## Accuracy

- **`humanize` 4.16.0.** The ladder above agrees with the prose reading of
  `time.py` of the tag (the tool returned summaries, not the code, so the
  agreement is on every branch and divisor and not on the code's text) and
  with the documented examples [humanize-docs], which the crate's tests pin
  (30 minutes and the four `precisedelta` examples); the probe in this work
  reproduced the first `precisedelta` example, *2 days, 1 hour and 33.12
  seconds*.
  The measured agreement is python-compatibility.md's: 300 000 random spans
  for `precisedelta` and 100 000 for `naturaldelta` against a transcription
  of the source, and 154 000 catalogue cases against Python's
  `gettext.GNUTranslations`, with no differences. That oracle is the
  maintainers' transcription, so it shares any misreading of the source; the
  package itself was not run in this work. Nothing is claimed for releases
  after 4.16.0.
- **Exactness.** The `humanize` path is integer arithmetic on
  microseconds, except where Python is float. The CLDR path counts in
  `f64`: a whole-second span is read exactly up to 2^53 seconds, and a span
  at an exact multiple of a unit's mean divides exactly. The probe held at the
  thresholds it tried (44 and 45 seconds, 21 and 22 hours, 5 and 6 days, 27
  and 28 days, 334 and 335 days).
- **CLDR 48 data.** `tests/cldr48_resolved.rs` compares every value taken
  from CLDR in all 50 locales with CLDR 48's own resolution in
  `cldr-json` 48.0.0, which the test's sample records, and a value differs
  from CLDR's only where one of the 76 override lines says so: 74 for a
  value a source argues against, 2 where `cldr-json`'s resolution differs
  from UTS #35's rule for the inheritance marker. All tests of the crate
  pass (the unit, CLDR, catalogue and doc tests, run 2026-10-03).
  `cldr-json` itself was not opened for this document.
- **Thresholds.** CLDR gives none, and no source read states these. Three of
  `DEFAULT`'s bounds, 45 seconds, 45 minutes and 22 hours, are the points at
  which moment.js documents its ranges to *a minute*, *an hour* and *a day*;
  its next bounds are 26 days to a month and 320 days to a year, not this
  table's [momentjs-relative-time]. The code says only that these are the
  bounds a chat client or a feed wants. They are the crate's choice, not a
  measured or cited one.
- **Disagreements found.** (1) `data.rs` and `README.md` say root states no
  special words and is language-free. CLDR 48's `root.xml` states English
  relative words for every field (*yesterday*, *last year*, *next month*,
  *now*) beside its signed patterns [cldr48-humanize], so `Numeric::Auto` in
  an unknown locale answers `-1 d` here and *yesterday* in root. (2) Root's
  quarter pattern is `-{0} Q`, with a capital, and `ROOT` has `-{0} q`.
  (3) CLDR 48's `units.xml` gives a month as a twelfth of a year and a
  quarter as a quarter of one, and no length for the year in seconds, only
  the Julian year of 31 557 600 s for the light-year
  [cldr48-units-supplemental]. The statement in `unit.rs` and the README that
  365.2425 days is the length CLDR and ICU use is therefore not supported by
  what was read; the means are the crate's own. (4) `lookup.rs` calls its
  style-before-parent order deliberate, which is not UTS #35's order
  (see above); it has no effect on the data shipped.
  (5) `pattern.rs` cites UTS #35 "§7" for the relative fields; they are in
  Part 4, *Dates*, under *Elements fields* [uts35-dates-48]. (6)
  `calendar_relative.rs` says "When `hc-format` exists, use that instead"
  of `write_clock_time`; `hc-format` exists.
- **Not settled.** Whether `Intl.RelativeTimeFormat` gives zero the future
  set, as `relative.rs` states: the page the tool summarised for
  [ecma402-relativetimeformat] said the opposite of the crate, so it was
  left as the crate's own convention.

## Sources

Every page was read with WebFetch on 2026-10-03, which returns a summary of
the page, not its text; the CLDR data files were read from
`raw.githubusercontent.com` into memory and not saved.

- [humanize-4-16-0-source]: `time.py` of the tag, the thresholds, the tense
  and the `precisedelta` helpers, as a prose description (the tool declined
  to reproduce the code). The other sources of the tag and the catalogues
  are python-compatibility.md's.
- [humanize-docs]: the documented examples of `naturaldelta` and
  `precisedelta`. No example is given for `naturaltime`, `naturalday` or
  `naturaldate` there.
- [humanize-github-readme]: the project's purpose, `i18n.activate("ru_RU")`
  and the README's example phrases, on the repository's main page. Its
  count of languages ("35+") was not used.
- [uts35-dates-48]: the `fields` element, `relative`, `relativeTime`,
  `relativePeriod`, the widths and the relative date-time pattern.
- [uts35-general-48]: unit patterns, the three lengths, `duration-*` ids.
- [uts35-v48]: the lookup procedure (a `source="locale"` alias restarts at
  the requested locale), the inheritance marker, and Part 3's plural
  categories and operands.
- [cldr48-humanize]: `fields` of `en.xml` (day, day-short, day-narrow,
  second, and the weekdays) and of `ru.xml` (the weekdays), the whole
  `fields` of `root.xml`, and the `miscPatterns` of `root`, `en`, `de`, `ru`
  and `ja`.
- [cldr48-units-supplemental]: `common/supplemental/units.xml`, the
  year-duration conversions.
- [mdn-relativetimeformat], [ecma402-relativetimeformat]: `numeric`,
  `style`, the eight units, plural selection on the absolute value. The
  specification's handling of negative zero was not settled.
- [momentjs-relative-time]: the default ranges of `fromNow()`.
- [cldr-json-48]: cited by the crate's test, *not read* here.
- *Not read:* ICU's `RelativeDateTimeFormatter` (cited by the README for
  the unit means), CLDR's `plurals.xml` (the rules reach this document
  through `hc-i18n`'s tests and Part 3), Python's `gettext` semantics and
  the gettext manual, and the source of `moment.js`.

## Code

- `crates/hc-humanize/src/natural.rs`, `natural/gettext.rs` and the
  generated `natural/catalogues.rs`. Tests in `natural.rs`:
  `naturaldelta_matches_the_documented_example`,
  `naturaldelta_follows_every_branch_of_the_documented_source`,
  `naturaldelta_reaches_below_a_second_only_when_asked`,
  `naturaltime_adds_the_tense_the_documented_source_adds`,
  `precisedelta_matches_the_documented_examples`,
  `precisedelta_takes_months_from_whole_days_and_refuses_an_impossible_minimum`,
  `precisedelta_rounds_before_testing_the_fraction_and_carries`,
  `precisedelta_keeps_pythons_float_microseconds_beyond_two_to_the_53` and
  `naturalday_and_naturaldate_name_the_near_days_and_write_the_rest`; and
  `crates/hc-humanize/tests/gettext_catalogues.rs`.
- `relative.rs`, `unit.rs`, `unit_choice.rs`, `approximate.rs`,
  `calendar_relative.rs`, `duration.rs`, `render.rs`, `lookup.rs`,
  `pattern.rs`, `data.rs` and the generated `data/cldr48.rs`, with
  `data/cldr48_overrides.tsv` and `scripts/humanize-cldr.py`; their unit
  tests, and `tests/cldr48_resolved.rs` with its sample
  `tests/data/cldr48_resolved.tsv` (`scripts/humanize-cldr-sample.py`).
- The boundary: `crates/hyper-calendar/src/humanize_lines.rs`
  (`relative_time_line` and its neighbours) and the `humanize` group of
  `exports.rs`.
