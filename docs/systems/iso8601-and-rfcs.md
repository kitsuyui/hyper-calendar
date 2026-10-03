# ISO 8601, RFC 3339 and RFC 2822 / RFC 5322 date-time text

Backs `hc-format`'s `iso8601` module with its `iso8601::duration` and
`iso8601::interval` submodules, its `rfc3339` and `rfc2822` modules,
`parse::detect`, and the types they share (`Strictness`, `IsoDate`,
`IsoTime`, `IsoDateTime`, `IsoDuration`, `Interval`, `RepeatingInterval`,
`OffsetDateTime`, `ZoneInfo`). The WebAssembly and C export
`hc_parse_iso_date` rests on `iso8601::parse_date`; `hc_format_iso_date` is
written by the facade's `Date` display, not by `hc-format`. No calendar
identifier is registered here. The calendars `iso8601`,
`iso8601-week` and `iso8601-ordinal` are `hc-calendars-solar`'s and are
listed in [../calendars.md](../calendars.md). Python's `fromisoformat`
profile is `hc_format::python`, explained in
[python-compatibility.md](python-compatibility.md). The module `rfc2822`
implements RFC 5322, which obsoletes RFC 2822; the name has stuck.

## What it is

ISO 8601 is the international standard for writing dates, times, durations
and intervals as text. Its first edition is ISO 8601:1988, followed by
editions in 2000 and 2004 and, in 2019, by two parts: ISO 8601-1:2019,
*Basic rules*, and ISO 8601-2:2019, *Extensions*. ISO 8601-1:2019/Amd 1:2022
made technical corrections, and the most significant was to bring back
`24:00:00` for the end of a calendar day [wikipedia-iso-8601]. It is a
family of notations, not one format. It has calendar, ordinal and week
dates, a basic format without separators and an extended one with them,
and any number of values dropped from the low end for reduced accuracy.

Three texts narrow it for use in protocols:

- **RFC 3339** (July 2002), an Internet profile of ISO 8601 "for
  representation of dates and times using the Gregorian calendar", which
  calls itself "a conformant subset of the ISO 8601 extended format" and
  achieves simplicity "by making most fields and punctuation mandatory"
  [rfc3339].
- **The W3C NOTE "Date and Time Formats"** (submitted 15 September 1997), a
  profile of six formats from `YYYY` to `YYYY-MM-DDThh:mm:ss.sTZD`
  [w3c-note-datetime].
- **RFC 5322**'s date (October 2008), which obsoletes RFC 2822 (April 2001),
  the date of an email header. It is not a profile of ISO 8601: it has a
  weekday and a month name and its own zone rules [rfc5322] [rfc2822].
  HTTP's `IMF-fixdate` is "a fixed-length and single-zone subset of the date
  and time specification used by the Internet Message Format"
  [rfc9110].

RFC 3339 has itself been updated: RFC 9557 (April 2024) extends the
timestamp with bracketed suffixes and rewrites the meaning of `Z`
[rfc9557]. The competing readings are in the table in "How it works".

## How it works

**The editions.** The 2000 edition allowed truncated representations such
as `--09-21` and `85-10-26`; ISO 8601:2004 removed them. The 2004 edition
allowed the `T` to be omitted by agreement and ISO 8601-1:2019 removed that
[wikipedia-iso-8601]. ISO 8601-1:2019 removed `24:00:00` and Amd 1:2022
brought it back [wikipedia-iso-8601]. A rule below is therefore the rule
of an edition, and where the edition is not known it is said so.

**Dates.** The calendar is the Gregorian. A year is four digits, `0000` to
`9999`, where 0000 is 1 BC; values `0000` through `1582` "shall only be
used by mutual agreement", and a year outside the four digits has a sign and an
agreed number of digits, `+002026`, with `-0001` for 2 BC
[wikipedia-iso-8601]. A calendar date is `YYYY-MM-DD` or `YYYYMMDD`; the
month alone is `YYYY-MM` and never `YYYYMM`, which would collide with the
truncated `YYMMDD`. An ordinal date is `YYYY-DDD` or `YYYYDDD`, with `DDD`
from 001 to 365 or 366. A week date is `YYYY-Www-D` or `YYYYWwwD`, or
`YYYY-Www` for the week, with the weekday from 1 (Monday) to 7 (Sunday)
[wikipedia-iso-8601]. Week 01 is the week with the year's first Thursday,
equivalently the week with 4 January in it. Weeks start on Monday, and a
year has 52 or 53 of them: 53 for a year that starts on a Thursday and for a
leap year that starts on a Wednesday. 28 December is always in the last
[wikipedia-iso-week-date]. Days at a year's ends can belong to the
neighbouring week-numbering year.

