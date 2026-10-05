# Week rules: the first day of the week, the minimal days of a first week, and the week numbers they give

Backs `hc-calendar`'s `week` module, `WeekRule` (`ISO`, `WORLD`, `new`,
`first_day`, `min_days`, `position`, `week_one_start`, `week_of_year`,
`weeks_in_year`, `week_of_month`, `to_fixed`), the one week arithmetic the
workspace has, which `hc-calendars-solar`'s `iso_week` counts its weeks by; and
`hc-i18n`'s `week` module, `week::for_locale`, which reads a locale's rule from
the regional data (`locale::region_min_days`, `locale::region_first_day_of_week`,
`data::REGION_MIN_DAYS`) and `names::first_day_of_week`. Through them it backs
`hc-format`'s CLDR pattern letters `w`, `Y`, `W`, `e` and `c`, and the
`strftime` conversions `%U` and `%W` (`Fields::week_of_year_sunday` and
`week_of_year_monday`), and the columns 14 and 15 of `hc_locale_info`. No
calendar identifier is registered: a week rule numbers the weeks of the
Gregorian year and month, it does not count days. The ISO 8601 week-date
calendars `iso8601-week` and the Python `isocalendar` of the facade are
`hc-calendars-solar`'s `iso_week` and are not described here; this document
says how the locale rule relates to them.

## What it is

A week is seven days, and every place agrees on that. It does not agree on
which day begins a week, nor on which week is the first of a year. A week
number therefore depends on two facts about the place that writes it, and CLDR
carries both in `weekData`: `firstDay`, the day a week begins on, and
`minDays`, "minimal days required in the first week of a month or year"
[uts35-v48]. If a first week must hold one day of the year, `minDays` is 1; if
it must hold all seven, 7.

UTS #35 Part 4 defines the week of the year from them: "Week 1 for a year is
the first week that contains at least the specified minimum number of days from
that year. Weeks between week 1 of one year and week 1 of the following year
are numbered sequentially from 2 to 52 or 53 (if needed)." Its example is 1998,
which began on a Thursday. With Monday as the first day and four minimal days,
"these are the values reflecting ISO 8601 and many national standards", week 1
of 1998 starts on 29 December 1997 and ends on 4 January 1998. With Sunday as
the first day, week 1 starts on 4 January 1998 and ends on the 10th, and "the
first three days of 1998 are then part of week 53 of 1997". "Values are
similarly calculated for the Week of Month" [uts35-v48].

CLDR's `weekData` lists the regions that start the week on a day other than
Monday: one on Friday (Maldives), fourteen on Saturday (AF BH DJ DZ EG IQ IR JO
KW LY OM QA SD SY) and fifty-six on Sunday, among them the United States,
Japan, India, Brazil and Portugal. It lists 44 regions whose first week needs
four days (most of Europe, with Fiji and the Antilles); every other region,
and the world (`001`), needs one [cldr48-supplemental]. So the United States,
Sunday and one, and Germany, Monday and four, are two points of one scale, and
ISO 8601's Monday and four is a third.

## How it works

A `WeekRule` is the pair (first day, `minDays`) with `minDays` kept in 1 to 7.

- `position(weekday)` is a weekday's place in the week, counted from the first
  day as 0.
- `week_one_start(year)` is the first day of week 1 of a Gregorian year. Take
  the first day of the week on or before 1 January; the week from it holds
  `7 − (1 January − that day)` days of the year. If that is at least `minDays`
  it is week 1, else week 1 starts a week later.
- `week_of_year(day)` is the week-numbering year and the week: the whole
  weeks since week 1, plus one, and a day outside its Gregorian year's weeks
  is in the neighbouring year's last or first week (the rustdoc states the
  contract).
- `weeks_in_year(year)` is the distance between week 1 of this year and of the
  next, in weeks: 52 or 53.
- `week_of_month(day)` counts from the month's first week, the one in which
  the month has `minDays` days; a partial week before it is week 0.
- `to_fixed(year, week, weekday)` is the inverse, and a week past the year's
  last runs into the next year, which is how UTS #35 asks a parser to read
  it. The boundary's `hc_fixed_from_week` is this inverse for a week date that
  exists: a week the year does not have, or a weekday outside 1 to 7, is
  `HC_ERR_INVALID_DATE` and not read forward, and the sweep
  `a_week_date_names_the_day_week_of_year_gave_it` (in
  `crates/hyper-calendar/src/python_lines.rs`) holds it to `hc_week_of_year`
  over every day of three years under four rules, with CPython's
  `date.fromisocalendar` for week 53 of 2020, day 5, which is 1 January 2021.
- `for_locale(locale)` takes the first day from the tag's `-u-fw-` key, else
  from its region's row of `firstDay` (Monday where the region is not listed),
  else, for a tag with no region, from the data entry of its language, which
  follows the region CLDR's likely subtags give the language; and `minDays`
  from the region, or from the language's likely region. A tag with a `-u-fw-`
  key moves the first day and keeps the region's `minDays`.

**A worked example: 1 January 2021, a Friday (fixed day 737 791).**

