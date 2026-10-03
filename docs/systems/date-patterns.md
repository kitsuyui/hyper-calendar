# Date and time patterns: CLDR field patterns, strftime and Python's

Backs `hc-format`'s `patterns`: `patterns::cldr`, `patterns::strftime`,
`patterns::zone`, `FormatContext`, `Fields` and `ParsedFields`; `hc-format`'s
`python` (`python::strptime`, `python::write_ctime`); the facade's
`Date::strftime`, `Time::strftime`, `DateTime::strftime`,
`DateTime::strptime` and `StructTime::strftime`; and the export
`hc_format_pattern`, whose `syntax` is `cldr` or `strftime`. The matcher of
Python's `strptime` and `humanize` are in
[python-compatibility.md](python-compatibility.md); the zone fields `z`, `v`,
`V` and `O` in [zone-names.md](zone-names.md); whole dates written from a
locale's own templates, with no pattern, in
[written-dates.md](written-dates.md). This document does not repeat them.

## What it is

A pattern is a template with fields in it: *2026-09-21*, *Mon Sep 21*,
*21 septembre 2026*. Three conventions for writing one are in use, and a
program that ports code from one world to another meets all three.

- **CLDR field patterns**, UTS #35 Part 4. A field is one letter repeated;
  the count chooses the width and, for names, the style (`M` is 9, `MM` is
  09, `MMM` is Sep, `MMMM` is September, `MMMMM` is S). The ASCII letters are
  reserved. Any other character is literal, and a run in single quotes is
  literal too, with `''` for an apostrophe. The names, digits, week rules
  and zone names come from the locale [uts35-dates-48].
- **C and POSIX `strftime`**. A field is `%` and a letter. POSIX fixes the
  conversions and what `%c`, `%x`, `%X` and `%r` are in the POSIX locale
  [posix-strftime-2024]. C libraries add to it and part from one another.
  glibc adds the flags `_`, `-`, `0` and `^`, a width, and `%P`, `%k`, `%l`
  and `%s` [glibc-manual-strftime]; its source also reads the flag `#`,
  which changes case, and copies an unknown conversion through unchanged
  [glibc-strftime-l-source]. musl reads `-`, `_`, `0` and `+` and a width on
  a few conversions only, has no `%k`, `%l` or `%P`, writes `%Y` in at least
  four digits, and fails on an unknown conversion
  [musl-strftime-source].
- **Python's `strftime` and `strptime`**. `strftime` hands the pattern to the
  platform's C library, so its output "is platform dependent", and
  the documentation marks `%-d` and its kin "Platform specific"
  [python-datetime-docs]; it adds `%f`, microseconds, and `%z` with seconds.
  The same page says `strptime`'s parsing "is dependent on the platform's C
  implementation" too; the matcher this crate follows is CPython's
  `_strptime` as the interpreters ran it, which
  [python-compatibility.md](python-compatibility.md) describes (its source
  was not read there either).

Policy §5 gives each convention a name of its own, not a parameter. The code
does so as follows.

| Convention | Named in the code | At the boundary |
| --- | --- | --- |
| CLDR field patterns | `hc_format::patterns::cldr`: `format`, `parse`, `parse_with_locale` | `syntax` `cldr` of `hc_format_pattern` |
| C and POSIX `strftime`, with the GNU flags and Python's `%f` | `hc_format::patterns::strftime`: `format`, `parse`, `parse_with_locale` | `syntax` `strftime` |
| Python's `strftime` | the `strftime` engine: Python's `%f` and the seconds of `%z` are conversions of it, and `Date::strftime` and its kin call it with the `C` names | none |
| Python's `strptime` | `hc_format::python::strptime`, a matcher of its own | none; `DateTime::strptime` and `StructTime::strptime` in the facade |

So Python's `strftime` is not a fourth engine: it is the second one, which
carries Python's two extensions in the same function. Python's `strptime` is
a different engine from `strftime::parse`, and a pattern may read differently
under the two (below).

## How it works