**Times and zones.** The clock has `hh` from 00 to 24, `mm` to 59 and `ss`
from 00 to 60, the 60 only for an added leap second. The seconds, or the
minutes and seconds, may be dropped. A decimal fraction, with a comma or a
dot, goes on the lowest-order element present, with no limit on digits;
`14:30,5` is 14 hours and 30.5 minutes. A time with no designator is local
time and says nothing of the zone. `Z` is the zero offset, and an offset is
`±hh:mm`, `±hhmm` or `±hh`; a zero offset is written with a plus sign and
`-00:00` is not permitted [wikipedia-iso-8601]. A date and time join with
`T`; "both date and time must use the same format"; a space in place of the
`T` is not ISO 8601 but is allowed by RFC 3339 [wikipedia-iso-8601].

**Durations.** `P3Y6M4DT12H30M5S` is three years, six months, four days,
twelve hours, thirty minutes and five seconds. A zero element may be
dropped, but at least one must stay, so `P` is not a duration and `PT0S` is.
`P1M` is a month and `PT1M` a minute. The smallest element may have a
fraction, `P0.5Y` or `P0,5Y`. Values may pass their carry points, so
`PT36H` is allowed. A week is `PnW`, shown as a form of its own. The
alternative form is `P0003-06-04T12:30:05` by agreement, and there "a value
of 13 for the month or 25 for the hour would not be permissible"
[wikipedia-iso-8601]. A duration in months has no length until it is
attached to a date: `2003-02-15T00:00:00Z/P2M` ends 59 days later and
`2003-07-15T00:00:00Z/P2M` 62 days later [wikipedia-iso-8601]. RFC 3339's
Appendix A, informational and based on the 1988 edition, gives a grammar
that has the week form alone, `duration = "P" (dur-date / dur-time /
dur-week)`, and no fractions [rfc3339].

**Intervals.** Four forms: start and end, start and duration, duration and
end, and a duration alone, joined by a solidus, which "may be replaced by a
double hyphen" by agreement. An end may leave out elements and take them
from the start: `2007-12-14T13:30/15:30`, `2008-02-15/03-14`,
`2007-11-13/15`. A repeating interval is `Rn/` before an interval, `R/` or
`R-1/` for no limit, `R0/` for no repetition [wikipedia-iso-8601].

**RFC 3339.** The grammar is `date-time = full-date "T" full-time`, with
`full-date = date-fullyear "-" date-month "-" date-mday`, an hour 00-23, a
minute 00-59, a second 00-58, 00-59 or 00-60, and a fraction `"."
1*DIGIT`. The offset is `"Z"` or a sign, an hour 00-23, a colon and a
minute [rfc3339]. The `T` and `Z` may be lower case, and a full date and a
full time may be separated by, say, a space (the notes of §5.6). The hour
24 is left out "in order to reduce confusion" (§5.7). The second 60 may
occur "at the end of months in which a leap second occurs", and "in time
zones other than 'Z', the leap second point is shifted by the zone offset
(so it happens at the same instant around the globe)"; the examples are
`1990-12-31T23:59:60Z` and the same second `1990-12-31T15:59:60-08:00`
(§5.7, §5.8). An offset is "local time minus UTC", so `18:50:00-04:00` is
`22:50:00Z` (§4.2). Section 4.3 gives the offset `-00:00` the meaning "the
time in UTC is known, but the offset to local time is unknown", which
"differs semantically from an offset of 'Z' or '+00:00', which imply that
UTC is the preferred reference point" [rfc3339]. RFC 9557 §2.2 revises
that section: the unknown offset is now `Z`; `-00:00`, "not allowed by
[ISO8601:2000] and therefore is less interoperable", is not formally
deprecated; and `+00:00` keeps "UTC is the preferred reference point"
[rfc9557].

