# ISO 8601 text beyond a date: date-times, durations, intervals and patterns

Backs `hc-format`'s `iso8601`, `rfc3339`, `rfc2822`, `python` and `patterns`
modules at the boundary: the `datetime` and `patterns` layers of the
WebAssembly module and the C library, written once in
`crates/hyper-calendar/src/datetime_lines.rs`.

## What it is

ISO 8601 is a family of representations, not one format: calendar, ordinal
and week dates, in basic and extended form, at any accuracy, with a time of
day, a zone, a duration or an interval beside them [wikipedia-iso-8601]. RFC
3339 is the internet's profile of it [rfc3339], RFC 5322's date and HTTP's
IMF-fixdate [rfc9110] are another notation for the same instants, and Python
keeps a profile of its own for `datetime.isoformat` [python-datetime-docs].
The two pattern languages programs read dates with, `strptime`'s and CLDR's
[uts35-v48], are a third family.

## How it works

A text is read into a *reading*: a local fixed day, the second of that day,
the attoseconds into it and what the text said about its zone. A reading is
not an instant. `2026-09-21T14:30:05` is a wall-clock time that is 14 hours
apart in Auckland and Honolulu, so the line that reads it carries no offset
and no POSIX second, and the library never takes it for UTC. A text with a
zone, `2026-09-21T14:30:05+09:00`, has an offset (32 400 s) and an instant,
1 789 968 605 s since the POSIX epoch.

Worked example, `1937-01-01T12:00:27.87+00:20` (RFC 3339 §5.8): the local day
is RD 707 110, the second 43 227 and the fraction 870 000 000 000 000 000
attoseconds; the offset is 1 200 s; the instant is 12:00:27.87 − 00:20 =
11:40:27.87 UTC on that day, which is −1 041 337 172.13 s, and its POSIX second
is the floor, −1 041 337 173, the fraction staying in the attosecond cell.
Python's `int(timestamp())` truncates toward zero and says −1 041 337 172.

A reduced date, `2026-W39`, names a week and no day, so it has no fixed day
and no reading; `hc_iso_date_parts` says what it is. A duration with years or
months, `P3Y6M4DT12H30M5S`, has no fixed length and is *nominal*; one with
days, `PT36H` or `P1W`, has an exact length on the nominal timeline, a day of
86 400 seconds. An interval is two readings, a reading and a duration, a
duration and a reading, or a duration, and repeats with `R5/` or `R/`.

## What is carried

- `hc_parse_datetime` in six syntaxes (`iso8601`, `iso8601-full`, `rfc3339`,
  `rfc2822`, `python`, `auto`); `hc_format_datetime` in eight (`iso8601`, its
  basic, ordinal and week forms, `rfc3339`, `rfc2822`, `imf-fixdate`,
  `python`) to a precision from hours to nanoseconds, truncating.
- `hc_format_iso_date_as` and `hc_iso_date_parts`: the week-numbering year,
  expanded years, reduced accuracy.
- `hc_iso_duration`, `hc_format_iso_duration`, `hc_iso_interval`.
- `hc_parse_pattern` and `hc_parse_pattern_in`: POSIX `strptime`, CPython's
  `strptime` and CLDR patterns read into the fields they name and the reading
  they resolve to, in the C locale's names or a locale's.
- *Not carried at the boundary*: a time zone (the offset is an argument, and
  the `tz` layer has the zones), `hc-format`'s Roman day names and calendar
  labels (the `calendars` layer reads a written date by `hc_parse_date`), and a
  value with several representations kept apart; see `hc-format`'s README for
  what the library itself holds.

## Accuracy

| Check | Source | Test | Result |
| --- | --- | --- | --- |
| RFC 3339 §5.8's four examples read to their instants | [rfc3339], Python's `datetime` | `rfc_3339_readings_are_instants` | agrees; the 1937 instant floors where Python truncates |
| IMF-fixdate `Sun, 06 Nov 1994 08:49:37 GMT` is 784 111 777 | [rfc9110], Python's `email.utils` | `an_email_date_is_read_as_one` | agrees |
| `2026-09-21` is `2026-W39-1` and `2026-264`; 2021-01-03 is `2020-W53-7` | Python's `isocalendar` and `%j` | `a_date_is_written_as_a_week_or_an_ordinal_date` | agrees |
| `P3Y6M4DT12H30M5S` is nominal; `PT36H` is 129 600 s; `P1W` 604 800 s | [wikipedia-iso-8601], Python's `timedelta` | `a_duration_is_its_components` | agrees |
| The interval and the repeating interval of the examples | [wikipedia-iso-8601], Python's `calendar.timegm` | `an_interval_is_its_ends` | agrees |
| `isoformat` of an instant with microseconds and a zero offset | Python's `datetime.isoformat` | `an_instant_is_written_in_each_syntax` | agrees |
| `strptime` of `%H:%M` is on 1900-01-01 | Python's `datetime.strptime` | `a_text_is_read_against_a_pattern` | agrees |

## Sources

| Key | What it gives | Read |
| --- | --- | --- |
| [wikipedia-iso-8601] | What ISO 8601 allows; the duration, interval and repeating-interval examples; the ISO texts are sold | Yes, 2026-09-26, and 2026-10-03 through a page summary for the examples |
| [rfc3339] | The internet profile; §5.8's examples | Yes, 2026-10-03, §5.8 through a page summary |
| [rfc9110] | IMF-fixdate, §5.6.7 | Yes, 2026-10-03, through a page summary |
| [python-datetime-docs] | `isoformat`, `fromisoformat`, `strptime` | Yes, 2026-09-26 |
| [uts35-v48] | CLDR date patterns | Yes, 2026-09-29 |

RFC 5322's appendix, where an example date of the RFC 822 family appears, was
not read; the instant of `Fri, 21 Nov 1997 09:55:06 -0600`, 880 127 706, is
Python's.

## Code

- `crates/hyper-calendar/src/datetime_lines.rs`: the lines; tests beside it
  and in `crates/hyper-calendar-wasm/src/tests/datetime.rs`, `patterns.rs`
  and the C library's twins.
- `crates/hc-format/src/iso8601.rs`, `rfc3339.rs`, `rfc2822.rs`, `python.rs`
  and `patterns/`: the grammars.
