# The Python profile: `strptime`, `humanize` and `timedelta` rounding

Python's `datetime` and `humanize` are not standards but behaviours that a
great deal of code depends on, so a port needs their answers, not their
intentions. Three of those behaviours are systems of their own: the regular
expression `strptime` is made of, the arithmetic and the gettext catalogues
`humanize` is made of, and the way `timedelta` rounds. [`../python-parity.md`](../python-parity.md)
lists every Python name and what answers it; this document explains the three
systems.

Policy §5 applies. These are conventions beside the library's own — ISO 8601
parsing, CLDR phrasing, exact spans — and they have their own names: the
`python` module of `hc-format`, the `natural` module of `hc-humanize`, and
`Resolution::Microsecond` of `TimeDelta`.

## What it is

**`strptime`.** CPython's `_strptime` module turns a pattern into one
regular expression. Each directive is a list of alternatives in a fixed
order (`%d` is `3[0-1]|[1-2]\d|0[1-9]|[1-9]| [1-9]`), each run of whitespace
is `\s+`, matching is case-insensitive and anchored at the start, and
whatever the match leaves over is an error. The first match in the order of
the alternatives wins, so the parse of a field depends on what follows it.
After the match the fields are resolved by rules of their own: a two-digit
year of 69 to 99 is the 1900s, a day of the year is added to 1 January
without a check, the ISO week is counted from 4 January.

**`humanize`.** A Python package that phrases numbers and time spans for
people: *a moment*, *1 year, 3 months*, *1.2 billion*, *2.9 KiB*. Its time
functions use thresholds and rounding that are the package's own; its
`precisedelta` divides a span into units one at a time, rounds the smallest
unit with a printf format and carries what the rounding overflowed. Its
words come from gettext catalogues: one `.po` file for each of 35 languages,
compiled by GNU `msgfmt`, in which a plural message is chosen by a C
expression of the count that each file states (`Plural-Forms`).

**`timedelta` rounding.** `timedelta * float`, `/ float` and `/ int` are
computed in whole microseconds: the float is taken as the exact ratio of two
integers (`float.as_integer_ratio()`), the product is formed exactly, and the
one rounding is to the nearest microsecond, ties to even. `timedelta /
timedelta` is the integer true division of the two spans in microseconds,
correctly rounded.

## How it works

**A worked `strptime`.** The pattern `%Y%m%d` against `2019124`:

1. `%Y` is exactly four digits: `2019`.
2. `%m` tries `1[0-2]`: `12` matches.
3. `%d` tries `3[0-1]`, `[1-2]\d`, `0[1-9]`: none match `4`; `[1-9]` does.
4. The pattern is spent at the end of the text: 2019-12-04.

A parser that reads a year greedily takes all seven digits and fails. Had the
text been `20191204`, `%m` would still take `12` and `%d` `04`; had it been
`2019124x`, the match would end before `x` and the leftover is an error. A
`%j` of 366 in a common year is 1 January of the next year, because the date
is `Jan 1 + (julian − 1)` with no check; an ISO week 53 is refused only when
the ISO year has 52 weeks (CPython 3.12 and later).

**A worked `precisedelta`.** `precisedelta(2 days + 1 µs)` at the default
minimum unit, seconds. The span divides into `days = 2`, `secs = 0`,
`usecs = 1`; the seconds are `usecs / 10⁶ = 0.000001`; `"%0.2f" % 0.000001`
is `0.00`, which reads back as `0.0`, and `0.0 > 0` is false, so no seconds
are written and the answer is *2 days* — not *2 days and 0.00 seconds*. At
59.999999 seconds the same rounding gives `60.00`, and `secs >= 60` carries
one into the minutes: *1 minute*. A 31-day span at minimum unit days has
`divmod(31, 30.5) = (1.0, 0.5)`, the remainder is truncated with `int`, and
the half day is gone: *1 month*.

**A worked `timedelta * float`.** 25 508 964 008 398 µs × 0.5: the ratio of
0.5 is 1/2, the product is exactly 12 754 482 004 199 µs, and the answer is
*147 days, 14:54:42.004199* with nothing to round. A multiplication by 0.1
shows the rounding: the double 0.1 is 3602879701896397 / 2⁵⁵, so 10 µs × 0.1
is 1.0000000000000000555 µs, which rounds to 1 µs; and 1 µs × 0.5 is 0.5 µs,
a tie, which rounds to the even 0 µs. This library does the product in 256
bits and rounds once, to the attosecond (`checked_scale`) or, for Python's
answer, to the microsecond (`checked_scale_at` with `Resolution::Microsecond`).