**A CLDR pattern.** The pattern is split into literals and runs of one
ASCII letter. A run is a field; everything else, non-ASCII letters such as
`年` included, is literal; an unclosed quote is an error
(`UnterminatedLiteral`, at the apostrophe). Numeric fields write the count
as a minimum width and zero-pad. `y` and `Y` are the exception: exactly two
letters write the two low-order digits, and any other count is a minimum
width, so `y yy yyy yyyy yyyyy` is *2026 26 2026 2026 02026* [uts35-dates-48].
Name fields take the width from the count: 1 to 3 letters abbreviated, 4
wide, 5 narrow, and for weekdays 6 short. `M` and `L`, `Q` and `q`, `E`,
`e` and `c` differ in the name's form or the number's base: `M` is the format
form, the one inside a date (the genitive in Russian), and `L` the
stand-alone one. `w`, `Y` and `W` count weeks by the locale's rule: its first
day of the week and `minDays`, the fewest days of the year the first week
must hold. Week 1 is the first week with at least that many, and the weeks
between week 1 of one year and week 1 of the next are numbered 2 to 52 or 53
[uts35-dates-48]. With no locale the rule is ISO 8601's, Monday and four
days. `hc_i18n::week::WeekRule` holds the rule and the values for each
region; this document did not re-read them in CLDR's `weekData`.

**A `strftime` pattern.** A `%` starts a field: flags (`-` no padding, `_`
spaces, `0` zeros, `^` or `#` upper case), a width, `:` (for `%:z`), a
modifier `E` or `O`, and the conversion letter. Every numeric conversion has
a default width and padding: `%d` is two digits and zeros, `%e` two and
spaces, `%j` three, `%Y` four. A width replaces the default; a flag replaces
the padding. The composites expand to the POSIX `C` locale's definitions:
`%F` is `%Y-%m-%d`, `%T` and `%X` are `%H:%M:%S`, `%R` is `%H:%M`, `%D` and
`%x` are `%m/%d/%y`, `%r` is `%I:%M:%S %p` and `%c` is `%a %b %e %H:%M:%S %Y`
[posix-strftime-2024]. `%E` and `%O` are POSIX's alternative era and
alternative digits and names; they are written in the calendar the context
or the locale's `-u-ca-` key names and in the locale's digits, and otherwise
as the unmodified conversion, as POSIX says [posix-strftime-2024]. Names come
from the locale or, with none, from the `C` locale. An unknown conversion is
an error (`UnknownField`) and nothing is guessed (policy §4). Parsing walks
the pattern: whitespace matches any run of whitespace, any other literal must
match, names are matched longest first and in either case, flags and widths
are ignored but for the width of `%f`, and a `%E` or `%O` reads as the
unmodified conversion.

**Where the conventions part.** The same field has a different name, a
different default or a different meaning in each. Each row is checked against
the code (the worked example, and `crates/hc-format/tests`).

| Question | `strftime` engine | CLDR engine |
| --- | --- | --- |
| The two-digit year | `%y`: the year modulo 100, so year −43 is 57 | `yy`: the two low-order digits of the year in its era, so 44 BC is 44 |
| The year's width | `%Y`: at least four, the sign inside the width (`0005`, `-043`) | `y`: as many as it needs; `yyyy` at least four (`5`, `0005`) |
| Year zero | `%Y` is `0000` | `y` is 1 and `G` is BC; `u` is 0 |
| Day of the year | `%j`: three digits by default | `D`, `DD`, `DDD`: the count is the minimum width |
| Week of the year | `%U` from the first Sunday, `%W` from the first Monday, 0 before it; `%V` is ISO's, 1 to 53 | `w` by the locale's rule, never 0 |
| Year of the week | `%G` and `%g`: ISO's | `Y`: the locale's rule's |
| The weekday as a number | `%u` Monday is 1, `%w` Sunday is 0 | `e` and `c`: counted from the locale's first day |
| Hour 0 to 23, space-padded | `%k` | `H` is the number; there is no space-padded hour |
| Hour 1 to 24 | none | `k`: 24 at midnight (`%k` is 0 to 23) |
| The day period | `%p` the locale's AM or PM, `%P` in lower case | `a`; `b` with noon and midnight; `B` flexible periods |
| The fraction of a second | `%f` six digits, truncated; a width asks for more | `S`, count digits, truncated |
| The offset | `%z` `+0900`, `%:z` `+09:00`, the seconds kept | `x`, `X`, `Z`, `O`: five forms each |
| The month's form | `%b` and `%B` the format form; `%Ob` and `%OB` the stand-alone form | `MMM` and `LLL` |
| The era and its year | `%EC`, `%Ey`, `%EY`, in the carried calendar | `G`, `y`, `u` |
| An unknown field | an error | an error |