**RFC 5322.** A date-time is `[ day-of-week "," ] date time [CFWS]`, the
day one or two digits, the month `Jan` to `Dec`, the year "any numeric year
1900 or later" of four or more digits, the time `hh:mm` with optional
`:ss`, and a zone of a sign and four digits [rfc5322]. The text says
"a date-time specification MUST be semantically valid": the weekday, if
present, must be the day the date implies, the time-of-day in the range
"00:00:00 through 23:59:60" and the last two digits of the zone 00 to 59
[rfc5322]. RFC 2822 instead gave the zone as "within the range -9959
through +9959" [rfc2822]. `+0000` is Universal Time, and `-0000` "also
indicates Universal Time" but marks a time "generated on a system that may
be in a local time zone other than Universal Time"; the same words are in
RFC 2822 §3.3 [rfc5322] [rfc2822]. Section 4.3 tells a parser to read the
obsolete forms: a two-digit year 00-49 is 2000-2049, 50-99 and any
three-digit year add 1900; `UT` and `GMT` are `+0000` and the US zones
`EST` -0500, `EDT` -0400, `CST` -0600, `CDT` -0500, `MST` -0700, `MDT`
-0600, `PST` -0800, `PDT` -0700; the military zones, of which RFC 822 gave
a wrong sign, and any other alphabetic zone "SHOULD all be considered
equivalent to -0000" unless more is known; comments and folding whitespace
may stand between tokens [rfc5322]. The HTTP date is
`day-name "," SP day SP month SP year SP hh:mm:ss SP "GMT"`, "00:00:00 -
23:59:60 (leap second)"; a recipient MUST also accept two obsolete formats,
the RFC 850 date and `asctime`'s [rfc9110].

**Where the readings compete.** One text differs from another in these
places. The library's reading is in the last column.

| Point | The readings | This library |
| --- | --- | --- |
| `-00:00` and `Z` | RFC 3339 (2002) §4.3: `-00:00` is an unknown local offset, `Z` and `+00:00` UTC as preferred reference. RFC 9557 (2024) §2.2: `Z` is the unknown offset, `-00:00` is still legal but discouraged. ISO 8601: `-00:00` is not permitted | the 2002 reading: `ZoneInfo::Zulu`, `ZoneInfo::Offset` of zero and `ZoneInfo::UnknownLocalOffset` are three values; ISO strictness refuses `-00:00`, RFC 3339 strictness reads it |
| The hour 24 | ISO 8601:2004 yes, ISO 8601-1:2019 no, Amd 1:2022 yes; RFC 3339 and the W3C profile no | `24:00` accepted under `Strictness::ISO`, kept apart from `00:00`; refused by `FULL` and `RFC_3339` |
| Basic format | ISO 8601 yes; RFC 3339 no; W3C no | `ISO` yes; `FULL` and `RFC_3339` no |
| Reduced accuracy | ISO 8601 yes; W3C `YYYY`, `YYYY-MM` and a time to the minute; RFC 3339 none | `ISO` yes; `FULL` and `RFC_3339` no |
| Week and ordinal dates | ISO 8601 yes; RFC 3339 "not permitted" [wikipedia-iso-8601] | accepted under all three presets (see Accuracy) |
| Date and time separator | ISO 8601 `T`; RFC 3339 `T`, `t` or, by its note, a space | `ISO` `T` only; `RFC_3339` `T`, `t` or a space |
| Date and time in one format | ISO 8601 the same format for both; RFC 3339 Appendix A "permits mixtures" | mixtures accepted under all three presets (see Accuracy) |
| A leap second away from UTC | RFC 3339: shifted by the offset | a second 60 only at `23:59`, in any zone (see Accuracy) |
| A fraction | ISO 8601 no limit, comma or dot; RFC 3339 dot and `1*DIGIT`; W3C leaves the digits to the adopting standard | at most 18 digits, a comma only where the preset allows |
| The year before 1900 | RFC 5322 "1900 or later"; ISO 8601 `0000`-`1582` by agreement | read with no flag for the agreement: RFC 5322 from 0001, ISO from 0000 |
| Zone words of an email | RFC 5322: `UT`, `GMT`, the US zones; others `-0000`; `UTC` unlisted | `UTC` read as `+0000`, the rest as the RFC |

