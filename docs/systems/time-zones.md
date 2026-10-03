# Time zones: UTC offsets, POSIX TZ strings, TZif files and the three answers of a local time

Backs `hc-tz` (`offset`, `zone`, `fixed`, `posix`, `tzif`, `system`,
`builtin`) and `hyper_calendar::zone_lines` (`zone_offset`, `load_zone`,
`with_zone`, `day_in_zone`, `start_in_zone`), and through them the exports
`hc_zone_load`, `hc_fixed_from_unix_in_zone`, `hc_unix_from_fixed_in_zone`
and `hc_zone_offset`, and the zone that `hc_zone_name`, `hc_format_pattern`
and the `zone:` summer-time source of `hc_radio_encode` read. No calendar
identifier is registered: a zone is a rule between an instant and a wall
clock. Where a zone is, and what its city is called, is
[zone-locations.md](zone-locations.md); what CLDR calls it in a locale is
[zone-names.md](zone-names.md). This document does not repeat either.

## What it is

A civil time zone is the rule that says what a wall clock reads at an
instant. It is a law, not a physical fact: a government sets an offset from
UTC, sometimes a second one for the summer, and may change either.
The IANA time zone database records the rules of "many representative
locations" and their history [iana-tz-link]. It names a zone
`Area/Location` after a city or small island, so that a political change
does not rename it, and it warns that pre-1970 entries "cover only a tiny
sliver" of what clocks did, and that its predictions of the future "will be
incorrect after future governments change the rules" [iana-tz-theory].
Abbreviations such as `CST` are ambiguous (China and North America), and
where there is no common English one the database writes a numeric `-03`
or `+0545` [iana-tz-theory].

Three kinds of rule are in use, and `hc-tz` holds all three behind one
trait, `TimeZone`:

- **A fixed offset**, `UtcOffset`: whole seconds east of UTC.
- **A POSIX `TZ` string**, such as `EST5EDT,M3.2.0,M11.1.0`: a standard
  offset and, optionally, a daylight offset with two yearly rules. It has
  no history. POSIX writes the offset with the opposite sign, positive to
  the west [posix-2024-tz].
- **A TZif file**, the IANA database's binary format [rfc9636]: every
  recorded transition, each with an offset, a daylight flag and an
  abbreviation, then a POSIX string, the *footer*, for the time after the
  last record.

Instant to wall clock is a function. Wall clock to instant is not: when
clocks go back, a reading names two instants; when they go forward, a
reading names none. The library gives the caller all three cases and never
picks one silently (policy §4).

## How it works

### A UTC offset

An offset is an integer number of seconds, positive east of Greenwich,
within ±25:59:59 (±93 599 s). RFC 9636 §3.2 says a TZif `utoff` SHOULD lie
in [−89 999, 93 599] [rfc9636], and the window here contains it, so that
the local-mean-time offsets of the database, which carry seconds, read
back. Everything works in POSIX time, where every day has 86 400 seconds:
the timeline civil rules are published in. Elapsed physical time across a
leap second is `hc_core::unix`'s business.

### A POSIX TZ string

The grammar is `std offset [dst [offset] [,start[/time],end[/time]]]`
[posix-2024-tz]. `std` and `dst` are three or more characters, or
`<`...`>` quoting that admits digits and signs (`<+0545>`). A `dst` with no
offset is one hour ahead of standard [posix-2024-tz]. A rule is one of
three forms [posix-2024-tz]:

- `Jn`, the day `n` of 1 to 365 with 29 February never counted;
- `n`, the zero-based day 0 to 365 with 29 February counted;
- `Mm.w.d`, weekday `d` (Sunday 0) of week `w` of month `m`, where week 5
  means the last such weekday, in the fourth week or the fifth
  [posix-2017-tz].

