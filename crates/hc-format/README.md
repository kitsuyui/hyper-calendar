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
  parser to accept, plus RFC 7231 `IMF-fixdate` output.
* **`patterns`** — both pattern languages, formatting and parsing, with names
  routed through `hc-i18n` when a locale is supplied.
* **`parse`** — a sniffing front door for when all you have is "a date string".

## ISO 8601-1:2019 coverage

| Representation | Extended | Basic | Notes |
| --- | --- | --- | --- |
| Calendar date `2026-09-21` | yes | yes | |
| Month `2026-09` | yes | **no** | `YYYYMM` is forbidden by the standard: it collides with the obsolete `YYMMDD`. |
| Year `2026` | yes | n/a | No separators to drop. |
| Ordinal date `2026-264` | yes | yes | |
| Week date `2026-W38-1` | yes | yes | |
| Week `2026-W38` | yes | yes | |
| Expanded year `+002026`, `-000500` | yes | **no** | See "Deliberately not supported". |
| Time `14:30:05`, `14:30`, `14` | yes | yes | |
| Fraction on any component, `14.5`, `14:30,5`, `14:30:05.123456789` | yes | yes | Both the `.` and the `,` decimal mark. |
| `24:00` end of day | yes | yes | Kept distinct from `00:00` of the next day. |
| `23:59:60` leap second | yes | yes | Only at `23:59`; see below. |
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

## Deliberately not supported

* **Truncated representations** (`--09-21`, `---21`, `-26-09-21`). ISO
  8601:2000 had them; ISO 8601:2004 removed them and ISO 8601-1:2019 does not
  bring them back. Accepting them would mean inferring a century from nothing.
* **Expanded years in the basic format** (`+0020260921`). ISO 8601 leaves the
  digit count of an expanded year "to agreement between the communicating
  parties". In the extended format a separator says where the year stops; in
  the basic format nothing does, so the same string is a six-digit year plus
  a month-day or an eight-digit year plus a day-of-year. This refuses to
  guess rather than picking one silently.
* **Reduced end-of-interval representations** (`2026-09-21T14:00/16:00`,
  where the end inherits the start's high-order components). The inheritance
  rule is ambiguous whenever the start is itself of reduced accuracy, and
  getting it wrong produces an interval of the wrong length without any
  error. Both ends must be complete.
* **`--` as an interval separator.** ISO 8601 permits it where `/` cannot be
  used; `/` is always available here.
* **Leap seconds outside `23:59`.** A positive leap second is inserted at the
  end of a UTC day, so `23:59:60Z` is real; the same physical second read in
  Tokyo is `08:59:60`, which `hc_calendar::CivilTime` cannot hold. Rather
  than silently shifting it, `hc-format` refuses it and says so.
* **More than 18 fractional digits.** An attosecond is this library's
  resolution. Truncating further digits would produce a value that no longer
  round-trips, so it is an error instead.
* **Carry limits in the alternative duration form.** `P0003-99-04` parses:
  the shape is checked, the magnitudes are not.
* **Other calendars.** ISO 8601 is Gregorian. Hijri, Hebrew and the rest are
  `hc-calendars-*`'s subject, and a pattern is not the place to reach them.

## RFC 3339

Stricter than ISO 8601 in every direction: no basic format, no reduced
accuracy, no expanded years, no `24:00`, a mandatory zone. It is looser in
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
`GMT`.

## Patterns

`strftime` supports `%Y %C %y %G %g %m %d %e %j %H %k %I %l %M %S %f %u %w
%U %W %V %a %A %b %B %h %p %P %z %:z %Z %s %n %t %% %F %T %R %D %r %c %x %X`,
the flags `-` `_` `0` `^` `#` and an explicit field width. Not implemented:
the `%E…`/`%O…` locale-alternative modifiers. `%f` is Python's: six digits of
microseconds, or as many as a width asks for, truncated. `%z` keeps the
seconds of an offset that has them (`+063415`), as Python does. When
parsing, `%U` and `%W` fix a date only together with a year and a weekday,
Python's rule; on their own they are read and discarded, because a week
number without a weekday names seven days.

## Python's profile

`python` is the ISO 8601 profile of CPython's `datetime`: `fromisoformat` for
dates, times and date-times, accepting exactly Python's forms (not reduced,
expanded or ordinal dates, not fractions of an hour or minute, any single
character between date and time); `isoformat` with `TimeSpec`, the
`timespec` argument; `ctime`; and `strptime` with Python's 1900-01-01
defaults. It keeps up to eighteen fraction digits where Python truncates to
six and accepts `23:59:60`; it refuses offsets with a fraction of a second,
which Python accepts. `tests/python_directives.rs` checks every directive
Python documents against that table's own sample.

CLDR supports `G y Y u Q q M L w W d D F E e c a h H K k m s S A z Z O X x`
and `'` quoting. Not implemented: `b`/`B` (flexible day periods, which need
per-locale hour ranges), `v`/`V` (metazones), `U` (cyclic year names), `r`
(related Gregorian year) and `g` (modified Julian day). `z`, `v` and `V` are
not resolved when parsing: mapping an abbreviation back to a zone is not
possible — `CST` is three different zones — so those fields are consumed and
the zone left unstated unless RFC 5322 assigns the name an offset.

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

* ISO 8601-1:2019 and ISO 8601-2:2019 (the signed duration, accepted as a
  documented extension).
* RFC 3339 (§4.2 offsets, §4.3 unknown local offset, §5.6 grammar).
* RFC 5322 §3.3 and §4.3; RFC 7231 §7.1.1.1 for `IMF-fixdate`.
* Unicode TR 35, *Date Field Symbol Table*, for the CLDR pattern letters.
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