**Worked example.** Take Monday 21 September 2026 at 14:30:05 in a zone
nine hours ahead of UTC.

- The ordinal date: January to August have 31, 28, 31, 30, 31, 30, 31, 31 =
  243 days, and 243 + 21 = 264. So `2026-264`.
- The week date: 1 January 2026 is a Thursday, so week 01 is the week that
  holds it, and starts on Monday 29 December 2025, day -2 of 2026. Day 264
  is 266 days later, and 266 = 38 × 7, so the Monday of week 1 + 38 = 39:
  `2026-W39-1`. Week 53 of 2026 exists, because the year begins on a
  Thursday: `2026-W53-1` is 28 December. `2021-W53-1` is refused, 2021
  having 52 weeks.
- All six spellings `2026-09-21`, `20260921`, `2026-264`, `2026264`,
  `2026-W39-1` and `2026W391` read as day 739,880 of the library's count,
  where day 1 is 0001-01-01.
- The instant: `2026-09-21T14:30:05+09:00` is 05:30:05 UTC. Day 739,880 is
  20,717 days after the Unix epoch, which is day 719,163, and
  20,717 × 86,400 = 1,789,948,800. Add 5 × 3,600 + 30 × 60 + 5 = 19,805:
  1,789,968,605.
- Fractions: `T14.5` is 14:30:00 and `T14:30,5` is 14:30:30.
- The unknown offset: `2026-09-21T14:30:05-00:00` is refused by
  `iso8601::parse` and read by `rfc3339::parse` as `UnknownLocalOffset`. It
  names the same instant as `...Z` and `...+00:00`, which is 14:30:05, or
  52,205 s after 1,789,948,800: 1,790,001,005. The three round-trip as
  themselves.
- The leap second: `1990-12-31T23:59:60Z`, from RFC 3339 §5.8, parses and
  its POSIX timestamp is 662,688,000, the same as `1991-01-01T00:00:00Z`,
  because POSIX time has no second 60.
- Durations: `P3Y6M4DT12H30M5S` has years and months and refuses to become a
  span; `PT1.5H` is 5,400 s, `P1W` 604,800 s and `PT0.5S` is
  500,000,000,000,000,000 attoseconds. `P0003-06-04T12:30:05` is the same
  duration as the first, in the alternative form.
- An interval: `2026-09-21T14:30:05Z/PT1.5H` is a start and a duration of
  5,400 s; `R5/PT1H/2026-01-01T00:00:00Z` is a repetition of five, a
  duration and an end. The types keep the parts and compute no dates from
  them.
- An email date: `Mon, 21 Sep 2026 14:30:05 +0900` gives the same
  1,789,968,605. `Fri, 21 Sep 2026 14:30:05 +0900` gives it too, and writes
  back `Mon`, the weekday computed from the date.

## What is carried

The module `iso8601` parses and writes these parts of the notation, and
keeps the written shape, so that a date, a time and a zone come back as
they were written.

- **Dates.** Calendar, ordinal and week dates, in basic and extended format
  (`DateParts::{Calendar, Ordinal, Week}`, `Style`); a year of four digits
  or an expanded year with a sign and at least four digits, up to nine
  (`YearStyle`), in the proleptic Gregorian calendar from year -9,999,999 to
  9,999,999 (`hc_calendars_solar::gregorian`, the range of the Gregorian
  calendar). Reduced accuracy: the year alone, `YYYY-MM`, and a week
  `YYYY-Www` without its day. The week and the day of the year are checked
  against the year, the day against its month.
- **Times.** The hour, minute and second, in either format; the lowest
  element may have a fraction with a point or a comma, kept, to 18 digits,
  as integer attoseconds; `24:00`, only with nothing after it, kept apart
  from `00:00` of the next day; `23:59:60`. `IsoTime::since_midnight` and
  `to_time_of_day` resolve a fractional hour or minute exactly.
- **Zones.** `Z`, `±hh`, `±hh:mm`, `±hhmm`, and an offset with seconds,
  `±hh:mm:ss` or `±hhmmss`, for an offset up to 25:59:59
  (`hc_tz::UtcOffset`);
  no designator at all, kept as `ZoneInfo::Unspecified`, which is not UTC
  and gives no instant until a zone is supplied.