The year rows and the week rows are where a port goes wrong. Python's
`%y`, `%Y`, `%U` and `%W` are the `strftime` engine's, and CLDR's `w` is
neither `%U`, `%W` nor `%V`: 1 January 2021 is week 1 in `en-US` and week 0
by `%U` (below).

**Python's two engines.** Python's `strftime` conversions are those of its
documentation's table, and the crate carries every one of them, with `%f`
and `%z` as Python writes them and `%:z` for the colon form
[python-datetime-docs]. Its `strptime` does not read a pattern as
`strftime::parse` does. `strftime::parse` reads `%Y` as a signed run of up
to ten digits, so `%Y%m%d` against `2021124` fails; `python::strptime` takes
four digits for `%Y`, then `%m` tries `1[0-2]` and `%d` takes the `4`, and
reads 4 December 2021. `%j` of 366 in 2021 is refused by `strftime::parse`
and is 1 January 2022 in `python::strptime`. `%U` without a weekday is
refused by `strftime::parse` for want of a month and is read and ignored by
`python::strptime`, which dates it 1 January. The matcher's rules are in
[python-compatibility.md](python-compatibility.md).

**Worked example.** Friday 1 January 2021, 15:04:05.678 at +09:00, which is
06:04:05.678 UTC. The date is Rata Die 737 791.

- 1 January 2021 is a Friday: Rata Die 0 is a Sunday, and
  737 791 = 7 × 105 398 + 5.
- `%j` is `001`.
- `%U`: the year's first Sunday is 3 January, so 1 January is in week 0.
  `%W`: the first Monday is 4 January, week 0. `%V`: the ISO week of 1
  January runs Monday 28 December to Sunday 3 January and has three days in
  2021, fewer than four, so it is the last week of 2020. 2020 began on a
  Wednesday and was a leap year, so it has 53 weeks: `%V` is `53` and `%G` is
  `2020`.
- CLDR `w` and `Y` by `en-US`, Sunday and one day: week 1 is the week that
  contains 1 January, Sunday 27 December to Saturday 2 January, which has
  two days of 2021, at least one: `w` is 1 and `Y` is 2021. By `de`, or with
  no locale, Monday and four days, the ISO rule: 53 and 2020.
- `e` counts from the first day: Friday is 6 in `en-US` and 5 in `de`.
- `A`, milliseconds in the day: (15 × 3 600 + 4 × 60 + 5) × 1 000 + 678 =
  54 245 678. `g`, the Julian day number of the date: 2 459 216.
- `%s`: 1 609 459 200 for 2021-01-01T00:00:00Z plus 6 h 4 min 5 s,
  1 609 481 045.

| Field | `strftime` | CLDR |
| --- | --- | --- |
| Date | `%Y-%m-%d` is `2021-01-01` | `yyyy-MM-dd` is `2021-01-01` |
| Two-digit year, day of the year | `%y %j` is `21 001` | `yy DDD` is `21 001`; `D` is `1` |
| Week | `%U %W %V` is `00 00 53` | `w` is `1` in `en-US`, `53` in `de` and with no locale |
| Week year | `%G` is `2020` | `Y` is `2021` in `en-US`, `2020` in `de` |
| Weekday | `%a %A %u %w` is `Fri Friday 5 5` | `EEE EEEE` is `Fri Friday`; `e` is `6` in `en-US`, `5` in `de` |
| Hours | `%k\|%l\|%I\|%p\|%P` is `15\| 3\|03\|PM\|pm` | `H K k h a` is `15 3 15 3 PM` |
| Fraction, offset | `%f` is `678000`; `%z` is `+0900`; `%:z` is `+09:00` | `SSS` is `678`; `xxx` is `+09:00`; `x` is `+09` |
| Whole | `%Y-%m-%d %H:%M:%S.%f %z` is `2021-01-01 15:04:05.678000 +0900` | `yyyy-MM-dd'T'HH:mm:ss.SSSxxx` is `2021-01-01T15:04:05.678+09:00` |

Python 3.9.6 on this macOS, with a datetime in a +09:00 zone, wrote the same
text as the `strftime` engine for every conversion of the left column but
`%:z`, which 3.9 does not have, `%3f` and `%P`, which came out as `P`; and
for `%C` (`20`), `%s`, `%c` (`Fri Jan  1 15:04:05 2021`), `%x` and `%X`.
Going by glibc's source, `%Y` of year 5 is `5` and not `0005`, and a
conversion it does not know is copied through; this crate and macOS write
`0005`, and the crate refuses the unknown conversion
[glibc-strftime-l-source, musl-strftime-source].

