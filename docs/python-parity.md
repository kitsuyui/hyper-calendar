# Python parity

This document maps Python's `datetime`, `time` and `calendar` modules, the
parts of `zoneinfo` and `dateutil` that have an equivalent here, and the
`humanize` package onto this library. There is one table per Python module,
class or package. Each row gives the Python name, what answers it here, and
how closely. The Status column takes three values:

- **yes** — the same question has the same answer here.
- **partial** — an answer exists and the note says what differs or is
  missing.
- **—** — nothing answers it, and the note says why.

Paths are relative to the workspace's crates; `civil` is
`hyper_calendar::civil`. The system document
[`systems/python-compatibility.md`](systems/python-compatibility.md)
explains the three conventions that need explaining — `strptime`'s regular
expression, `humanize`'s arithmetic and its gettext catalogues, and
`timedelta`'s rounding — with worked examples and the measured agreement.

## What was read, and what was run

- **Documentation.** The Python 3.13 documentation of
  [`datetime`](https://docs.python.org/3.13/library/datetime.html),
  [`time`](https://docs.python.org/3.13/library/time.html) and
  [`calendar`](https://docs.python.org/3.13/library/calendar.html), read
  2026-09-26 (`datetime`) and 2026-10-03 (all three). The tests that anchor
  each row quote the documentation's examples: `hc-core`'s `duration`
  tests, `hc-format`'s `python` module and `tests/python_directives.rs`,
  `hc-humanize`'s `natural` module and `tests/gettext_catalogues.rs`, and
  `hyper-calendar`'s `tests/python.rs`.
- **`humanize` 4.16.0**, the release of 2026-06-30 on PyPI. Its sources
  `time.py`, `number.py`, `filesize.py`, `lists.py`, `i18n.py` and
  `README.md`, and the 35 catalogues `locale/*/LC_MESSAGES/humanize.po`, were
  read at the tag `4.16.0` of
  [python-humanize/humanize](https://github.com/python-humanize/humanize),
  from raw.githubusercontent.com into memory, 2026-10-03; the documentation
  at <https://humanize.readthedocs.io/> (`time` and `number`) was read the
  same day. That documentation is built from the project's `main` branch,
  which is 29 commits past the release, so a behaviour that changed after
  4.16.0 is the release's here; the list is under *humanize*, below.
- **Interpreters, as oracles only.** CPython 3.12.14 and 3.14.7, and GNU
  `msgfmt` with Python's `gettext`, were run on this machine to answer the
  same question this library answers, over hundreds of thousands of random
  cases per function (the counts are in the system document). Nothing here
  depends on Python at build time, and a test pins a value only where the
  documentation states the rule or the test says it is an interpreter's
  answer. The interpreters disagree with the documentation in places, and
  the rows say which they follow.

## What differs everywhere, on purpose

- **No clock** (policy §13). `today()`, `now()` and `utcnow()` have no
  equivalent; every function that needs the present takes it.
- **No hidden zone.** Python's naive `datetime` is `civil::DateTime`; its
  aware one is a `DateTime` with a zone *beside* it — an
  `hc_format::ZoneInfo` from a parser, an `hc_tz::TimeZone` passed to a
  method — so a reading is never silently local time or UTC. Python's
  `fold` is `hc_tz::Disambiguation`, which can also refuse to choose.
- **Exact, and wider.** Spans are exact to the attosecond rather than the
  microsecond, years run from −9 999 999 to 9 999 999 rather than 1 to
  9999, and `23:59:60` exists. Where Python rounds a span to the
  microsecond, the unmarked method keeps the attosecond; the Python
  rounding is the same method with `Resolution::Microsecond`.
- **Operators panic, `checked_*` do not.** `+`, `-`, `*`, `/` and `%` read as
  Python's do and panic where Python raises `OverflowError` or
  `ZeroDivisionError`; each has a `checked_*` twin (policy §8).

## date

| Python | hyper-calendar | Status | Note |
| --- | --- | --- | --- |
| `date(y, m, d)` | `civil::Date::new` | yes | |
| `date.today()` | — | — | no clock |
| `date.fromtimestamp` | `civil::Date::from_timestamp` | yes | the zone is an argument (feature `tz`) |
| `date.fromordinal` | `civil::Date::from_ordinal` | yes | the ordinal is `Rd` |
| `date.fromisoformat` | `civil::Date::from_iso_format`, `hc_format::python::parse_date` | yes | feature `format`; see *fromisoformat*, below |
| `date.fromisocalendar` | `civil::Date::from_iso_calendar` | yes | |
| `date.min` / `max` | `Date::MIN` / `MAX` | yes | wider range |
| `date.resolution` | `Date::RESOLUTION` | yes | |
| `year` / `month` / `day` | `Date::year` … | yes | |
| `date + timedelta` | `Add`, `Date::checked_add_delta` | yes | whole days only, as Python |
| `date - timedelta` | `Sub`, `Date::checked_sub_delta` | yes | |
| `date - date` | `Sub`, `Date::duration_since` | yes | |
| comparisons | `Ord` | yes | |
| `replace` | `Date::replace` | yes | |
| `timetuple` | `civil::StructTime::from_date` | yes | midnight, `tm_isdst` −1; see *time* |
| `toordinal` | `Date::to_ordinal` | yes | |
| `weekday()` | `date.weekday().monday_first_number()` | yes | |
| `isoweekday()` | `Date::iso_weekday` | yes | |
| `isocalendar()` | `Date::iso_calendar` | yes | an `IsoWeekDate` |
| `isoformat()` / `str()` | `Display` | yes | expanded years outside 0000–9999 |
| `ctime()` | `Date::ctime` | yes | feature `format` |
| `strftime` | `Date::strftime`, `hc_format::patterns::strftime` | yes | C-locale names; a locale through `FormatContext` |
| `__format__` | — | — | Rust's `{}` takes no strftime pattern; call `strftime` |

## time

| Python | hyper-calendar | Status | Note |
| --- | --- | --- | --- |
| `time(h, m, s, us)` | `civil::Time::from_hms_micro`, `Time::new` | yes | `new` takes nanoseconds |
| `time.min` / `max` / `resolution` | `Time::MIN` / `MAX` / `RESOLUTION` | yes | `MAX` is inside a leap second; resolution an attosecond |
| `hour` … `microsecond` | `Time::hour` … | yes | |
| `tzinfo` / `fold` | the `ZoneInfo` beside it | partial | no aware time-of-day type |
| `time.fromisoformat` | `civil::Time::from_iso_format`, `hc_format::python::parse_time` | yes | returns the zone beside the time; see *fromisoformat* |
| `replace` | `Time::replace` | yes | |
| `isoformat(timespec)` | `Time::iso_format`, `hc_format::python::write_time` | yes | |
| `str()` | `Display` | partial | trims fraction zeros (`01:02:03.5`); `iso_format(TimeSpec::Auto)` is Python's |
| `strftime` | `Time::strftime` | yes | dated 1900-01-01, as Python |
| `utcoffset` / `dst` / `tzname` | — | — | a time of day names no instant for a zone to answer about |
| comparisons | `Ord` | yes | |

## datetime

| Python | hyper-calendar | Status | Note |
| --- | --- | --- | --- |
| `datetime(...)` | `civil::DateTime::from_parts` | yes | |
| `today` / `now` / `utcnow` | — | — | no clock |
| `fromtimestamp` | `DateTime::from_timestamp` | yes | the zone is an argument |
| `utcfromtimestamp` | `DateTime::from_timestamp_utc` | yes | |
| `fromordinal` | `DateTime::from_ordinal` | yes | |
| `combine` | `DateTime::combine` | yes | |
| `fromisoformat` | `DateTime::from_iso_format`, `hc_format::python::parse_date_time` | yes | any one-character separator, as Python; see *fromisoformat* |
| `fromisocalendar` | `DateTime::from_iso_calendar` | yes | |
| `strptime` | `DateTime::strptime`, `hc_format::python::strptime`, the boundary's `hc_parse_pattern` with `python` | yes | CPython's regular-expression rules; 1900-01-01 defaults; see *strptime* |
| `min` / `max` / `resolution` | `DateTime::MIN` / `MAX` / `RESOLUTION` | yes | |
| attributes | `date` / `time` fields | yes | |
| `tzinfo` / `fold` | a zone beside it, `hc_tz::Disambiguation` | partial | no aware type; see above |
| `dt + timedelta` | `Add`, `DateTime::checked_add_delta` | yes | nominal 86 400-second days |
| `dt - timedelta` | `Sub`, `DateTime::checked_sub_delta` | yes | |
| `dt - dt` | `Sub`, `DateTime::nominal_duration_since` | yes | naive; aware through `timestamp` |
| comparisons | `Ord` | yes | naive |
| `date()` / `time()` | fields | yes | |
| `timetz()` | — | — | there is no aware time of day (a time names no instant for a zone to answer about); `time()` and the zone beside it carry the same two facts |
| `replace` | `DateTime::replace` with `civil::Replace` | yes | no `tzinfo` or `fold` keyword |
| `astimezone` | `DateTime::astimezone` | yes | both zones are arguments |
| `utcoffset` | `DateTime::utc_offset` | yes | |
| `dst` / `tzname` | `TimeZone::is_dst_at` / `abbreviation_at` | partial | on the zone, at the instant; `dst` is a flag |
| `timetuple` | `civil::StructTime::from_date_time` | yes | the flag is an argument, −1 for a naive reading |
| `utctimetuple` | `StructTime::gmtime` of `DateTime::timestamp_utc` | partial | of an instant, not of an aware `datetime` |
| `toordinal` | via `date` | yes | |
| `timestamp` | `DateTime::timestamp`, `timestamp_utc` | yes | |
| `weekday` / `isoweekday` / `isocalendar` | via `date` | yes | |
| `isoformat(sep, timespec)` | `DateTime::iso_format`, `hc_format::python::write_date_time` | yes | |
| `ctime` | `DateTime::ctime` | yes | |
| `strftime` | `DateTime::strftime` | yes | |

## timedelta

`Resolution` says which unit a scaled or divided span is rounded to, half to
even: `Attosecond`, the span's own, or `Microsecond`, Python's. Python
multiplies by the float's exact ratio (`float.as_integer_ratio()`) and
rounds once; so does `Duration::scale_f64_nearest`, in 256-bit integers.

| Python | hyper-calendar | Status | Note |
| --- | --- | --- | --- |
| `timedelta(weeks=…, …, microseconds=…)` | `TimeDelta::from_parts` with `TimeDeltaParts` | partial | integer arguments; a fractional one through `Duration::from_secs_f64` |
| `min` / `max` / `resolution` | `TimeDelta::MIN` / `MAX` / `RESOLUTION` | yes | wider range, attosecond resolution |
| `days` / `seconds` / `microseconds` | `TimeDelta::days` … | yes | normalised as Python |
| `total_seconds` | `TimeDelta::total_seconds` | yes | |
| `+` / `-` | `Add` / `Sub`, `checked_add` / `checked_sub` | yes | |
| unary `-`, `+`, `abs` | `Neg`, `TimeDelta::abs` | yes | Rust has no unary `+` |
| `* int` | `Mul<i64>`, `TimeDelta::checked_mul` | yes | |
| `* float` | `TimeDelta::checked_scale_at(f, Resolution::Microsecond)` | yes | `checked_scale` is the same product exact to the attosecond |
| `td / td` | `TimeDelta::ratio` | yes | the double nearest the exact quotient, as Python's integer true division |
| `td / int` | `TimeDelta::checked_div_nearest_at(i, Resolution::Microsecond)` | yes | |
| `td / float` | `TimeDelta::checked_div_f64_at(f, Resolution::Microsecond)` | yes | |
| `td // int` | `Div<i64>` | partial | floors at the attosecond, not the microsecond |
| `td // td` | `TimeDelta::checked_div_floor` | yes | |
| `td % td` | `Rem`, `TimeDelta::checked_rem` | yes | the divisor's sign, as Python |
| `divmod` | `TimeDelta::checked_div_rem` | yes | |
| comparisons | `Ord` | yes | |
| `bool` | `TimeDelta::is_zero` | yes | |
| `str` | `Display`, `Duration::days_and_clock` | yes | more than six fraction digits only below a microsecond |
| `repr` | — | — | `Debug` is Rust's |
| `hash` | `Hash` | yes | |

Beyond Python's range the answers differ by design: `timedelta` refuses a
span of 10⁹ days and this library does not.

## tzinfo, timezone and zoneinfo

| Python | hyper-calendar | Status | Note |
| --- | --- | --- | --- |
| `tzinfo` | `hc_tz::TimeZone` | yes | |
| `timezone(offset, name)` | `hc_tz::FixedTimeZone` | yes | |
| `timezone.utc`, `datetime.UTC` | `hc_tz::Utc` | yes | |
| `utcoffset` | `TimeZone::offset_at` | yes | |
| `dst` | `TimeZone::is_dst_at` | partial | a flag, not the amount |
| `tzname` | `TimeZone::abbreviation_at` | yes | |
| `fromutc` | `TimeZone::local_at` | yes | |
| `fold` | `hc_tz::Disambiguation`, `LocalResolution` | yes | three answers, not two |
| `zoneinfo.ZoneInfo` | `hc_tz::builtin::zone`, `TzifTimeZone` | yes | |
| `ZoneInfo.key` | `TimeZone::name` | yes | |
| `zoneinfo.available_timezones()` | `hc_tz::builtin::ids` | partial | the built-in table's names; a zone loaded from TZif has the name its caller gave |
| `ZoneInfo.from_file`, `no_cache`, `clear_cache`, `TZPATH` | — | — | no I/O beyond reading a TZif file a caller hands over, and no cache that outlives a call (policy §13) |
| `timezone.min` / `max` | `hc_tz::MAX_OFFSET_SECONDS` | partial | ±25:59:59, not ±23:59; the Python profile of the parsers refuses 24 hours or more |

## time (the module)

`civil::StructTime` is `time.struct_time`: nine integers that are not checked
when built (`calendar.timegm` is documented to take whatever they add up
to), with `to_date_time` where a tuple that names no moment is refused. At
the boundary `hc_gmtime`, `hc_timegm`, `hc_localtime`, `hc_mktime` and
`hc_asctime` are `gmtime`, `timegm`, `localtime`, `mktime` and `asctime`, a
`struct_time` crossing as one line of its nine fields.

| Python | hyper-calendar | Status | Note |
| --- | --- | --- | --- |
| `struct_time` | `civil::StructTime` | partial | the nine fields; `tm_zone` and `tm_gmtoff` are the zone's (`TimeZone::abbreviation_at`, `offset_at`) and a reading has no zone of its own |
| `gmtime(seconds)` | `StructTime::gmtime` | yes | whole seconds; a fraction is dropped, as Python's |
| `localtime(seconds)` | `StructTime::localtime(unix, &zone)` | yes | the zone is an argument (feature `tz`); with none, there is no local time |
| `mktime(tuple)` | `StructTime::mktime(&zone, policy)` | partial | a `Disambiguation` where Python reads `tm_isdst`, listed by `hc_mktime_policies`, with `hc_local_resolution` for what comes before the policy; a `tm_sec` of 60 is refused on every day, as `datetime(*tuple[:6])` refuses it |
| `strftime(format, tuple)` | `StructTime::strftime` | yes | feature `format` |
| `strptime(string, format)` | `StructTime::strptime` | yes | the default format `"%a %b %d %H:%M:%S %Y"` is the caller's to pass; a second of 61 is refused |
| `asctime(tuple)` | `StructTime::asctime` | partial | the weekday is the date's, as `datetime.ctime` has it, where Python's reads `tm_wday` of the tuple; `hc_asctime` takes the six fields and a year of 1 to 9999 |
| `ctime(seconds)` | `DateTime::ctime` of `from_timestamp` | yes | in a zone the caller gives |
| `timezone`, `altzone`, `daylight`, `tzname`, `tzset` | — | — | the machine's zone; a zone is an argument and `TimeZone` answers each at an instant |
| `time`, `time_ns`, `monotonic`, `perf_counter`, `process_time`, `sleep` | — | — | no clock (policy §13) |

## calendar

`civil::calendar` holds the functions of the module that work on a year, a
month or a day; at the boundary they are `hc_isleap`, `hc_leapdays`,
`hc_calendar_weekday`, `hc_monthrange` and `hc_monthcalendar`.

| Python | hyper-calendar | Status | Note |
| --- | --- | --- | --- |
| `isleap(year)` | `calendar::is_leap_year`, `Date::is_leap_year` | yes | |
| `leapdays(y1, y2)` | `calendar::leapdays` | yes | counts backwards when `y2` is before `y1`, as Python's |
| `weekday(y, m, d)` | `calendar::weekday`, `Date::weekday` | yes | Monday 0; exact in every year |
| `monthrange(y, m)` | `calendar::monthrange` | yes | |
| `monthcalendar(y, m)` | `calendar::monthcalendar(y, m, first_weekday)` | yes | the first weekday is an argument |
| `timegm(tuple)` | `StructTime::timegm` | yes | the day, hour, minute and second are not checked; a month outside 1–12 is refused |
| `setfirstweekday`, `firstweekday` | — | — | a process-wide setting; the first weekday is an argument (policy §13) |
| `day_name`, `day_abbr`, `month_name`, `month_abbr` | `Date::strftime("%A")` and its kin; `hc_i18n` names for a locale | partial | names of a date, not a table indexed by number |
| `MONDAY` … `SUNDAY`, `Day` | `hc_calendar::Weekday` | yes | |
| `JANUARY` … `DECEMBER`, `Month` | — | — | a month is its number 1–12 |
| `Calendar.itermonthdates`, `monthdatescalendar`, `monthdays2calendar`, `itermonthdays3`, `itermonthdays4`, `yeardatescalendar`, `yeardays2calendar`, `yeardayscalendar` | — | — | not yet done: they are compositions of `monthcalendar` (a row of `width` months for a year), and the documentation names them without the exact shape of each tuple |
| `TextCalendar`, `prmonth`, `prcal`, `HTMLCalendar`, `LocaleTextCalendar` | — | — | not yet done: their layout (centring, widths, the HTML's class names) is specified by `calendar.py`'s source, which was not read; the documentation lists the methods and the CSS class attributes only |
| `weekheader`, `IllegalMonthError`, `IllegalWeekdayError` | — | — | follow the text calendar's layout and Python's exception classes, which Rust's errors do not mirror |

## dateutil

`dateutil` is a third-party package, and its documentation was not read for
this table: each row names what here does a comparable job, and none claims
that the two agree on every input.

| Python | hyper-calendar | Status | Note |
| --- | --- | --- | --- |
| `relativedelta(months=…, years=…)` | `Date::add_months`, `add_years`, `DateTime` plus a `TimeDelta` | partial | the steps clamp the day to the target month's length; the other fields (`weekday=`, `yearday=`, absolute replacement) are not carried |
| `easter.easter(year, method)` | `hc_holiday::computus::easter`, `gregorian_easter`, `orthodox_easter`, `orthodox_easter_julian_date` | partial | the Western, Orthodox and Julian reckonings, as `Rd` or as a date; `dateutil`'s own tests were not run |
| `tz.gettz`, `tz.tzlocal`, `tz.tzutc`, `tz.tzfile` | `hc_tz` | partial | named zones and fixed offsets; no local zone (policy §13) |
| `rrule`, `rruleset` | — | — | not yet done: iCalendar recurrence rules (RFC 5545 §3.3.10) have no engine here, and the RFC was not read for this table; `hc-holiday`'s rules answer the library's own recurring days |
| `parser.parse` (fuzzy parsing) | — | — | not yet done: `hc-format` parses ISO 8601, RFC 3339, RFC 2822 and a pattern exactly, and no source for `dateutil`'s guessing rules was read |

## fromisoformat

`hc_format::python`, over `hc_format::iso8601`'s scanner with Python's
profile. The documentation (3.13) says `fromisoformat` takes "any valid ISO
8601 format" with exceptions, and the table of what an interpreter does
beyond it is in the system document.

Carried: `YYYY-MM-DD`, `YYYYMMDD`, `YYYY-Www-D`, `YYYYWwwD`, and a week
without a day, `YYYY-Www`, which is the week's Monday; times of `HH`,
`HH:MM`, `HH:MM:SS` and their basic forms, a fraction after `.` or `,` of
any length (more than six digits are truncated, here to eighteen), an
optional leading `T` on a time alone, any one character between date and
time; `Z`, `±HH`, `±HHMM`, `±HH:MM`, and offsets with seconds.

Differences, each a deliberate choice or a limit:

- **Wider.** Year 0 (`0000-01-01`); a second
  of 60 (`23:59:60`), which Python's `datetime` refuses. Both are what this
  library holds and Python cannot.
- **Narrower.** An offset of 24 hours or more is refused, as Python's
  `timezone` does; an offset with a fraction of a second is refused,
  because `UtcOffset` counts whole seconds (the documentation lists it as
  an exception to ISO 8601).
- **Behaviours of CPython's C parser that the documentation does not
  describe, and this library does not copy.** One character of any kind
  between the time and its offset (`10:20:30 +05:30`); an offset's minutes
  or seconds above 59 (`+05:60`); a fraction after the hour or the minute
  (`10.5`, `10:20.5`) or after a colon (`10:20:30:40`), or an empty one
  (`10:20:30.`); and, on CPython 3.14 only, `24:00`. They are refused here.
  The reason is that no source states them: the documentation says
  "Fractional hours and minutes are not supported".
- **An ambiguous string is read differently.** `2019-W01-1102-05` has two
  readings (a weekday `1` and a separator `1`, or the week and a separator
  `-`); CPython chooses by the length of the text, and this parser reads
  the longest date.

## strptime

`hc_format::python::strptime` is CPython's `_strptime` as code: a pattern is
a regular expression with ordered alternatives, matched with backtracking,
so `%Y%m%d` reads `20191204` and a parser that reads a year greedily does
not. The system document gives the alternatives and a worked example.
Checked against CPython 3.12.14 over about 400 000 random patterns and texts, and
by the 233 hand-picked cases of the probe, with the differences below.

| Directive | Format | Parse | Status |
| --- | --- | --- | --- |
| `%a` `%A` `%w` `%d` `%b` `%B` `%m` `%y` `%Y` `%H` `%I` `%p` `%M` `%S` `%j` `%c` `%x` `%X` `%%` `%G` `%u` `%V` `%:z` | yes | yes | yes (23 rows) |
| `%f` | six digits, truncated | one to six digits | yes |
| `%z` | `+HHMM`, `+HHMMSS` with seconds | `±HH:MM`, `±HHMM`, with seconds, `Z`; the colons must agree | yes |
| `%Z` | yes | `UTC` and `GMT`, which set no zone | partial |
| `%U` `%W` | yes | with a year and a weekday, as Python | yes (2 rows) |
| missing fields default to 1900-01-01 | — | `hc_format::python::strptime` | yes |

Differences from CPython:

- A digit is `0`–`9`; CPython's `\d` is any Unicode decimal digit, so it
  reads `２０１９` and `٢٠١٩` as 2019. Not yet done: no Unicode decimal-digit
  table is carried.
- `%Z` accepts `UTC` and `GMT`. CPython also accepts the abbreviations of
  the machine's own zone, which a library with no hidden zone does not have.
- `%z` with a fraction of a second is refused (`UtcOffset` counts whole
  seconds), and an offset of 24 hours or more, as CPython's `timezone`.
- Year 0 (`%Y` of `0000`) and a year the date's rules carry beyond 9999,
  which Python refuses; and a second of 60.
- CPython 3.14 adds `%e`, flags and widths (`%-d`, `%5Y`) and a space-padded
  `%H`; the 3.13 documentation says none of them, and they are refused.
- The documentation says the leading zero of `%y` is optional on parsing;
  the interpreters read `%y` as exactly two digits, and so does this.
- CPython 3.12 reads a bare `%` as a pattern; 3.11 and 3.14 refuse it, as
  this does.

## humanize

`hc_humanize::natural::Natural`, with `humanize` 4.16.0's thresholds,
arithmetic and strings. The CLDR-phrased formatters elsewhere in
`hc-humanize` are a separate convention (policy §5) and are not changed by
this. English is `NaturalPhrases::ENGLISH`; a language is one more
`NaturalPhrases`, and the 35 that `humanize` ships are carried. Every function
in the table below that writes words is a boundary export of the `natural`
layer (`hc_naturaldelta`, `hc_naturaltime`, `hc_precisedelta`, `hc_naturalday`,
`hc_naturaldate`, `hc_ordinal`, `hc_intcomma`, `hc_intcomma_float`, `hc_apnumber`,
`hc_intword` and the rest), in the catalogue that serves a locale.

| Python | hyper-calendar | Status | Note |
| --- | --- | --- | --- |
| `naturaldelta` | `Natural::naturaldelta` | yes | whole years, `days // 365`, as 4.16.0 |
| `naturaltime` | `Natural::naturaltime`, `naturaltime_delta` | yes | `now` is an argument |
| `naturalday` | `Natural::naturalday` | yes | today is an argument; C-locale `strftime` |
| `naturaldate` | `Natural::naturaldate` | yes | |
| `precisedelta` | `Natural::precisedelta` | yes | the arithmetic step for step, including its rounding, carries and float microseconds; `format="%0.2f"` is `decimals: 2` |
| `ordinal(value, gender)` | `Natural::ordinal`, `write_ordinal_of` with `Gender` | yes | `Gender::Male` is the default |
| `intcomma` | `Natural::intcomma`, `intcomma_f64` | yes | the separators are an argument; `NaturalPhrases::grouping` is the language's own |
| `intword` | `Natural::intword`, `intword_digits`, `intword_f64` | yes | up to the googol: an `i128` stops short of it, and the digits do not |
| `apnumber` | `Natural::apnumber`, `apnumber_f64` | yes | |
| `fractional` | `Natural::fractional` | yes | |
| `scientific` | `Natural::scientific` | yes | |
| `metric` | `Natural::metric` | yes | |
| `clamp` | `Natural::clamp` with `ClampFormat` | partial | Python's `format` is a format string or a function; here a closed set of four shapes and a function |
| `naturalsize` | `Natural::naturalsize` with `SizeStyle` | yes | `binary` and `gnu` are two flags in Python |
| `natural_list` | `Natural::naturallist` | yes | not translated, in Python either |
| `i18n.activate`, `deactivate` | `NaturalPhrases::by_catalogue`, `by_language`, `Natural::for_catalogue` | yes | no process-wide current language: it is the `Natural` you hold |
| the gettext catalogues | `NaturalPhrases::catalogues()` | yes | 35, generated from the `.po` files; a fuzzy or untranslated message is English, as `msgfmt` compiles it |
| the catalogue for a locale | `NaturalPhrases::for_locale`, and the `natural` boundary lines | no | Python has no locale lookup. A locale is served only by a catalogue that translates every message of the function, so a partly translated catalogue (Japanese's `precisedelta`, whose *and* is untranslated) answers in English whole; `by_catalogue` keeps Python's own result, raw `%d` and `%(value)s` included (Korean, Bengali, Vietnamese in `intword`) |
| `thousands_separator`, `decimal_separator` | `NaturalPhrases::grouping` | yes | |

Things that are not `humanize` 4.16.0 here, and why:

- **Not carried: behaviour the project's `main` changed after the release**,
  which the next release will carry. `naturaldelta` rounds the years of
  a span of two or more (`round(days / 365)`, so 1000 days is 3 years where
  4.16.0 says 2); `fractional` folds a fraction that reduces to a whole
  number into the integer (`0.9999` is `1`, `0.0` is `0`, where 4.16.0
  writes `1/1` and `0/1`); `intword` carries a rounded value into the next
  power at any size (`10**24 - 1` is `1.0 septillion`, where 4.16.0 writes
  `1000.0 sextillion`); `metric` carries `9.999` into `10.0`; the
  French decimal separator is a comma (4.16.0's is a full stop); and a
  Sinhala catalogue (`si_LK`). A caller who wants the later behaviour reads
  the release it lands in; the reason for following the release is that
  `pip install humanize` gives it.
- **`naturaldelta`, `naturaltime` and the other time functions take a typed
  `Duration`** and not "a number, a string or a `datetime`", so Python's
  `return str(value)` paths for a value that is not a number do not exist
  (type errors here), and an `OverflowError` for a span beyond a
  `timedelta` is `HumanizeError::Overflow` beyond about 10²⁶ years.
- **Not carried: `intcomma`, `ordinal` and `apnumber` of a string.** The
  arguments are typed, as in the item above, so a caller passes the number;
  reading a number from text is not yet done.

## What is not carried yet, and why

Each is a follow-up, not a decision (policy §13):

- `calendar`'s text and HTML calendars and the year layouts: layouts
  specified only by `calendar.py`'s source, which was not read.
- `dateutil.rrule` and `dateutil.parser`: no source read; no engine.
- Unicode decimal digits in `strptime`: no table of them is carried.
- An aware time of day (`datetime.timetz`): not yet done. No type holds a
  time of day with a zone, and a time names no instant for a zone to answer
  about, so `time()` and the zone beside it carry the same two facts.