**A worked catalogue.** `activate("ru_RU")`, then `naturaltime` of 21
seconds. `ngettext("%d second", "%d seconds", 21)` asks the Russian catalogue,
whose header says `plural=(n%10==1 && n%100!=11 ? 0 : n%10>=2 && n%10<=4 &&
(n%100<10 || n%100>=20) ? 1 : 2)`; for 21 the first condition holds, so form
0, `%d секунда`, and the phrase is *21 секунда назад*. For 3 it is form 1,
*3 секунды назад* (the example in `humanize`'s README); for 25, form 2.
CLDR's categories would say the same for Russian, but not for every language
(Latvian puts 0 with the many in gettext and with *zero* in CLDR), so the
expression is evaluated as written (`natural::gettext`).

## What is carried

- **`strptime`**: every directive of `_strptime.TimeRE`; the compound
  directives `%c`, `%x` and `%X` as C-locale patterns; the resolution above;
  the checks Python makes (a directive twice, ISO year without week and
  weekday, the colon rule of `%z`). *Not carried*: Unicode decimal digits
  (no table is carried), the machine's zone names for `%Z`, a `%z` with a
  fraction (`UtcOffset` has whole seconds), CPython 3.14's `%e`, flags and
  widths (the 3.13 documentation does not say them). Year 0, year 10 000 and
  a second of 60 are accepted where Python refuses them, because the library
  holds them.
- **`fromisoformat`**: the forms of the 3.13 documentation, including a week
  without a day. *Not carried*: the C parser's behaviours the documentation
  does not describe (a stray character before the offset, an offset's
  minutes above 59, a fraction after the hour); an offset of 24 hours or
  more is refused as Python refuses it.
- **`humanize`**: every function of 4.16.0's `time`, `number`, `filesize` and
  `lists` modules, with the 35 catalogues and the separators of `i18n.py`; the
  functions in the parity table, and every function with words at the boundary
  in the `natural` layer, in the catalogue that serves the locale. A locale
  resolves along its fallback chain to the first catalogue that is for it and
  translates *every* message the function can write: a catalogue holds the
  English of the source for a message it lacks, and a few hold a raw `%d` or
  `%(value)s` for a word that takes no number (Korean, Bengali and Vietnamese
  in the powers), so a message that is empty, fuzzy or a raw placeholder in
  the `.po` file counts as untranslated. The German catalogue, which has
  *fünf* and *Millionen* and no *Byte*, serves `apnumber` and `intword` and
  not `naturalsize`; Japanese, which leaves *now*, *a moment* and the *and* of
  a list untranslated, serves `naturalday` and `ordinal` but not
  `naturaltime`, `naturaldelta` or `precisedelta`, which are then English
  whole, as are `precisedelta` in Korean, Simplified Chinese and Slovak. The
  fine units (*milliseconds*, *microseconds*) are needed only when the
  minimum unit is below the second. The language cell of every line says
  which catalogue wrote it, `en` where none serves, so a result is never
  written in two languages. `natural_list`
  is in no catalogue (its `, ` and ` and ` are literals in `lists.py`), so every
  locale gets English for it. *Not carried*: the behaviour `main` changed after
  the release (listed in the parity document), the parsing of strings into
  numbers, `naturaltime` of two readings at the boundary (a caller subtracts
  them), and a bare language with two catalogues, `pt`, which takes neither: no
  region is guessed.
- **`timedelta`**: `* float`, `/ float`, `/ int` and `/ timedelta` as Python
  computes them, beside the exact versions. `// int` floors at the
  attosecond, not the microsecond.
- **`time` and `calendar`**: `struct_time`, `gmtime`, `localtime`, `mktime`,
  `strptime`, `strftime`, `asctime`, `timegm`, `isleap`, `leapdays`,
  `weekday`, `monthrange` and `monthcalendar`. *Not carried*: the text and
  HTML calendars (their layout is in `calendar.py`, which was not read), the
  machine's zone and clock.

## Accuracy

Each function was run beside an independent answer over random cases, and
every disagreement was either fixed or is listed in the parity document.

- **`precisedelta`**: a transcription of 4.16.0's `time.py` as an oracle (it
  reproduces every example of the documentation), 300 000 random spans,
  minimum units, suppressed units and formats: no differences. The
  transcription, and not the package, because the package was not installed
  here and the source was read directly.
- **`naturaldelta`, the number functions, `naturalsize`, `natural_list`**: the
  same transcription, 100 000 spans and several hundred thousand numbers,
  floats near powers of ten, powers of 1000 and 1024, and integers of 1 to
  400 digits: no differences. Python's `repr` of a float, used by
  `intcomma`, over 100 000 doubles (the shortest digits, and the even one on
  an exact tie): none. `fractional` against Python's own `fractions` over
  178 697 values: none.
