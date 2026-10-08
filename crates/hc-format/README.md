# hc-format

Parsing and formatting for `hyper-calendar`: ISO 8601-1:2019, RFC 3339,
RFC 5322 (email and HTTP dates), and the two pattern vocabularies —
`strftime`/`strptime` and CLDR / Unicode TR 35 field patterns.

Formatting writes into a `core::fmt::Write` sink, so a `no_std` caller
renders into its own buffer. `String`-returning conveniences sit behind the
`alloc` feature.

## What it does

* **`iso8601`** — the standard, not a profile of it. Both directions, with the
  written shape preserved so that parse → format is byte-identical.
* **`rfc3339`** — the internet profile, including the `-00:00` of §4.3.
* **`rfc2822`** — RFC 5322 §3.3, with the obsolete syntax §4.3 requires a
  parser to accept, plus RFC 9110 `IMF-fixdate` output.
* **`patterns`** — both pattern languages, formatting and parsing, with names
  routed through `hc-i18n` when a locale is supplied.
* **`parse`** — a sniffing front door for when all you have is "a date string".
* **`fat`** — the MS-DOS date and time words of the FAT file system, a local
  reading from 1980 to 2107 at two-second resolution, decoded to a
  `CivilDateTime` in no zone and encoded from one.
* **`ccsds`** — the CCSDS Calendar Segmented Code and the ASCII codes A and
  B, UTC readings with the leap second checked against the table, and a
  front door that reads any binary CCSDS code from its P-field. The binary
  counts, CUC and CDS, are `hc-core::ccsds`. See
  `docs/systems/ccsds-time-codes.md`.
* **`radio`** — a minute's frame of the JJY, DCF77 and WWVB time codes,
  WWVB's phase code included, decoded with its parity checked and encoded,
  and read as the JST, CET or CEST, or UTC minute it names; DCF77's zone
  and A1 (`dcf77::summer_time`), WWVB's summer-time bits
  (`wwvb::DstState::of_day`) and the phase code's `dst_next`
  (`wwvb::DstNext::of_day`) can be read from a `hc_tz::TimeZone`'s rules
  instead of the caller. See `docs/systems/radio-time-codes.md`.
* **`irig`** — a frame of the IRIG serial time codes A, B, D, E, G and H:
  the BCD time of year, the year, the control bits and the straight binary
  seconds, decoded with every marker and field checked and encoded, for
  the coded expressions IRIG Standard 200-16 permits each format. See
  `docs/systems/irig-time-codes.md`.
* **`east_african_hours`** — the Ethiopian and Swahili hours, the civil
  clock read six hours on over a day half and a night half, as two named
  reckonings. See `docs/systems/hours-of-the-day.md`.
* **`night_watches`** — 更点, the Chinese night from 19:00 to 05:00 in five
  watches of two hours and five points of 24 minutes, read from the civil
  clock. The seasonal reckoning, fifths of the night from dusk to dawn, is
  not carried.
* **`label`** — a calendar's eras, years, months, days and dates written as
  a locale writes them, for any registered calendar, and `parse_date`, its
  inverse: 令和8年9月28日, 康熙五十二年十一月初一, *28 Eylül 2026* and
  ٢٨ سبتمبر ٢٠٢٦ read back as the calendar's fields and the fixed day. The
  reader walks the same templates, reads every name at every width and
  native, Latin and Han numerals, and refuses a text that is not one day —
  two days read, a year of two digits, a year named only by a recurring
  cycle, a weekday that is not the day's — with the reason. See
  `docs/systems/written-dates.md`.

## ISO 8601-1:2019 coverage

| Representation | Extended | Basic | Notes |
| --- | --- | --- | --- |
| Calendar date `2026-09-21` | yes | yes | |
| Month `2026-09` | yes | **no** | `YYYYMM` is forbidden by the standard: it collides with the obsolete `YYMMDD`. |
| Year `2026` | yes | n/a | No separators to drop. |
| Ordinal date `2026-264` | yes | yes | |
| Week date `2026-W38-1` | yes | yes | |
| Week `2026-W38` | yes | yes | |
| Expanded year `+002026`, `-000500` | yes | **no** | In the basic format the digits are split by the workspace's count; see "What it refuses, and what it does not carry". |
| Time `14:30:05`, `14:30`, `14` | yes | yes | |
| Fraction on any component, `14.5`, `14:30,5`, `14:30:05.123456789` | yes | yes | Both the `.` and the `,` decimal mark. |
| `24:00` end of day | yes | yes | Kept distinct from `00:00` of the next day. |
| `23:59:60` leap second | yes | yes | Where the zone's clock reads UTC's `23:59:60`, `15:59:60-08:00` as well as `23:59:60Z` (RFC 3339 §5.7, §5.8), on a day the leap-second table says ended in an inserted second; see below. |
| Zone `Z`, `+09`, `+09:00`, `+0900`, `+09:00:00`, `+090000` | yes | yes | |
| Unqualified local time | yes | yes | Stays unqualified. It is not UTC. |
| Duration `P3Y6M4DT12H30M5S`, `PT0.5S`, `P1W` | yes | yes | |
| Alternative duration `P0003-06-04T12:30:05` | yes | yes | |
| Interval `start/end`, `start/duration`, `duration/end`, `duration` | yes | yes | |
| Repeating interval `R5/PT1H/…`, `R/…` | yes | yes | |