## What is carried

- **`strftime`**: `%Y %C %y %G %g %m %d %e %j %H %k %I %l %M %S %f %u %w %U
  %W %V %a %A %b %B %h %p %P %z %:z %Z %s %n %t %% %F %T %R %D %r %c %x
  %X`; the flags `-`, `_`, `0`, `^` and `#`; a width; `%E` on `%c %C %x %X
  %y %Y` and `%O` on `%b %B %d %e %H %I %m %M %S %u %U %V %w %W %y`; and
  the same conversions parsed, but for `%P` which reads as `%p`, and `%s`,
  which reads a POSIX timestamp. The zone's abbreviation, for `%Z`, is the
  caller's: it is a field of `FormatContext`, because an offset does not
  determine it (`+09:00` is `JST` in Tokyo and `KST` in Seoul); with none,
  `%Z` writes `UTC` for a Zulu reading and the offset for another.
- **CLDR**: the fields `G y Y u U r Q q M L w W d D F g E e c a b B h H K k
  m s S A z Z O v V X x`, quoting, the locale's digits where its numbering
  system is positional, the week rule, the Buddhist and Minguo eras through
  `-u-ca-`, and the same fields parsed, `Q`, `q`, `W`, `F` and `A` read and
  discarded.
- **Python**: `Date::strftime`, `Time::strftime` (dated 1900-01-01, as
  Python does), `DateTime::strftime`, `ctime`, and `StructTime::strftime`,
  over the `strftime` engine; `strptime` as in
  [python-compatibility.md](python-compatibility.md).
- **The range**: the Gregorian days of `hc_calendars_solar::gregorian`, years
  −9 999 999 to 9 999 999.
- **At the boundary**: `hc_format_pattern` with `syntax` `cldr` or
  `strftime` (matched in either case): an instant in a zone and a locale. It
  has no `strptime` and no `cldr::parse`.

Not carried, each with its reason:

- Not carried: the `strftime` composites in the locale's own form. `%c`, `%x`,
  `%X` and `%r` expand to the `C` locale's pattern whatever the locale
  (`de` writes `Fr. Jan.  1 15:04:05 2021`, German names in the `C` order),
  where POSIX says the locale's representation [posix-strftime-2024].
  Not yet done; CLDR's date and time patterns are in `hc-i18n` for the
  `label` module.
- Not carried: a width on a name. `%10A` is `Friday`; glibc's manual says a
  narrower result is "right adjusted and space padded"
  [glibc-manual-strftime]. Not yet done.
- Not carried: `%10F` and `%10T` with a width. POSIX 2024 says a width on `%F`
  adjusts the year [posix-strftime-2024]; the expansion ignores it. Not yet
  done.
- Not carried: glibc's `#`, which swaps case: upper case for names, lower
  case for `%p` and `%Z` [glibc-strftime-l-source]. The crate reads `#` as
  `^`, so `%#p` is `PM`. No source read decides it; not yet done.
- Not carried: glibc's copy-through of an unknown conversion and of a `%` at
  the end. The crate refuses, which musl does too [musl-strftime-source].
- Not carried: a `%Y` without padding, as glibc writes it for years under
  1000. The crate follows musl and macOS. Under policy §5 the glibc answer
  would be a convention of its own. Not yet done.
- Not carried: validation of CLDR's field counts. UTS #35 lists `d` and `dd`,
  one `W`, and `D` to `DDD`, and calls other sequences invalid, handled "as
  described in Handling Invalid Patterns" [uts35-dates-48]. The crate
  writes `ddd` as `001`, `hhh` as `012` and `WW` as `00`. That section was
  not read; not yet done.
- Not carried: the CLDR symbols `l`, `j`, `J` and `C`. `l` "should be ignored in
  patterns" and the other three "must not occur in pattern or skeleton
  data" [uts35-dates-48]; the crate refuses all four with `UnknownField`.
  `l` should be ignored, not refused. Not yet done.
- Not carried: skeletons, `availableFormats`, the locale's date and time
  lengths and `dateTimeFormat`, and the `numbers` attribute that gives a
  field a numbering system of its own. The sources describe them
  [uts35-dates-48]; nothing in the engine generates a pattern. Not yet done.