The day a rule names is computed on the proleptic Gregorian calendar by
`hc_calendar::gregorian` (policy §2), whose arithmetic is Reingold and
Dershowitz's [reingold2018], not read here. `time` is the local time of the
change, 02:00:00 by default [posix-2017-tz]. It is read in the offset in
force just before the change: the standard offset at the start of summer
time, the daylight offset at its end. POSIX.1-2017 allows hours 0 to 24 and
no sign [posix-2017-tz]; RFC 9636 §3.3.2 widens it for a version 3 file to
−167 to 167, as `M3.5.0/-2` [rfc9636]; and POSIX.1-2024 reads the time to
167 hours too [posix-2024-tz]. The parser takes the wider range. If a `dst`
has no rule at all, POSIX leaves the dates to the implementation
[posix-2024-tz], and the parser then uses the United States rules,
`M3.2.0,M11.1.0`; the code's comment names tzcode's default string as its
source, which was not read here.

Two consequences matter. A start later than its end in the same year is a
southern-hemisphere zone, whose summer straddles the new year. And a time
past 24:00 is an ordinary spelling, not an error: Egypt's
`EET-2EEST,M4.5.5/0,M10.5.4/24` ends summer time at 24:00 on the last
Thursday of October, so in 2026 (Thursday 29 October) the end is
24:00 EEST, 21:00 UTC, 1 793 307 600, and the wall clock goes from 24:00
back to 23:00. RFC 9636 §3.3.1 spells permanent summer time as
`XXX3EDT4,0/0,J365/23`, a period from 1 January 00:00 to 31 December 23:00
in the daylight offset, which ends where the next begins [rfc9636]; the
parser accepts it and reports no transition at all.

### A TZif file

A file is a 44-byte header (magic `TZif`, a version byte and six counts), a
data block, and, from version 2 on, the same again with 64-bit times
followed by `"\n"`, a POSIX string and `"\n"` [rfc9636]. The data block is
the transition times, a type index for each, the local time types
(`utoff`, `isdst`, an index into the abbreviation text), the abbreviation
text, leap-second records, and the standard/wall and UT/local indicators.
RFC 9636 §4 says a reader SHOULD skip the version 1 block of a version 2
or later file [rfc9636]; the reader here does, and reads the 64-bit block.
Parsing borrows from the byte slice and allocates nothing. It refuses a
zero type count, indicator counts that are neither 0 nor the type count,
transitions not in strictly ascending order, an `isdst` above 1, an offset
outside the window, an abbreviation index outside the text and a footer not
between two newlines.

Which rule governs an instant:

- At or after the last recorded transition, the footer, if it is present
  and not empty; RFC 9636 §3.3 says the footer governs "on or after" the
  last transition [rfc9636]. This is what extends a table to every later
  year. Without a footer the last type persists: for the version 1 fixture
  of New York in the `tzif` tests, every later summer is standard time.
- Between two transitions, the type of the transition in force, found by
  binary search.
- Before the first transition, RFC 9636 §3.2 says time type 0 [rfc9636].
  The reader takes the first type that is not a daylight type, and type 0
  when every type is. The two agree for every file of the system database
  checked below, and differ for a file whose type 0 is a daylight type
  (Accuracy).

The leap-second records and the two indicator arrays are read and reported,
and not applied (see What is carried). RFC 9636 §3.1 defines versions 1 to
4: version 2 adds the 64-bit block and the footer, version 3 the extended
footer, and version 4 only "truncated at the start" leap-second records and
an expiration time [rfc9636]. The reader accepts 1 to 3 and refuses a `4`
byte (Accuracy).

### From a local reading to an instant

The resolver needs nothing but a zone's instant-to-offset function, so it
serves a fixed offset, a POSIX string and a TZif file alike. Take the
reading as if it were UTC and collect the offsets the zone keeps on each of
the five days from two before to two after. For each offset, the candidate
instant is the reading minus the offset, kept only if the zone really keeps
that offset at the candidate. One survivor is `Unambiguous`; two are
`Ambiguous { earlier, later, earlier_offset, later_offset }`; none is
`Nonexistent`. For a nonexistent reading the resolver scans the two days
either side in steps of an hour, bisects to the exact second of the offset
change, and reports `gap_start` (the reading the clocks jumped from),
`gap_end` (the reading they jumped to), the instant, both offsets, and the
two instants the reading would have under each offset: `before_gap`, which
is before the gap opened, and `after_gap`, at or after it closed. Their
difference is the length of the gap.