Reduced accuracy is supported everywhere the standard allows it, and
`Strictness` lets a caller refuse it. `Strictness::ISO` accepts everything
above; `Strictness::FULL` demands a complete extended-format date-time with a
zone; `Strictness::RFC_3339` is the internet profile. Every field of
`Strictness` is an independent permission, because the axes really are
independent.

## What it refuses, and what it does not carry

Most of these are refusals rather than guesses (policy §4); the others say
what they are.

* **Truncated representations** (`--09-21`, `---21`, `-26-09-21`). ISO
  8601:2000 had them; ISO 8601:2004 removed them and ISO 8601-1:2019 does not
  bring them back. Accepting them would mean inferring a century from nothing.
* **Expanded years in the basic format** (`+0020260921`). ISO 8601 leaves the
  digit count of an expanded year "to agreement between the communicating
  parties". In the extended format a separator says where the year stops, and
  in a week date the `W`; in the basic format nothing does, so the same string
  is a six-digit year plus a month-day or an eight-digit year plus a
  day-of-year. This reads the digits by the count the workspace writes,
  `hc_calendar::gregorian::expanded_year_digits`, so that a date written by
  one crate reads back in another: nine digits are a six-digit year and a day
  of the year (`-000001001`), ten a six-digit year, a month and a day
  (`+0123450607`), eleven a seven-digit year, a month and a day; eight or
  fewer are the year alone, twelve or more are refused. The one spelling this
  cannot give back, the ordinal date of a year beyond six digits, which
  would read as another year's month and day, the writers refuse.