- **Durations.** `PnYnMnDTnHnMnS` with any ordered subset of the elements,
  `PnW`, a fraction on the lowest element with a point or a comma, the
  alternative form in both formats, and a sign. `IsoDuration::
  to_exact_duration` returns the span of a duration without years and
  months, with a week as 7 days and a day as 86,400 s, and refuses one with
  years or months with `ValueError::NominalDuration`.
- **Intervals.** The four forms and `Rn/` and `R/`, each part a complete
  or reduced date-time or a duration.
- **Presets.** `Strictness::ISO`, `FULL` and `RFC_3339`, each a setting of
  fourteen independent permissions. `ISO` accepts all that is listed
  above. `FULL` refuses the basic format, reduced accuracy, the comma,
  `24:00` and a signed duration, and demands a time and a zone with
  minutes.
  `RFC_3339` is `FULL` that also refuses expanded years and accepts the
  space, lower case and `-00:00`.
- **RFC 3339.** `rfc3339::parse` and `parse_iso`, and `write`, `write_unix`
  and `write_civil` with `SubsecondPrecision` (automatic, whole seconds, or
  `Digits(1..=18)` truncating). The year must be 0000 to 9999 and a zone
  must be present.
- **RFC 5322.** `rfc2822::parse` with the obsolete syntax of §4.3,
  `write` in the form §3.3 tells a generator to write, `write_imf_fixdate`
  for HTTP, and `named_zone_offset`. A parsed `GMT` or `UT` is
  `ZoneInfo::Zulu`; `-0000` and every unlisted alphabetic zone
  `UnknownLocalOffset`.
- **Sniffing.** `parse::detect` chooses a grammar by five rules (`R` and a
  digit or `/`, a leading `P`, a `/`, a letter other than `T`, `W` or `Z`,
  otherwise ISO 8601) and says which it chose.

Not carried, with the reason each time:

- The text of ISO 8601-1:2019, its Amendment 1:2022 and ISO 8601-2:2019: no
  source read. The standard is sold by ISO, and ISO's own overview page
  answered a bot challenge. Every rule above is from the secondary account
  [wikipedia-iso-8601], from the RFCs, and from the W3C note. A rule that
  only the standard fixes is not checked.
- Truncated representations (`--09-21`): ISO 8601:2004 removed them
  [wikipedia-iso-8601].
- A century or a decade as a reduced year, `19` and `198`, which
  [wikipedia-iso-8601] gives as allowed: not yet done; the parser asks for
  four digits. The standard's own text, which would say whether the 2019
  edition keeps them, was not read.
- An expanded year in the basic format: the sources leave its digit count
  to agreement [wikipedia-iso-8601], so `+0020260921` is a six-digit year and
  a day or an eight-digit year and an ordinal. The parser refuses a run of
  ten digits or more and reads a shorter run as the whole year, so
  `+20260921` is a year out of range.
- An interval end that leaves out elements (`2007-12-14T13:30/15:30`,
  `2007-11-13/15`): not yet done. [wikipedia-iso-8601] gives examples for a
  start of full accuracy only; what the standard says of a reduced start
  was not read.
- `--` as the interval separator, and `R-1/` for an unbounded repetition
  [wikipedia-iso-8601]: not yet done. `R/` is carried.
- Open or unknown interval ends (`../2025`, `2024/`) and every other
  ISO 8601-2 extension: not carried by `hc-format`. `hc-uncertainty`'s
  `edtf` module parses the Extended Date/Time Format.
- Adding a duration to a date, or counting the occurrences of a repeating
  interval: not yet done in `hc-format`. [wikipedia-iso-8601] gives only the
  two examples of `P2M` above, and no rule for a month that is short.
- More than 18 fractional digits: the library's resolution is an
  attosecond; RFC 3339's `1*DIGIT` and ISO 8601's "no limit" admit more.
- A leap second away from the end of a UTC day, in a zone with an offset:
  not yet done. `hc_calendar::CivilTime` holds a second 60 only at 23:59.
- Offsets with hours above 25: RFC 2822 gives the range as -9959 to +9959
  [rfc2822], and RFC 5322 constrains only the last two digits [rfc5322].
  Not yet done; `hc_tz::UtcOffset` stops at 25:59:59.