- **The catalogues**: GNU `msgfmt --check` compiled each of the 35 `.po`
  files in memory and Python's `gettext.GNUTranslations` answered
  `naturaltime`, `naturaldelta`, `precisedelta`, `ordinal` of both genders,
  `apnumber`, `intword`, `naturalsize` and `intcomma`; 154 000 random cases
  over the 35, none different.
- **One language in a result**: `crates/hc-humanize/tests/serving.rs` runs
  every function that writes words (`naturaldelta`, `naturaltime` and
  `precisedelta`, each with and without a minimum unit below the second,
  `apnumber`, `intword`, `naturalsize`, `naturalday`, `ordinal` of both
  genders) over every locale `hc-i18n` carries, the language of every
  catalogue with and without a region and a few chains (`de-AT`, `pt-AO`,
  `zh-Hant-TW`, `no`), and holds each result either to the English of the
  source, character for character, or to the words of one catalogue with no
  English word of the source left in it and no raw placeholder; the same
  holds of the boundary lines in `hyper-calendar`. The catalogues' groups of
  translated messages are generated from the `.po` files with the catalogues.
- **`strptime`**: CPython 3.12.14 over about 400 000 random patterns and texts
  and 233 hand-picked ones. The only disagreements are the accepted
  differences above (a year beyond 9999, a leap second), and CPython 3.14's
  additions.
- **`fromisoformat`**: 32 486 constructed strings against CPython 3.12.14:
  the differences are the families listed in the parity document.
- **`timedelta`**: 320 000 random scalings, divisions and ratios against
  CPython 3.14.7: none, except where Python raises `OverflowError` beyond its
  own range.

## Sources

- The Python 3.13 documentation of `datetime`, `time` and `calendar`
  (`python-datetime-docs`, `python-time-docs`, `python-calendar-docs`), read
  2026-09-26 and 2026-10-03.
- `humanize` 4.16.0: the documentation (`humanize-docs`), the release on PyPI
  (`humanize-pypi`), and its sources, README and catalogues at the tag
  `4.16.0` (`humanize-4-16-0-source`), read directly from
  raw.githubusercontent.com into memory on 2026-10-03; nothing was saved.
  The project's `main` branch was compared with the tag through GitHub's
  compare view, to list what changed after the release.
- CPython 3.12.14 and 3.14.7, and GNU `msgfmt`, run locally as oracles
  (`cpython-interpreter-checks`). CPython's `_strptime` source and the
  gettext manual were *not read*: the alternatives of each directive,
  the resolution rules and the `Plural-Forms` semantics are known from
  the documentation, the catalogues' own headers and what the interpreters
  answered, and the alternatives in `hc-format`'s `python/strptime.rs` are
  checked by the differential runs above, not by reading the module.

## Code

- `crates/hc-format/src/python.rs` and `python/strptime.rs`: the profile and
  the matcher; tests in `python.rs` pin the cases above.
- `crates/hc-humanize/src/natural.rs`, `natural/numbers.rs`,
  `natural/gettext.rs` and the generated `natural/catalogues.rs`
  (`scripts/humanize-gettext.py`, `--check` verifies it); tests in each, and
  `crates/hc-humanize/tests/gettext_catalogues.rs`.
- `crates/hc-core/src/duration.rs` (`scale_f64_nearest`, `div_f64_nearest`,
  `div_int_nearest`, `ratio`) and `wide.rs` (the 256-bit integer),
  `crates/hyper-calendar/src/civil.rs` (`Resolution`, the `TimeDelta`
  methods) and `civil/struct_time.rs` (`StructTime` and `calendar`); tests in
  `crates/hyper-calendar/tests/python.rs` and `hc-core`'s `duration`.
- `NaturalPhrases::for_locale` and `NaturalWords` choose the catalogue for a
  locale (`crates/hc-humanize/src/natural.rs`; test
  `a_locale_is_served_by_the_first_catalogue_that_translates_the_words`).
- The boundary's `natural` layer: `hc_apnumber`, `hc_fractional`,
  `hc_scientific`, `hc_metric`, `hc_naturalsize`, `hc_naturallist`,
  `hc_intword`, `hc_naturaldelta`, `hc_naturaltime`, `hc_precisedelta`,
  `hc_naturalday`, `hc_naturaldate`, `hc_ordinal`, `hc_intcomma` and
  `hc_intcomma_float`: `crates/hyper-calendar/src/exports.rs` and
  `humanize_lines.rs`. `strptime` and `fromisoformat` cross the boundary as
  `hc_parse_pattern` and `hc_parse_datetime` with the syntax `python`, and
  `isoformat` as `hc_format_datetime` with it (`datetime_lines.rs`).