- Not carried: a `-u-ca-` key other than the Gregorian, ISO 8601, Buddhist
  and Minguo. The engine refuses it (`Unrepresentable`) rather than write
  Gregorian months. The fields `r` and `U` of other calendars exist in
  UTS #35; the engine's months and days are Gregorian. Not yet done; the
  `label` module writes any registered calendar's dates
  ([written-dates.md](written-dates.md)).
- Not carried: lenient parsing. UTS #35 says "a lenient parse should be
  used" with resynchronisation on a literal and a try of the locale's other
  patterns [uts35-dates-48]; `cldr::parse` follows its pattern strictly:
  literals must match, and `y` reads a run of up to ten digits, so
  `yyyyMMdd` does not read `20210101`. Not yet done; no source read says how
  ICU reads adjacent numeric fields.
- Not carried: a source for the two-digit year of a parsed `yy` and `%y`. The
  crate pivots at 69, as POSIX's and Python's `strptime` do (68 is 2068 and
  69 is 1969); the pages of UTS #35 read here give no pivot, and ICU's rule
  was not read.
- Not carried: the zone fields `z`, `v` and `V` when parsing, `O` and `Z` in
  the locale's digits, and the hour of a parsed `B` (the module
  documentation lists them).

## Accuracy

Exact, as the rest of the crate: no floating point, the fraction in
attoseconds.

- **`strftime` against a C library.** 298 769 days, 1 January 1583 to 31
  December 2400, each at 15:04:05, formatted by the macOS C library's
  `strftime` and by the crate with `%Y-%m-%d %j %U %W %V %G %g %y %C %u %w
  %a %A %b %B %e %H %I %p %M %S`: no differences. The C side was given the
  weekday and the day of the year computed apart from the crate, from a
  days-from-civil formula. It checks the numbers and the `C` names of one
  libc, not glibc's flags.
- **Python.** CPython 3.9.6 on the same macOS wrote the same text as the crate
  for the conversions of the worked example's left column, `%f`, `%z` and
  `%Z` among them. It did not carry `%P`, `%^a`, `%#a`, `%#p`, `%10Y` and
  `%Q`, which came out as `P`, `^a`, `#a`, `#p`, `10Y` and `Q`: that libc, not
  Python. For year 5 it writes `0005|00|05|0005|066` for
  `%Y|%C|%y|%G|%j`, as the crate does.
- **CLDR against UTS #35's own examples.** Five of the six patterns of its
  "Date Format Pattern Examples" are written exactly: `yyyy.MM.dd G 'at'
  HH:mm:ss zzz` (*1996.07.10 AD at 15:08:56 PDT*), `h:mm a`, `hh 'o''clock'
  a, zzzz`, `K:mm a, z` (*0:00 PM, PST*) and `yyyyy.MMMM.dd GGG hh:mm aaa`
  (*01996.July.10 AD 12:08 PM*), with the zone names given to the context.
  The sixth, `EEE, MMM d, ''yy`, is *Wed, July 10, '96* in the table and
  *Wed, Jul 10, '96* here: the table's own symbol rows give `MMM` as
  abbreviated (*Sep*), so the example's *July* is its error, not the code's.
- **CLDR against ICU.** The tests' comments name Node 22.15's
  `Intl.DateTimeFormat` (ICU 76.1, CLDR 46), read 2026-10-03, for the digits
  and the Buddhist and Minguo eras, and the week tests cite UTS #35's 1998
  example, with `pt-PT` and `en-US`. No ICU run covers every field.
- **Tests.** `cargo test -p hc-format` passes: 327 unit tests, 5 in
  `python_directives.rs`, 29 in `round_trip.rs` and 10 doctests. The
  round trips are 20 000 random readings between 1583 and 2400 through four
  `strftime` patterns and four CLDR patterns, each formatted and parsed
  back to the same reading.

Where the code, its documentation and a source disagree:

- `ParsedFields::era` is documented "from `%` nothing or CLDR `G`": no
  `strftime` conversion sets it.
- `README.md` of `hc-format` cites "POSIX.1-2017 `strftime`/`strptime`"; the
  module cites IEEE Std 1003.1-2024 [posix-strftime-2024]. POSIX's `strptime`
  page was not read here, for either.
- `g` is "Modified Julian day" in UTS #35's row, which says it differs from
  the conventional Julian day number in two respects and gives 2451334 as
  its example [uts35-dates-48]. The crate writes the Julian day number of the
  local date, 2 459 216 for 2021-01-01, whose modified Julian day is 59 215;
  the example's size is the Julian day number's, and the crate follows it.