- The reading of `Z` of RFC 9557 [rfc9557], and its bracketed suffixes, a
  zone name `[Europe/Paris]` and a key `[u-ca=hebrew]`: not yet done.
- The two obsolete HTTP date formats, the RFC 850 date
  `Sunday, 06-Nov-94 08:49:37 GMT` and `asctime`'s
  `Sun Nov  6 08:49:37 1994`, which a recipient MUST accept [rfc9110]: not
  yet done. `rfc2822::parse` refuses both.
- The W3C profile as a preset, `Strictness` having no flag to refuse week
  and ordinal dates or a time of the hour alone: not yet done.
- A check that an RFC 5322 weekday is a day name, or is the day the date
  implies: not yet done.
- Calendars other than the Gregorian: ISO 8601 uses it, and the other
  calendars are `hc-calendars-*`'s.

The library refuses these, and says what and where:

- `YYYYMM` (the standard forbids it [wikipedia-iso-8601]); `-00:00` under
  the ISO presets; mixing separators inside the date, the time or the week
  date; `24:00` with a non-zero part; an hour above 24; a month 13, a 30th of
  February, a day 366 in a common year, a week 53 in a 52-week year, a
  weekday 8; a leap second outside `23:59`; a second above 60; an offset
  with minutes or seconds above 59 or beyond 25:59:59.
- Duration components out of order, a second `T`, a fraction on a component
  that is not the last, a number of more than 18 digits, `P` alone, an
  interval of two durations or of more than two parts, a repetition count of
  more than 18 digits or a sign.
- Under `RFC_3339`: the basic format, the date without a time, a time
  without a zone, the minutes of the time dropped, the hour-only offset, a
  comma, the hour 24 and an expanded year.
- In `rfc2822::parse`: an unknown month name, a day beyond the month, an
  hour above 23, a minute above 59, a second 60 away from 23:59, a year of
  fewer than two or more than nine digits, an offset minute above 59, an
  unclosed comment and text after the zone.

## Accuracy

Everything is integer arithmetic: sub-second values are attoseconds and
offsets whole seconds, so there is no rounding in a parse or a write. The
instants are exact over the Gregorian range, years -9,999,999 to 9,999,999;
the four-digit forms of RFC 3339 and RFC 5322 are limited to 0000 to 9999.

**Measured.**

- Every day from 0001-01-01 to 9999-12-31, 3,652,059 days, was written as a
  calendar date, an ordinal date and a week date by Python 3.9.6's
  `datetime.date` (`toordinal`, `isoformat`, `isocalendar`), and the
  10,956,177 strings were read by `iso8601::parse_date`. Each gave the day
  Python's ordinal gave: no disagreement. Python is a cross-check on the
  week algorithm, not a source of the rule.
- `tests/round_trip.rs` writes and reads back every day from 1583-01-01 to
  2400-12-31, 298,769 days, as a calendar, an ordinal and a week date in
  both formats, every second of a day, every whole-minute offset in every
  offset style, 30,000 generated date-times and the duration, interval and
  RFC 3339 and RFC 5322 shapes, from a fixed-seed generator. Its 29 tests
  and the 95 unit tests picked out by the module names `iso8601`, `rfc3339`
  and `rfc2822` pass in a debug build.
- Published values. Wikipedia's `2009-W53-7` and `2010-01-03`, `2009-W01-1`
  and `2008-12-29`, and `1981-095` and `1981-04-05` read as one day each.
  Four of RFC 3339 §5.8's five examples parse; the fifth is a departure below. The POSIX seconds of
  `1990-12-31T23:59:60Z` (662,688,000), `1996-12-19T16:39:57-08:00`
  (851,042,397), RFC 9110's `Sun, 06 Nov 1994 08:49:37 GMT` (784,111,777)
  and the example above (1,789,968,605 and 1,790,001,005) are the UTC
  timestamps Python gives for the same instants.

**Departures found.** Each was found by running the code against the text
read, and none has been fixed. No test asserts the behaviour the text asks
for in any of them; `a_leap_second_anywhere_but_midnight_is_refused` asserts
the present behaviour for a reading in UTC, where it is right.

