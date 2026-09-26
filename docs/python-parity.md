# Python parity

Requirement 2 of the brief asks for "the basics of Python's `datetime`,
`time`, `date`, `timedelta` and `humanize`". This is the correspondence, one
table per Python class or package: the Python name, what answers it here,
and how closely.

- **yes** — the same question has the same answer here.
- **partial** — an answer exists and the note says what differs or is
  missing.
- **—** — nothing answers it, and the note says why.

Paths are relative to the workspace's crates; `civil` is
`hyper_calendar::civil`. The Python behaviour is that of the Python 3.13
documentation of [`datetime`](https://docs.python.org/3/library/datetime.html)
and of [`humanize`](https://humanize.readthedocs.io/) 4.x, both read
2026-09-26, and the tests that anchor each row quote the documentation's
examples: `hc-core`'s `duration` tests, `hc-format`'s `python` module and
`tests/python_directives.rs`, `hc-humanize`'s `natural` module and
`hyper-calendar`'s `tests/python.rs`.

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
  microsecond, this floors at the attosecond.
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
| `date.fromisoformat` | `civil::Date::from_iso_format`, `hc_format::python::parse_date` | yes | feature `format` |
| `date.fromisocalendar` | `civil::Date::from_iso_calendar` | yes | |
| `date.min` / `max` | `Date::MIN` / `MAX` | yes | wider range |
| `date.resolution` | `Date::RESOLUTION` | yes | |
| `year` / `month` / `day` | `Date::year` … | yes | |
| `date + timedelta` | `Add`, `Date::checked_add_delta` | yes | whole days only, as Python |
| `date - timedelta` | `Sub`, `Date::checked_sub_delta` | yes | |
| `date - date` | `Sub`, `Date::duration_since` | yes | |
| comparisons | `Ord` | yes | |
| `replace` | `Date::replace` | yes | |
| `timetuple` | — | — | no `struct_time`; the fields are accessors |
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
| `time.fromisoformat` | `civil::Time::from_iso_format`, `hc_format::python::parse_time` | yes | returns the zone beside the time |
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
| `fromisoformat` | `DateTime::from_iso_format`, `hc_format::python::parse_date_time` | yes | any one-character separator, as Python |
| `fromisocalendar` | `DateTime::from_iso_calendar` | yes | |
| `strptime` | `DateTime::strptime`, `hc_format::python::strptime` | yes | 1900-01-01 defaults |
| `min` / `max` / `resolution` | `DateTime::MIN` / `MAX` / `RESOLUTION` | yes | |
| attributes | `date` / `time` fields | yes | |
| `tzinfo` / `fold` | a zone beside it, `hc_tz::Disambiguation` | partial | no aware type; see above |
| `dt + timedelta` | `Add`, `DateTime::checked_add_delta` | yes | nominal 86 400-second days |
| `dt - timedelta` | `Sub`, `DateTime::checked_sub_delta` | yes | |
| `dt - dt` | `Sub`, `DateTime::nominal_duration_since` | yes | naive; aware through `timestamp` |
| comparisons | `Ord` | yes | naive |
| `date()` / `time()` | fields | yes | |
| `replace` | `DateTime::replace` with `civil::Replace` | yes | no `tzinfo` or `fold` keyword |
| `astimezone` | `DateTime::astimezone` | yes | both zones are arguments |
| `utcoffset` | `DateTime::utc_offset` | yes | |
| `dst` / `tzname` | `TimeZone::is_dst_at` / `abbreviation_at` | partial | on the zone, at the instant; `dst` is a flag |
| `timetuple` / `utctimetuple` | — | — | no `struct_time` |
| `toordinal` | via `date` | yes | |
| `timestamp` | `DateTime::timestamp`, `timestamp_utc` | yes | |
| `weekday` / `isoweekday` / `isocalendar` | via `date` | yes | |
| `isoformat(sep, timespec)` | `DateTime::iso_format`, `hc_format::python::write_date_time` | yes | |
| `ctime` | `DateTime::ctime` | yes | |
| `strftime` | `DateTime::strftime` | yes | |

## timedelta

| Python | hyper-calendar | Status | Note |
| --- | --- | --- | --- |
| `timedelta(weeks=…, …, microseconds=…)` | `TimeDelta::from_parts` with `TimeDeltaParts` | partial | integer arguments; a fractional one through `Duration::from_secs_f64` |
| `min` / `max` / `resolution` | `TimeDelta::MIN` / `MAX` / `RESOLUTION` | yes | wider range, attosecond resolution |
| `days` / `seconds` / `microseconds` | `TimeDelta::days` … | yes | normalised as Python |
| `total_seconds` | `TimeDelta::total_seconds` | yes | |
| `+` / `-` | `Add` / `Sub`, `checked_add` / `checked_sub` | yes | |
| unary `-`, `+`, `abs` | `Neg`, `TimeDelta::abs` | yes | Rust has no unary `+` |
| `* int` | `Mul<i64>`, `TimeDelta::checked_mul` | yes | |
| `* float` | `TimeDelta::checked_scale` | partial | through `f64`, not Python's half-even microsecond |
| `td / td` | `TimeDelta::ratio` | yes | |
| `td / int`, `td / float` | `Div<i64>`, `checked_scale` | partial | floors at the attosecond; Python rounds half to even at the microsecond |
| `td // int` | `Div<i64>` | partial | floors at the attosecond, not the microsecond |
| `td // td` | `TimeDelta::checked_div_floor` | yes | |
| `td % td` | `Rem`, `TimeDelta::checked_rem` | yes | the divisor's sign, as Python |
| `divmod` | `TimeDelta::checked_div_rem` | yes | |
| comparisons | `Ord` | yes | |
| `bool` | `TimeDelta::is_zero` | yes | |
| `str` | `Display`, `Duration::days_and_clock` | yes | more than six fraction digits only below a microsecond |
| `repr` | — | — | `Debug` is Rust's |
| `hash` | `Hash` | yes | |

## tzinfo and timezone

| Python | hyper-calendar | Status | Note |
| --- | --- | --- | --- |
| `tzinfo` | `hc_tz::TimeZone` | yes | |
| `timezone(offset, name)` | `hc_tz::FixedTimeZone` | yes | |
| `timezone.utc` | `hc_tz::Utc` | yes | |
| `utcoffset` | `TimeZone::offset_at` | yes | |
| `dst` | `TimeZone::is_dst_at` | partial | a flag, not the amount |
| `tzname` | `TimeZone::abbreviation_at` | yes | |
| `fromutc` | `TimeZone::local_at` | yes | |
| `fold` | `hc_tz::Disambiguation`, `LocalResolution` | yes | three answers, not two |
| `zoneinfo.ZoneInfo` | `hc_tz::builtin::zone`, `TzifTimeZone` | yes | |
| `timezone.min` / `max` | `hc_tz::MAX_OFFSET_SECONDS` | partial | ±25:59:59, not ±23:59 |

## strftime and strptime directives

`hc_format::patterns::strftime`, both directions, checked directive by
directive against the documentation's table in
`crates/hc-format/tests/python_directives.rs`.

| Directive | Format | Parse | Status |
| --- | --- | --- | --- |
| `%a` `%A` `%w` `%d` `%b` `%B` `%m` `%y` `%Y` `%H` `%I` `%p` `%M` `%S` `%j` `%c` `%x` `%X` `%%` `%G` `%u` `%V` `%:z` | yes | yes | yes (23 rows) |
| `%f` | six digits, truncated | one to six digits | yes |
| `%z` | `+HHMM`, `+HHMMSS` with seconds | yes | yes |
| `%Z` | yes | RFC 5322 names only | partial |
| `%U` `%W` | yes | with a year and a weekday, as Python | yes (2 rows) |
| missing fields default to 1900-01-01 | — | `hc_format::python::strptime` | yes |

Two differences remain in `%Z`: parsing resolves only the names RFC 5322
gives an offset, because an abbreviation does not name a zone (`CST` is
three), and `%z` offsets with a fraction of a second, which Python accepts,
are refused because `hc_tz::UtcOffset` counts whole seconds. Locale names
come from `hc-i18n` when a `FormatContext` carries a locale, and are the C
locale's otherwise, as Python's are without `setlocale`.

## humanize

`hc_humanize::natural::Natural`, with `humanize`'s thresholds and strings.
The CLDR-phrased formatters elsewhere in `hc-humanize` are a separate
convention and are not changed by this.

| Python | hyper-calendar | Status | Note |
| --- | --- | --- | --- |
| `naturaldelta` | `Natural::naturaldelta` | yes | |
| `naturaltime` | `Natural::naturaltime`, `naturaltime_delta` | yes | `now` is an argument |
| `naturalday` | `Natural::naturalday` | yes | today is an argument; C-locale `strftime` |
| `naturaldate` | `Natural::naturaldate` | yes | |
| `precisedelta` | `Natural::precisedelta` | yes | `format="%0.2f"` is `decimals: 2` |
| `ordinal` | `Natural::ordinal` | yes | no `gender`: English does not need it |
| `intcomma` | `Natural::intcomma`, `intcomma_f64` | yes | separators are an argument, `Grouping::ENGLISH` by default |
| `intword` | `Natural::intword` | yes | the googol is beyond `i128` |
| translations (`i18n.activate`) | `NaturalPhrases` | partial | English only |
| `naturalsize` | — | — | bytes, not time: out of scope |

`humanize` ships gettext catalogues for about forty languages. None is
carried: they were not read, and a translation that has not been read is
not one this library can vouch for. A language is one more
`NaturalPhrases` value, and its plurals are chosen by
`hc_i18n::PluralRules`. The number functions `humanize` also has —
`apnumber`, `fractional`, `scientific`, `clamp`, `metric`, `naturallist` —
are not time and are not attempted.