- *ISO 8601, Monday and four.* The Monday on or before 1 January is 28
  December 2020; the week from it holds 7 − 4 = 3 days of 2021, fewer than
  four, so week 1 of 2021 starts on Monday 4 January 2021. 1 January is
  before it, so it is in the last week of 2020. Week 1 of 2020 starts on
  Monday 30 December 2019 (the Monday on or before 1 January 2020, a
  Wednesday, holds 7 − 2 = 5 days of 2020), 368 days before, and 368 ÷ 7 =
  52 whole weeks, so the day is in week 53 of 2020, which has 53 weeks
  (week 1 of 2021 starts 371 days after week 1 of 2020).
  `WeekRule::ISO.week_of_year` is `(2020, 53)`.
- *United States, Sunday and one.* The Sunday on or before 1 January is 27
  December 2020, and the week from it holds 7 − 5 = 2 days of 2021, at least
  one, so it is week 1 of 2021. The day is in week 1 of 2021:
  `(2021, 1)`.
- *`strftime` `%U`, Sunday and seven.* Week 1 needs all seven days, so it
  starts on Sunday 3 January 2021, and 1 January is in the last week of 2020.
  `%U` writes 0 for a day that is not in its own year's numbering:
  `week_of_year_sunday` is 0. `%W` is the same rule from Monday, and is 0 too.
- *Week of month.* October 2026 starts on a Thursday and November 2026 on a
  Sunday. With Monday and four, October's week 1 starts on Monday 28
  September (it holds 1 to 4 October, four days), so 1 and 4 October are
  in week 1 and 5 October in week 2; 1 November is a Sunday, the last day of
  a week that holds one day of November, so it is in week 0 and 2 November
  in week 1. With Sunday and one, 1 November is in week 1 and 4 October in
  week 2.
- *`to_fixed`.* Week 53 of 2021 under ISO is Monday 3 January 2022, the next
  year's week 1; week 53 of 2020 on a Friday is 1 January 2021.

The facade writes the same numbers. `hyper_calendar::zone_lines::format_pattern_line`
of the instant 2021-01-01T12:00:00Z in UTC with the CLDR pattern `w Y W e c`:

| Locale | `w Y W e c` | Rule |
| --- | --- | --- |
| `en-US` | `1 2021 1 6 6` | Sunday and one |
| `de` | `53 2020 0 5 5` | Monday and four |
| `ar` | `1 2021 1 7 7` | Saturday and one |
| `pt-PT` | `53 2020 0 6 6` | Sunday and four |
| `en-US-u-fw-mon` | `1 2021 1 5 5` | Monday and one |

`e` and `c` are the place of the weekday in the locale's week, counted from 1,
which is `position + 1`: a Friday is the sixth day of a week that starts on
Sunday and the seventh of one that starts on Saturday. The `strftime` pattern
`%U %W %V %G` is `00 00 53 2020` for the same instant.

## What is carried