- **RFC 3339 reads week and ordinal dates.** `rfc3339::parse` of
  `2026-W39-1T14:30:05Z` and of `2026-264T14:30:05Z` returns
  `2026-09-21T14:30:05Z`. RFC 3339's `full-date` is the calendar date
  [rfc3339], and [wikipedia-iso-8601] says week numbers and ordinal days are
  not permitted. The crate README says the profile is stricter "in every
  direction"; no `Strictness` field refuses them.
- **RFC 3339 reads offsets the grammar forbids.** The offset hour of §5.6 is
  00-23. `rfc3339::parse` of `2026-09-21T14:30:05+24:00` and `+25:59` returns
  a value; `+26:00` is refused only because `hc_tz::UtcOffset` stops at
  25:59:59.
- **The leap second is in the wrong place for an offset.** RFC 3339 shifts
  the leap second by the offset. The parser accepts the second 60 only at
  `23:59`, in any zone. It therefore refuses §5.8's own
  `1990-12-31T15:59:60-08:00`, and accepts `1972-06-30T23:59:60+09:00`,
  which names 15:00:00 UTC, with the leap-second flag set on
  `to_utc_instant`. The README gives the limit of `CivilTime` as the
  reason for the refusal, and does not mention the acceptance.
- **The two formats mix across the date and the time.** The `ISO` preset
  reads `2026-09-21T143005` and `20260921T14:30:05+09:00`.
  [wikipedia-iso-8601] says both must use the same format; RFC 3339
  Appendix A says its grammar permits mixtures, because ISO 8601 "is not
  clear". The mixed forms the code refuses are those inside one part: the
  date, the time or the week date.
- **Reading and writing are not always the identity.** Parsing `P01D` and
  writing gives `P1D`; likewise `PT00S`, `+P1D` and `P1YT` give `PT0S`, `P1D`
  and `P1Y`. The leading zeros, the plus sign and the stray `T` are lost.
  The README promises that parse then format is byte-identical; the
  round-trip tests start from values, not text. Dates, times, zones and
  intervals of dates keep their shape.
- **A week with other elements.** `P1W2D` and `P1Y2W` are accepted. The
  sources read show `PnW` as its own form [wikipedia-iso-8601] [rfc3339],
  but state no ban on a mixture, so whether ISO 8601-1:2019 allows it is
  not known here.
- **RFC 5322 is read more loosely than its grammar.** The weekday is any
  alphabetic word (`Xyz,` and `Monday,` parse) and is ignored when it
  disagrees with the date, though §3.3 requires it to be the day implied by
  the date [rfc5322]; the module's comment says the RFC does not ask a
  parser to raise the disagreement. A year of 1 or 1899 is read though the
  year is "any numeric year 1900 or later" [rfc5322] [rfc2822], and a year of
  5 to 9 digits is read, so `21 Sep 12345 00:00:00 +0000` parses and then
  cannot be written. `21Sep2026` is read, which RFC 5322's
  obsolete forms allow (`obs-day` and `obs-year` take optional folding
  space) and RFC 2822's `obs-month`, which requires it, does not. A second
  60 away from `23:59` is refused, though the range of §3.3 is "00:00:00
  through 23:59:60". An offset of 2600 to 9959 is refused though the grammar
  has four digits and §3.3 constrains only the last two.
- **Two spellings of a year outside 0000 to 9999.** `IsoDate::from_fixed`
  writes six digits at least, `+012345-01-01` and `-000001-01-01`, as the
  formatter of `hc-format` does; the facade's `Date` display, which
  `hc_format_iso_date` uses, writes `{:+06}`, `+12345-01-01` and
  `-00001-01-01`. ISO 8601 leaves the digit count to agreement; both
  spellings are read back.
- **`-00:00` and `Z` follow RFC 3339 (2002).** Under RFC 9557, `Z` means the
  unknown offset. Read as RFC 9557 reads them, `Zulu` and
  `UnknownLocalOffset` are one value. The library has no name for that
  reading ([policy.md](../policy.md) §5).

## Sources

- [wikipedia-iso-8601]: the editions and their dates, the dates, week dates,
  ordinal dates, times, zones, durations, intervals and repeating
  intervals of ISO 8601, truncated forms, `24:00`, `-00:00`, and what the
  page says of RFC 3339 and the W3C profile. A secondary account of the
  standard, whose text was not read. Re-read as HTML on 2026-10-03.