- The `Y` of a BC date is written as the signed astronomical week year
  (`-0043`) and the `y` as the year of the era (`44`); the table does not say
  which a week year in an era calendar is.
- The `syntax` names `cldr` and `strftime` are literals in
  `zone_lines::format_pattern_line`, not entries of a table that
  `convention_names.rs` holds, as policy §5 says a string name is.

## Sources

- [uts35-dates-48]: UTS #35 Part 4, version 48.2, as HTML: "Date Format
  Patterns" with its quoting rules and examples, the Date Field Symbol Table
  (letters, counts, `y`, `Y`, `D`, `g`, `l`, `j`, `J`, `C`), "Week of Year",
  and the opening of "Parsing Dates and Times". Read 2026-10-03. "Handling
  Invalid Patterns", cited by the table, was not read.
- [posix-strftime-2024]: the Open Group Base Specifications Issue 8,
  `strftime`: the conversions, the `%U`, `%V` and `%W` rules, the `C` locale's
  `%c`, `%x`, `%X` and `%r`, `%F` and `%C` with widths, the modifiers. Read
  2026-10-03. The page for `strptime` was not read.
- [glibc-manual-strftime]: the GNU C Library manual's "Formatting Calendar
  Time": the flags `_`, `-`, `0` and `^`, the width, `%P`, `%k`, `%l`, `%s`,
  `%e`, `%C`. Read 2026-10-03. The manual lists no `#` and no `:`.
- [glibc-strftime-l-source]: glibc's `time/strftime_l.c`, at the head of its
  repository, read as text: the flags including `#`, the width, `DO_NUMBER`
  defaults (`%Y` and `%G` with a minimum of one digit, `%C`, `%y`), `%P`, `%z`
  and the copy-through of an unknown conversion. Read 2026-10-03.
- [musl-strftime-source]: musl's `src/time/strftime.c`, read as text: the
  conversions, the flags `-`, `_`, `0` and `+`, `%Y` at least four digits, the
  failure on an unknown conversion. Read 2026-10-03.
- [python-datetime-docs]: the platform dependence, and the directive table
  with `%f`, `%z`, `%Z`, `%U`, `%W`, `%c`, `%x`, `%X` and the `%-d` forms,
  from the Python 3.13 page read 2026-10-03. The fetch returned neither the
  table of `%G`, `%u` and `%V` nor the numbered notes; the tests cite the
  page as read on 2026-09-26.
- CPython 3.9.6 and the macOS C library, run locally as oracles. CLDR's
  `weekData`, ICU, gnulib (where `%:z` is) and musl's documentation were not
  read; the week rule's values are `hc-i18n`'s.

## Code

`crates/hc-format/src/patterns.rs` (`FormatContext`, `Fields`,
`ParsedFields` and the `C` names), `patterns/strftime.rs`, `patterns/cldr.rs`,
`patterns/zone.rs` and `python.rs`. Tests that anchor it: in `strftime.rs`,
`the_week_and_day_of_year_fields_agree_with_the_calendar`,
`the_padding_flags_do_what_they_say`, `an_explicit_width_overrides_the_default`,
`the_era_modifier_writes_the_locales_alternative_era` and
`the_alternative_modifier_writes_the_locales_alternatives`; in `cldr.rs`,
`a_two_letter_year_is_the_last_two_digits_and_nothing_else_is`,
`the_week_fields_follow_the_locales_week_rule`,
`the_hour_fields_differ_at_midnight_and_noon` and
`a_week_date_in_a_locales_rule_round_trips`; in `patterns.rs`,
`the_sunday_and_monday_week_numbers_differ_where_they_should`;
`tests/python_directives.rs` (every directive of Python's table) and
`tests/round_trip.rs`
(`strftime_format_then_parse_is_the_identity_over_the_whole_range`,
`cldr_format_then_parse_is_the_identity_over_the_whole_range`,
`strftime_and_iso_8601_never_disagree_about_a_date`).

The facade's adapters are in `crates/hyper-calendar/src/civil.rs` and
`civil/struct_time.rs` (`docs/python-parity.md` lists them), and the boundary's
in `crates/hyper-calendar/src/zone_lines.rs` (`format_pattern_line`) and
`exports.rs` (`hc_format_pattern`); its description for the WebAssembly
module is under "Formatting by pattern" in
`crates/hyper-calendar-wasm/README.md`.