`Disambiguation` reduces the answer to one instant, and has no `Default`:

| Policy | Repeated reading | Skipped reading |
| --- | --- | --- |
| `Earliest` | the first occurrence | `before_gap` |
| `Latest` | the second occurrence | `after_gap` |
| `PushForward` | the first occurrence | `after_gap`, the reading plus the gap |
| `Reject` | `AmbiguousLocalTime` | `NonexistentLocalTime` |

`PushForward` is the `compatible` policy of ECMAScript Temporal and the
default of `java.time` (the crate's own description; not read here).

### The daylight flag and summer time

`is_dst_at` is the zone's own flag, from the rule or the file's `isdst`; it
is not a comparison of offsets. The tz database's main format gives Ireland
a negative saving, so `Europe/Dublin` is `IST` at UTC+1 in summer with flag
0 and `GMT` at UTC+0 in winter with flag 1 [tz-europe-eire-rules]; its
footer is `IST-1GMT0,M10.5.0,M3.5.0/1`. `is_summer_time_at` answers the
question the names ask. It is the flag, unless every reading with the other
flag within 366 days either side has a lower offset, when the instant is
summer time whatever its flag, or every one has a higher offset, when it is
not. For Dublin on 2026-07-01 12:00 UTC the offset is 3 600 s, the flag is 0
and summer time is true; on 2026-01-15 12:00 UTC the offset is 0, the flag
is 1 and summer time is false. [zone-names.md](zone-names.md) says why the
names need it.

### The next transition

`next_transition(utc)` is the first instant strictly after `utc` at which
the offset, the flag or the abbreviation changes, else `None`. A fixed
offset has none. A POSIX string takes the starts and ends of the years
around `utc` and keeps one only where the flag a second earlier differs,
which is why `XXX3EDT4,0/0,J365/23` has none. A TZif file walks its
recorded transitions, passing over any that change nothing, and then asks
its footer from the later of `utc` and the last record.

**Worked example.** `America/New_York` on 10 March and 3 November 2024.

1. The rule is `EST5EDT,M3.2.0,M11.1.0`: `EST` at −18 000 s, `EDT` at
   −14 400 s (the default hour more), 02:00 by default.
2. 1 March 2024 was a Friday, so the first Sunday is the 3rd and
   `M3.2.0`, the second, is 10 March (Rd 738 955). From 1970-01-01 that is
   738 955 − 719 163 = 19 792 days, or 1 710 028 800 s. 02:00 in the
   standard offset: 1 710 028 800 + 7 200 + 18 000 = 1 710 054 000, which
   is 07:00 UTC. Likewise 1 November was a Friday, so `M11.1.0` is Sunday
   3 November (Rd 739 193), 20 030 days, 1 730 592 000 s; 02:00 in the
   daylight offset: 1 730 592 000 + 7 200 + 14 400 = 1 730 613 600, 06:00
   UTC. `transitions_in_year(2024)` returns exactly
   `(1710054000, 1730613600)`.
3. **Skipped.** The reading 2024-03-10 02:30 as −18 000 s is
   1 710 028 800 + 9 000 + 18 000 = 1 710 055 800, but from 1 710 054 000
   the zone keeps −14 400 s: rejected. As −14 400 s it is 1 710 052 200,
   before the change, where the zone keeps −18 000 s: rejected. No survivor:
   `Nonexistent`, with `gap_start` 02:00:00, `gap_end` 03:00:00,
   `transition` 1 710 054 000, `before_gap` 1 710 052 200 (01:30 EST) and
   `after_gap` 1 710 055 800 (03:30 EDT). `Earliest` gives 1 710 052 200;
   `Latest` and `PushForward` give 1 710 055 800; `Reject` fails. 01:59:59
   resolves to 1 710 053 999 and 03:00:00 to 1 710 054 000, both unique.
4. **Repeated.** The reading 2024-11-03 01:30: as −14 400 s,
   1 730 592 000 + 5 400 + 14 400 = 1 730 611 800, before the change at
   1 730 613 600, so kept; as −18 000 s, 1 730 615 400, after it, so kept.
   `Ambiguous` with `earlier` 1 730 611 800 (offset −14 400), `later`
   1 730 615 400 (offset −18 000), an hour apart. `Earliest` and
   `PushForward` give the first; `Latest` the second; `Reject` fails. At
   1 730 613 599 the wall clock reads 01:59:59 EDT and at 1 730 613 600 it
   reads 01:00:00 EST. 00:59:59 and 02:00:00 that day are unique.
5. **Next transitions.** From 2024-01-01 (1 704 067 200) the next is
   1 710 054 000, then 1 730 613 600, then 1 741 503 600 (2025-03-09
   07:00 UTC). At 1 710 054 000 `hc_zone_offset` writes
   `-14400 1 EDT 1730613600 -18000 builtin`.
6. **From a file.** The system file of tzdata 2026c, version 2, 236
   transitions, the last at 2 140 668 000 (2037-11-01 06:00 UTC), with the
   same footer, gives the same numbers in steps 3 and 4. A scratch file
   of the same shape whose records stop at 2024-11-03 06:00 UTC (built for
   this document, not in the repository) gives the 2024 changes from its
   records and then 1 741 503 600 from its footer, and 1 782 907 200
   (2026-07-01 12:00 UTC) is 08:00 EDT there. After `hc_zone_load` of the
   system file the same line ends `loaded`, not `builtin`.
7. **Another zone.** Cairo's start, `M4.5.5/0`, is the last Friday of April
   2026, the 24th, at 00:00 EET: 22:00 UTC on the 23rd, 1 776 981 600. The
   reading 2026-04-24 00:30 is `Nonexistent` with the gap 00:00 to 01:00,
   so midnight does not exist that day, and `hc_unix_from_fixed_in_zone`
   gives the first instant after the gap, 1 776 981 600. New York's
   midnight does exist: day 738 955 begins at 1 710 046 800.

A scratch program that calls the public API of `hc-tz` and
`zone_lines` printed every number above; the tests of `hc-tz` pin the
instants of steps 2 to 4 for 2024 (Code).

## What is carried

In `hc-tz`:

- `offset`: `UtcOffset` and `MAX_OFFSET_SECONDS`, parsing and writing the
  ISO 8601 spellings, and applying an offset to a civil date-time.
- `zone`: the object-safe `TimeZone` trait (`name`, `offset_at`,
  `abbreviation_at`, `is_dst_at`, `is_summer_time_at`, `next_transition`,
  `resolve_local`, `local_at`, `unix_at`), `LocalResolution`,
  `Disambiguation`, and `resolve_local_by_probing`.
- `fixed`: `FixedTimeZone` and `Utc`.
- `posix`: `PosixTz` and `PosixTimeZone`, the three rule forms, quoted
  abbreviations, the implied daylight offset, tzcode's United States rules
  when a daylight name has none, and the year range
  `FIRST_RULE_YEAR` to `LAST_RULE_YEAR` with `rules_answer_at`.
- `tzif`: `TzifData` and `TzifTimeZone` for versions 1 to 3, including the
  leap-second records and the indicator arrays as data.
- `system` (`std`): `read_zone` from `/usr/share/zoneinfo` or `$TZDIR`,
  which refuses a name that is empty, absolute, contains `..`, a
  backslash or a character outside the set the database uses.
- `builtin`: eighteen zones as POSIX strings, below.

**The built-in data.** `crates/hc-tz/data` holds four location files from
release 2026d, `zone1970.tab`, `zone.tab`, `backward` and `backzone`,
vendored unmodified [iana-tzdb-2026d]; the crate's rows are generated from
them by a test and checked against them (see
[zone-locations.md](zone-locations.md)). They carry no rules. The only rules
compiled in are the 18 strings of `builtin::ZONES`: `UTC` and 17 zones from
Africa/Cairo to Pacific/Auckland. Each is the footer of that zone's file in
release 2026d, which the crate says is unchanged from 2026c for these zones
(the `NEWS` entry of 2026d, dated 2026-09-11, changes the future of the
Northwest Territories and `America/Inuvik`, which is not among them
[iana-tzdb-news-2026d]). No script in the repository generates the strings,
and `system`'s tests re-read them against a local database when one exists.
The table is sorted, found by `hc_core::catalogue::matches` (ASCII case and
surrounding space ignored) and chosen for awkward offsets: India, Nepal,
Lord Howe Island, Sydney, Auckland, Cairo, and Denver for WWVB.

**The facade** (`hyper_calendar::zone_lines`, feature `tz`). `with_zone`
selects the rules for a name: first a TZif file the caller has loaded, then
the built-in table. `load_zone(name, bytes)` validates the bytes by parsing
them, copies them, and replaces any entry whose name matches; bytes that do
not parse are `Malformed` and nothing is kept. The loaded table is the one
piece of state that outlives a call (policy §13), a mutex-guarded list
parsed again on each call. Every export that reads a zone by name does so
through `with_zone`,
so the day, the offset and a radio frame's summer time cannot disagree.
`zone_offset` writes six tab-separated cells: the offset; the flag; the
abbreviation, empty where it is numeric; the next transition; the offset
after it; and `builtin` or `loaded`. `day_in_zone` is the local day of an
instant. `start_in_zone` is the instant a day begins: the midnight; for a
skipped midnight `after_gap`; for a repeated one the earlier. Instants and
days are held to the years −9 999 994 to 9 999 994, the seconds
−315 631 497 830 400 to 315 507 195 014 399, whichever rules answer; outside
them the exports are `OutOfRange`.

Not carried:

- **A rule database in the library** (not yet done): history for any zone
  but the 18 comes from a TZif file the caller supplies;
  `scripts/wasm-tzdata.sh` copies the eighteen's files from the host, whose
  release need not be 2026d.
- **TZif version 4** (not yet done). Its only change is the leap-second
  table [rfc9636], and `zic` has written it for a truncated leap-second
  table since release 2021b [iana-tzdb-news-2026d]; the reader refuses the
  byte.
- **Applying the leap-second records**, because the leap-second table is
  `hc_core::leap`'s (policy §2). The transition times of a `right/` file
  are in UNIX leap time, "UNIX time plus all preceding leap-second
  corrections" [rfc9636], and the reader takes them as POSIX time.
- **A previous transition** (not yet done): the trait has `next_transition`
  only.
- **A local zone** (policy §13: no clock; the caller supplies the zone), and
  **an abbreviation-to-zone lookup** (§4: `CST` has no single answer).
- **Rule history beyond what a TZif file carries**, such as why a rule
  changed: no source was read for it.

## Accuracy

Offsets, instants and flags are exact integers. The checks, run on
2026-10-03 against the system database of this machine, tzdata 2026c, 598
TZif files (586 version 2, 12 version 3, all with a footer, none version 4;
45 with no recorded transition), each file parsed without error:

| Check | Result |
| --- | --- |
| `cargo test -p hc-tz` | 123 unit tests and 1 doctest pass, among them five that read the system database |
| The closest two consecutive offset-changing transitions in the 598 files | 601 200 s apart (6.96 days), in `America/Boa_Vista`, `America/Noronha`, `America/Recife`, `Asia/Gaza` and others |
| Every minute from 3 days before to 3 days after each of the 40 closest pairs, 1 158 900 instants | each local reading resolves to its own instant, as the sole or one of the two; no `Nonexistent`; 0 failures |
| Type 0 of each file | a standard type in all 598, so the two readings of "before the first transition" agree |
| The built-in table against the file, hourly from 1900 to 2100 | agrees on offset, flag and abbreviation from the instant after the last disagreement below |

After the last record of a TZif file, and for every POSIX rule, the answer
is the rule's own, exact for the years −9 999 994 to 9 999 994 and as right
as the rule is: the database predicts future timestamps and "will be
incorrect after future governments change the rules" [iana-tz-theory].

The resolver assumes a zone's offset holds for about a day around a change
(it looks two days either side). The measured minimum above is 6.96 days, so
the assumption holds for this database; a file with two changes within a
day or two of one reading is not covered.

The built-in strings are right about the present and wrong about the past.
The last hour at which the table and the file disagree, sampled hourly:

| Zone | Last disagreement (UTC) |
| --- | --- |
| `UTC` | none |
| `Asia/Tokyo` | 1951-09-08T14Z |
| `Asia/Kolkata` | 1945-10-14T17Z |
| `Asia/Kathmandu` | 1985-12-31T18Z |
| `Asia/Seoul` | 1988-10-08T16Z |
| `Asia/Shanghai` | 1991-09-14T16Z |
| `Europe/Berlin`, `Europe/London`, `Europe/Paris` | 1995-10-29T00Z |
| `America/New_York`, `America/Denver`, `America/Los_Angeles` | 2006-11-05, at 05Z, 07Z and 08Z |
| `Australia/Sydney`, `Australia/Lord_Howe` | 2007-10-27T15Z |
| `Pacific/Auckland` | 2007-03-31T13Z |
| `America/Sao_Paulo` | 2019-02-17T01Z |
| `Africa/Cairo` | 2022-10-27T20Z |
| `Europe/Moscow` | 2014-10-25T21Z |

`Europe/Moscow` is later than the crate's README suggests ("abolished
daylight saving in 2011", `README.md` line 123 and `builtin.rs` line 15):
the file keeps `MSK` at UTC+4, flag 0, from
2011-03-27 to 2014-10-26, so `MSK-3` is wrong for those years. A sample
hourly could miss a mismatch shorter than an hour. The 2007 United States
change is held by the test
`the_2007_rule_change_moved_the_spring_transition_from_april_to_march`
and by the system test of the 2006 and 2007 instants.

Where the code and the sources differ:

- **Before the first transition.** `first_standard_type_index`
  (`tzif.rs`, lines 503-506, used at 612-613) reads RFC 8536 §3.2 as "the
  first type that is not a daylight type", and its test is named
  `instants_before_the_first_transition_use_the_first_standard_type`.
  RFC 9636 §3.2 says type 0 [rfc9636]. A hand-built version 2 file whose
  type 0 is `EDT` (−14 400, flag 1) and whose type 1 is `EST`, with one
  transition, reads −18 000 s and `EST` at the epoch; RFC 9636 gives
  −14 400 s. No real file differs (above).
- **Version 4.** `TzifVersion::from_byte(b'4')` is `UnsupportedTzifVersion`
  (`tzif.rs`, line 992). RFC 9636 defines it; the README (line 79) and the
  module documents cite RFC 8536, which RFC 9636 obsoletes [rfc9636].
- **Offsets of 25 hours.** `AAA25BBB,M3.2.0,M11.1.0` parses, though POSIX
  limits the hour of an offset to 24 [posix-2017-tz]; `AAA26BBB` is
  refused. The window is the one `offset.rs` documents. A rule hour of 168
  is refused as `MalformedRule`; 167 is read.
- **Exports.** The documentation of `hc_fixed_from_unix_in_zone` says the
  module carries "seventeen" zones (`exports.rs`, lines 4201 and 4213);
  `builtin::ZONES` has 18 and the test
  `the_builtin_zone_count_is_the_one_the_readme_states` pins it.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [rfc9636] | TZif: versions 1 to 4, §3.1 header, §3.2 data block (type 0 before the first transition, `utoff` window), §3.3 footer ("on or after" the last transition), §3.3.1 all-year summer time, §3.3.2 the extended hours, §4 skipping the version 1 block, UNIX leap time | Yes, 2026-10-03; the HTML at rfc-editor.org, in several fetches, each returned by the fetch tool as a summary with short quotations |
| [posix-2017-tz] | Issue 7: hours 0 to 24 and no sign in an offset and in a rule time, the 02:00:00 default, week 5 as the last such weekday | Yes, 2026-10-03 |
| [posix-2024-tz] | Issue 8: the format, the name rules, `Jn`, `n` and `Mm.n.d`, the time field to 167 hours, the one-hour daylight default, the missing rule left to the implementation, the sign | Yes, 2026-10-03 |
| [iana-tz-theory] | Zone names, the abbreviation rules, the warning about pre-1970 data and future predictions | Yes, 2026-10-03 |
| [iana-tz-link] | The scope of the database, TZif as RFC 9636, the release naming | Yes, 2026-10-03 |
| [iana-tzdb-2026d] | The four location files, vendored in `crates/hc-tz/data` (their headers re-read 2026-10-03), and the release date, 2026-09-11, from `NEWS` | Yes |
| [iana-tzdb-news-2026d] | The 2026d and 2026c entries; version 4 first written by 2021b, for the leap-second table | Yes, 2026-10-03 |
| [tz-europe-eire-rules] | The `Eire` rules and `Europe/Dublin`, the negative winter saving | Yes, 2026-10-03, at the `2026d` tag |
| RFC 8536, the earlier TZif RFC the crate cites | Superseded by RFC 9636, which was read for every §3 rule above | Not read |
| The United States Energy Policy Act of 2005, EU Directive 2000/84/EC, the Australian and New Zealand state rules | The transition dates the README says the tests come from | Not read; the crate's tests agree with the system database instead |
| Reingold and Dershowitz, *Calendrical Calculations*, 4th ed. [reingold2018] | The Rata Die arithmetic `hc_calendar::gregorian` implements | Not read for this document |

The fetch tool returns a summary of a page and not the page, so the
quotations above are its quotations of the page, not a transcript. What the
code implements is held by the tests and the measurements in Accuracy, and
not by the quotations.

## Code

`crates/hc-tz/src/zone.rs`, `posix.rs`, `tzif.rs`, `offset.rs`, `fixed.rs`,
`system.rs` and `builtin.rs`, and `crates/hyper-calendar/src/zone_lines.rs`.
The tests that anchor them, in `hc-tz`:
`the_skipped_hour_resolves_to_no_instant_and_names_the_gap`,
`the_repeated_hour_resolves_to_two_instants`,
`disambiguation_policies_pick_the_documented_instant`,
`pushing_a_skipped_reading_forward_lands_on_the_same_wall_clock_offset`,
`summer_time_is_the_higher_offset_where_the_saving_is_negative`
(`zone.rs`); `the_eastern_united_states_switches_on_the_published_dates`,
`the_skipped_hour_at_the_start_of_american_saving_time_does_not_exist`,
`the_repeated_hour_at_the_end_of_american_saving_time_is_ambiguous`,
`egypt_ends_its_saving_period_at_the_last_instant_of_a_thursday`,
`southern_hemisphere_saving_time_straddles_the_new_year`,
`week_five_means_the_last_one_whether_or_not_there_are_five`,
`julian_rules_never_count_the_twenty_ninth_of_february`,
`rule_times_may_be_negative_or_past_midnight`,
`rules_that_never_change_have_no_next_transition`,
`every_hour_of_a_year_in_new_york_resolves_back_to_its_own_instant` and
`the_rules_answer_at_both_ends_of_their_range_as_in_their_cycle`
(`posix.rs`); `the_next_transition_walks_the_record_and_then_the_footer`,
`instants_after_the_last_transition_follow_the_footer`,
`version_three_footers_may_use_hour_twenty_four_and_negative_times`,
`transitions_must_be_strictly_increasing` and
`the_thirty_two_and_sixty_four_bit_blocks_agree` (`tzif.rs`);
`the_system_database_agrees_with_the_builtin_table_on_current_rules` and
`the_system_database_and_the_builtin_table_change_at_the_same_instants`
(`system.rs`, skipped where no database exists); and
`every_builtin_string_parses_and_renders_back_to_itself` (`builtin.rs`).
The facade's lines are in `zone_lines.rs`, and its exports are declared in
`crates/hyper-calendar/src/exports.rs`.