- [wikipedia-iso-week-date]: the week of the first Thursday, 4 January and
  28 December, the long years and the week-numbering year. Secondary; the
  standard was not read. Re-read as HTML on 2026-10-03.
- [rfc3339]: Klyne and Newman, July 2002. Sections 4.2, 4.3, 5.5 to 5.8 and
  Appendices A to C: the grammar, the leap second, the offsets and the
  duration and period grammar. Read as HTML on 2026-10-03.
- [rfc9557]: Sharma and Bormann, April 2024. §2.2 and §2.3, the update of
  RFC 3339 §4.3, and the suffix syntax in outline. Read as HTML on
  2026-10-03.
- [w3c-note-datetime]: Wolf and Wicksteed, the six formats and their
  symbols. Read as HTML on 2026-10-03.
- [rfc5322]: Resnick (ed.), October 2008, §3.3, §4.3 and Appendix B. Read as
  HTML on 2026-10-03. It is the text the module documentation and the crate
  README cite.
- [rfc2822]: Resnick (ed.), April 2001, §3.3 and §4.3, read for the zone
  range, `-0000` and the obsolete forms. Read as HTML on 2026-10-03.
- [rfc9110]: Fielding, Nottingham and Reschke (eds.), June 2022, §5.6.7, the
  HTTP date and its two obsolete formats. Read as HTML on 2026-10-03.
- Not read: ISO 8601-1:2019, ISO 8601-1:2019/Amd 1:2022 and ISO 8601-2:2019,
  which ISO sells; every rule of ISO 8601 above is cited from
  [wikipedia-iso-8601] instead. ISO's own overview page of ISO 8601 answered
  a bot challenge ("Just a moment...") and was not pursued.

## Code

`crates/hc-format/src/iso8601.rs`, `iso8601/duration.rs` and
`iso8601/interval.rs`, `rfc3339.rs`, `rfc2822.rs`, `parse.rs` for the
sniffing, and `value.rs` for the value types and the spelling of a year;
the week, ordinal and Gregorian arithmetic is in
`crates/hc-calendars-solar/src/iso_week.rs`, `ordinal.rs` and `gregorian.rs`.

The tests that anchor it: in `iso8601.rs`,
`the_two_hundred_and_sixty_fourth_day_of_2026_is_the_twenty_first_of_september`,
`the_first_day_of_week_thirty_eight_of_2026_is_a_monday`,
`a_month_accuracy_date_has_no_basic_spelling`,
`iso_8601_forbids_the_negative_zero_offset`,
`twenty_four_hundred_survives_a_round_trip`,
`a_leap_second_anywhere_but_midnight_is_refused` and
`an_hours_only_offset_is_iso_but_not_rfc_3339`; in `rfc3339.rs`,
`plus_zero_and_minus_zero_name_the_same_instant_and_different_facts`,
`the_1972_leap_second_parses_and_round_trips` and
`rfc_3339_refuses_the_basic_format`; in `rfc2822.rs`,
`obsolete_two_digit_years_follow_the_rule_in_section_four_point_three`,
`an_unrecognised_or_military_zone_means_the_offset_is_unknown` and
`the_weekday_is_derived_from_the_date_not_copied_from_the_text`; in
`iso8601/duration.rs`, `a_nominal_duration_refuses_to_become_a_span` and
`a_fraction_may_not_sit_on_a_component_that_is_not_the_last`; in
`iso8601/interval.rs`, `all_four_interval_shapes_round_trip` and
`zero_repetitions_are_not_the_same_as_unbounded`; and the sweeps of
`crates/hc-format/tests/round_trip.rs`.

The WebAssembly and C exports `hc_parse_iso_date` and `hc_format_iso_date`
(`crates/hyper-calendar-wasm/src/civil.rs`,
`crates/hyper-calendar-ffi/src/civil.rs`) read a date with
`iso8601::parse_date` and refuse one of reduced accuracy: the parse gives
the fixed day or an invalid-date status. They do not reach the rest of this
document: durations, intervals, RFC 3339 and RFC 5322 have no export.