- `WeekRule` for the Gregorian calendar, in `hc-calendar`. `ISO` (Monday, 4),
  `WORLD` (Monday, 1, CLDR's `001`) and `hc_i18n::week::for_locale` for every
  locale of `hc-i18n`.
- The first day of the week of every region CLDR 48 lists, 71 rows of
  `locale::REGION_FIRST_DAY` (Monday for the rest), and `minDays` of 44
  regions, generated into `data::REGION_MIN_DAYS` by `scripts/locales-cldr.py`.
- The pattern letters `w`, `Y`, `W`, `e` and `c` of the CLDR engine, which use
  the locale's rule, or ISO 8601's when no locale is given (the `C` locale is
  POSIX's, and CLDR's root rule, Monday and one, is not what a program that
  names no locale means by a week number); the `strftime` conversions `%U` and
  `%W`, `%V` and `%G` (ISO), `%u` and `%w`; and the CLDR parser's resolution of
  a week year, a week and a weekday to a day through `to_fixed`.
- The boundary: `hc_week_of_year` writes a fixed day's week-numbering
  year, week, weeks in the year and week of the month under any rule given
  as its first day and `minDays`; `hc_locale_info` writes the first day (column 14, an ISO
  weekday number) and `minDays` (column 15) of a locale, and `hc_format_pattern`
  writes the letters above.

Not carried:

- **The week rules of the calendars that are not Gregorian.** `week_of_year`
  and `week_of_month` count the Gregorian year and month; the field `w` in
  another calendar's pattern is the Gregorian week of the same day. No source
  read gives a week rule for a month or year of another calendar. Not yet done.
- **The weekend days.** CLDR's `weekendStart` and `weekendEnd` are not read by
  `hc-i18n`: not yet done. The weekend days this library answers are the laws
  `hc-holiday` carries, each from its statute or notice, in
  `regional-weekends.md`.
- **Which year a week year in an era calendar is.** `Y` writes the Gregorian
  week year, as the signed astronomical year (`-0043` for 44 BCE); UTS #35's
  table does not say which a week year in an era calendar is
  ([date-patterns.md](date-patterns.md)).
- **Weeks of other periods**, such as the retail 4-5-4 and the 52/53-week
  years of `fiscal-calendars.md`, which have their own rules and identifiers.

## Accuracy

The arithmetic is integer arithmetic on fixed days and has no tolerance. It was
measured on 2026-10-04 against four independent things.

- **UTS #35's own example.** The test `the_specs_1998_example_holds` checks
  that Monday and four starts week 1 of 1998 on 29 December 1997, that Sunday
  and four starts it on 4 January 1998 and ends it on the 10th, that the first
  three days of 1998 are in week 53 of 1997 and that 1997 has 53 weeks of that
  rule.
- **ISO 8601.** `WeekRule::ISO.week_of_year` and `hc_calendars_solar::iso_week`
  give the same week-numbering year and week on every one of the 73 414 days
  from 1900-01-01 to 2100-12-31: 0 differences. `iso_week` is
  written from the rule of Wikipedia's "ISO week date"
  [wikipedia-iso-week-date], and its test comment records agreement with
  Python's `date.isocalendar` for 1900 to 2099.
- **Python's `strftime`.** The facade's `%U %W` for `en-US` and Python 3.9.6's
  `date.strftime('%U %W')` give the same pair on the same 73 414 days: 0
  differences.
- **ICU.** `week::for_locale` of `und-XX` for every one of the 676
  two-letter codes `AA` to `ZZ` that `hc-i18n` parses was compared with Node
  22.15.1's `new Intl.Locale('und-XX').weekInfo` (ICU 76.1, CLDR 46): `firstDay`
  and `minimalDays` agree for 661 and differ for 15. `IS` is Sunday in CLDR 48
  (this crate) and Monday in CLDR 46, which the two releases'
  `supplementalData.xml` both say [cldr48-supplemental] [cldr46-supplemental].
  The other fourteen are `AN` and thirteen deprecated or reserved codes (`BU`,
  `DD`, `FX`, `JT`, `MI`, `NT`, `PU`, `PZ`, `RH`, `SU`, `UK`, `WK`, `YD`),
  for which CLDR 48's `weekData` has no row and ICU gives a value of its own. The test `a_locales_rule_is_cldrs_week_data` pins nineteen locales to
  Node's answers.
- **The week of the month** is a comparison and not a source of the rule: UTS
  #35 says the Week of Month is "similarly calculated" and does not say what a
  day before the first week is. ICU4J's `Calendar.weekNumber` returns 0 for it,
  and so does this crate [icu4j-calendar-week-number].

## Sources

Read directly as HTML on 2026-10-04 unless noted.

- [uts35-v48]: UTS #35 Part 4 (Dates), version 48.2, "Week of Year" and "Week
  Elements" (`firstDay`, `minDays`), and the Date Field Symbol Table rows for
  `Y`, `W`, `e` and `c`; read 2026-10-03 and again 2026-10-04.
- [cldr48-supplemental]: CLDR 48 `common/supplemental/supplementalData.xml`,
  `weekData/firstDay` and `weekData/minDays`, tag `release-48`; read 2026-10-04.
- [cldr46-supplemental]: the same file at tag `release-46`, for the
  comparison with Node's ICU; read 2026-10-04.
- [icu4j-calendar-week-number]: ICU4J's `Calendar.weekNumber`, a comparison for
  the week of the month; read 2026-10-03.
- [wikipedia-iso-week-date]: the ISO week date algorithm, a secondary account;
  ISO 8601 itself was not read.
- Node 22.15.1, `Intl.Locale.prototype.weekInfo`, as a measuring tool: it is
  ICU 76.1 with CLDR 46, not a source of the rule.
- POSIX `strftime`'s `%U` and `%W` are described by [posix-strftime-2024]; the
  measured agreement with them is Python's, above, and the page itself is not
  re-read for this document.

## Code

`crates/hc-calendar/src/week.rs` holds `WeekRule`, the arithmetic written
once; `crates/hc-i18n/src/week.rs` reads a locale's rule from CLDR's week data
(`for_locale`) and re-exports it, with `locale.rs` (`region_min_days`,
`region_first_day_of_week`, `REGION_FIRST_DAY`) and `data/cldr48_locales.rs`
(`REGION_MIN_DAYS`, generated by `scripts/locales-cldr.py`); the users are
`crates/hc-format/src/patterns/cldr.rs` (`write_field` for `w`, `Y`, `W`, `e`
and `c`, and `week_rule`), `crates/hc-format/src/patterns.rs`
(`Fields::week_of_year_sunday` and `week_of_year_monday`) and
`crates/hyper-calendar/src/i18n_lines.rs` (`locale_info_line`).

The tests that anchor it are in `crates/hc-calendar/src/week.rs`:
`the_specs_1998_example_holds`, `the_iso_rule_gives_iso_weeks`,
`a_week_date_round_trips` and `the_week_of_the_month_counts_from_its_first_week`;
in `crates/hc-i18n/src/week.rs`: `a_locales_rule_is_cldrs_week_data` and
`a_locale_numbers_its_weeks`; and in
`crates/hc-format/src/patterns.rs` the tests of `%U` and `%W`.