* **Reduced end-of-interval representations** (`2026-09-21T14:00/16:00`,
  where the end inherits the start's high-order components). The inheritance
  rule is ambiguous whenever the start is itself of reduced accuracy, and
  getting it wrong produces an interval of the wrong length without any
  error. Both ends must be complete.
* **Not carried: `--` as an interval separator.** ISO 8601 permits it where
  `/` cannot be used. Not yet done; `/` is always available here.
* **A second 60 that is no leap second, or on a day with none.** A positive
  leap second is inserted at the end of a UTC day, so `23:59:60Z` is real on
  the 27 days `hc_core::leap` lists, from 1972-06-30 to 2016-12-31, and a
  zone's clock reads the same physical second at that time less its offset
  (RFC 3339 §5.7): `1990-12-31T15:59:60-08:00`, §5.8's own example, and
  Tokyo's `2017-01-01T08:59:60+09:00` are read, held by
  `hc_calendar::CivilTime::leap_second` at their own hour and minute.
  `hc-format` refuses a second 60 whose reading shifted to UTC is not
  `23:59:60`, as `23:59:60+01:00` or `15:59:60+08:00`, one on a day the table
  says had none, and, as no one has announced them, on a day past the
  table's validity. A reading with no zone names no instant, but its clock is
  read as UTC's: the second 60 is at `23:59`, and its day must be one the
  table says had one, as for a zoned reading.
* **More than 18 fractional digits.** An attosecond is this library's
  resolution. Truncating further digits would produce a value that no longer
  round-trips, so it is an error instead.
* **Carry limits in the alternative duration form.** `P0003-99-04` parses:
  the shape is checked, the magnitudes are not.
* **Other calendars.** ISO 8601 is Gregorian. Hijri, Hebrew and the rest are
  `hc-calendars-*`'s subject, not this crate's, and a pattern is not the
  place to reach them.

## RFC 3339

Stricter than ISO 8601 in every direction: no basic format, no reduced
accuracy, no expanded years, no `24:00`, no week or ordinal date, an offset
of an hour 00 to 23 and no seconds, a mandatory zone. It is looser in
three ways, all of which are honoured: the lowercase `t` and `z`, the space
separator of §5.6's note, and `-00:00`.

`-00:00` means the timestamp is known and is stated in UTC, but the offset of
the zone that produced it is unknown. `+00:00` asserts the opposite. The two
parse to different `ZoneInfo` values and round-trip as themselves.

## RFC 5322 / RFC 2822

The parser accepts what §4.3 says a parser must: two-digit years (00–49 →
2000s, 50–99 → 1900s) and three-digit years (+1900), the named zones `UT`,
`GMT`, `EST`, `EDT`, `CST`, `CDT`, `MST`, `MDT`, `PST`, `PDT`, single-letter
military zones and any other unrecognised alphabetic zone (all of which mean
"offset unknown"), optional seconds, an optional day-of-week, and comments
and folding whitespace between every pair of tokens, nested arbitrarily.

The writer emits none of that: four-digit year, numeric offset, seconds
always present, day-of-week derived from the date rather than copied from the
input. `write_imf_fixdate` produces the HTTP `Date` shape with the literal
`GMT`. The zone is `+hhmm`, with no digits for seconds, so an offset with
seconds, which would write another instant than the one the wall clock
names, is refused; `iso8601::write` and `rfc3339::write` refuse it too.

## Patterns

`strftime` supports `%Y %C %y %G %g %m %d %e %j %H %k %I %l %M %S %f %u %w
%U %W %V %a %A %b %B %h %p %P %z %:z %Z %s %n %t %% %F %T %R %D %r %c %x %X`,
the flags `-` `_` `0` `^` `#` and an explicit field width, and POSIX's
modifiers (IEEE Std 1003.1-2024, "Modified Conversion Specifiers",
[posix-strftime-2024]): `%EC`, `%Ey`, `%EY`, `%Ex` and `%Ec` write the era,
year and date of the calendar `FormatContext::era_calendar` names, or of
the Buddhist or Minguo calendar a locale's `-u-ca-` key names
(`th-u-ca-buddhist` writes `%EY` as พ.ศ. 2569; the Japanese eras, in
`hc-calendars-regional`, which this crate does not depend on, are given
by the caller, as the facade's `hc_format_pattern` does for
`-u-ca-japanese`), `%EX` is `%X`, `%Ec` joins `%Ex` and `%X` by the
locale's date-time format, and a locale
with no such calendar writes the unmodified conversion, as POSIX says;
`%c`, `%x`, `%X` and `%r` are the POSIX locale's definitions with no
locale and, with one, CLDR 48's medium date, time and date-time formats
and its `hms` item from `hc-i18n`'s `formats` (`de` writes `%x` as
*21.09.2026* and `%c` as *21.09.2026, 14:30:05*; `ja` *2026/09/21* and
*2026/09/21 14:30:05*), the calendar being the one the locale's `-u-ca-`
key names among the Gregorian, Buddhist and Minguo;
`%Ob` and `%OB` write the stand-alone month, POSIX's nominative
`alt_mon` (Russian's *сентябрь* for *сентября*); and `%Od` … `%Oy` write the
number in the locale's alternative digits, CLDR 48's `native` system where
it is not the usual one, else its `traditional` one (Devanagari for `hi`,
Han numerals for `ja`, Hebrew for `he`). The Japanese eras need the
calendar passed in, since this crate does not depend on the nengō table.
`strptime` reads a modified conversion as the unmodified one. `%f` is Python's: six digits of
microseconds, or as many as a width asks for, truncated. `%z` keeps the
seconds of an offset that has them (`+063415`), as Python does. When
parsing, `%U` and `%W` fix a date only together with a year and a weekday,
Python's rule; on their own they are read and discarded, because a week
number without a weekday names seven days.

## Python's profile

`python` is the ISO 8601 profile of CPython's `datetime`: `fromisoformat` for
dates, times and date-times, accepting Python's documented forms (not
reduced, expanded or ordinal dates; not fractions of an hour or minute; a week
without a day is its Monday; any single character between date and time);
`isoformat` with `TimeSpec`, the `timespec` argument; `ctime`; and `strptime`,
which is CPython's `_strptime` as code — each directive's alternatives in
Python's order, matched with backtracking, so `%Y%m%d` reads `20191204` and
`%j` of 366 in a common year rolls into the next year. It keeps up to eighteen
fraction digits where Python truncates to six and accepts `23:59:60` and year
0; it refuses offsets with a fraction of a second, which Python accepts, and
an offset of 24 hours or more. The behaviours of CPython's C parser that the
documentation does not describe, and a few CPython 3.14 additions, are not
copied; `docs/python-parity.md` lists them, and
`docs/systems/python-compatibility.md` explains the matcher and gives the
measured agreement. `tests/python_directives.rs` checks every directive
Python documents against that table's own sample.

CLDR supports `G y Y u U r Q q M L w W d D F g E e c a b B h H K k m s S A
z Z O v V X x` and `'` quoting, UTS #35 Part 4, version 48.2
[uts35-dates-48]. In these Gregorian patterns `U`, with no cyclic year
names, "behaves like `y`", `r` "is the same as the `u` year", and `g` is
the Julian day number of the local date, 2451545 for 2000-01-01. `b` writes
*noon* and *midnight* at 12:00:00 and 00:00:00 where the locale's language
has them and am or pm otherwise, and `B` the flexible period of the
language's rules, *in the afternoon*, *at night*, from CLDR 48's
`dayPeriods.xml` (`hc_i18n::day_periods`). The zone fields follow "Using
Time Zone Names": `z` the specific name, *Pacific Daylight Time* (a name
given to the context first), `v` the generic one, *Pacific Time*, *Pacific
Time (Canada)* for Vancouver, `V` the short identifier `uslax`, `VV` the
zone as given, `VVV` its exemplar city and `VVVV` its location, *Italy
Time*, `O` and `ZZZZ` the localized GMT format in the locale's words and
digits, *UTC+2* in French, each falling back as UTS #35 lists; the context
takes the zone's identifier (`with_zone_id`) and whether the reading is
daylight time (`with_daylight`). The names are the `zone-names` feature,
English's with root's, and `localized-zone-names`, every other locale's;
without them the fields take their fallbacks.
`docs/systems/zone-names.md` works the rules through. `z`, `v` and `V` are
not resolved when parsing: mapping an abbreviation back to a zone is not
possible — `CST` is three different zones — so those fields are consumed
and the zone left unstated unless RFC 5322 assigns the name an offset. `B`
parsed gives am or pm where its period lies wholly on one side of noon.

The numeric fields are written in the locale's default numbering system
where it is positional (`d MMMM y` is *٢١ سبتمبر ٢٠٢٦* in `ar-EG`, as ICU
writes it) and read back in it or in Latin digits; `O` and `Z` still read
Latin digits only. `w`, `Y` and `W` count weeks by the locale's week rule,
`hc_i18n::week::for_locale` (an `hc_calendar::week::WeekRule`): its first day of the week (`-u-fw-` first) and CLDR
48's `minDays` for its region, so 1 January 2021 is *2021-W1* in `en-US` and
*2020-W53* in `de`, ISO 8601's Monday and four days being the rule with no
locale; `W` is 0 for a day before its month's first week, as ICU4J numbers
it. `-u-ca-buddhist` and `-u-ca-roc` write `G`, `y`, `u` and `U` in the
Buddhist or Minguo calendar (*พ.ศ. 2569*), and a `-u-ca-` key that names any
other calendar, with no calendar given to the context, is refused, because
these patterns write only the Gregorian months and days. `Z` to `ZZZ` write
the seconds of an offset that has them, as `xxxx` does, and `O` and `OOOO` write
the locale's `GMT` for a zero offset.

With no locale, names are the POSIX `C` (English) ones, which is what
`strftime` without `setlocale` gives and what a protocol field needs. With a
`hc_i18n::Locale`, every name comes from `hc-i18n`, and a parser accepts both
the locale's names and the `C` ones.

## Errors

Every `ParseError` carries an `ErrorKind` — what was expected — and a byte
offset into the original input, including for values nested inside an
interval. There is no "invalid date" variant.

## Accuracy and range

Exact. There is no floating point anywhere in this crate: sub-second values
are integer attoseconds and offsets are integer seconds. The supported day
range is whatever `hc_calendars_solar::gregorian` supports (years
−9 999 999 to 9 999 999); the four-digit forms of RFC 3339 and RFC 5322 are
additionally limited to years 0000–9999, and say so when asked to write a
year outside it.

## Reference data

* ISO 8601-1:2019 with Amendment 1:2022 (which restores `24:00` for the end
  of a day) and ISO 8601-2:2019 (the signed duration, accepted as a
  documented extension). The ISO texts were not read; what each allows is
  taken from Wikipedia's "ISO 8601", read 2026-09-26.
* Roman dates: Reingold and Dershowitz, *Calendrical Calculations*, 4th ed.
  (2018), chapter 3, as their code (`reingold2018code`) states the rules.
* RFC 3339 (§4.2 offsets, §4.3 unknown local offset, §5.6 grammar).
* RFC 5322 §3.3 and §4.3; RFC 9110 §5.6.7 for `IMF-fixdate`.
* UTS #35 (version 48.2), Part 4, *Date Field Symbol Table*, for the CLDR pattern letters.
* POSIX.1-2017 `strftime`/`strptime`, plus the GNU flag extensions.
* IERS Bulletin C for the leap second of 30 June 1972, which the tests anchor
  against.

## Tests

The property tests in `tests/round_trip.rs` take every day from
1583-01-01 to 2400-12-31 through all three ISO date forms in both formats,
every second of a day through the time grammar, every whole-minute offset
through every offset style, and tens of thousands of generated date-times,
durations, intervals and patterns through format-then-parse. The generator is
a fixed-seed LCG so that a CI failure reproduces exactly.
