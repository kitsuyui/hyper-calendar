# Statistical software dates: SAS, Stata and MATLAB

Backs the identifiers `sas-date`, `stata-date`, `stata-week` and
`matlab-datenum` in `hc-calendars-solar`, and the datetimes `sas-datetime`,
`stata-tc` and `stata-tc-utc` in `hc-core::sas_stata`.

## What it is

Statistical and numerical packages store a date as a number, so that the
difference of two dates is a subtraction. SAS and Stata count from
1 January 1960 — days for a date, and for a date and time, seconds in SAS
and milliseconds in Stata [sas-lrcon-dates; stata-help-datetime]. MATLAB
counts days from "January 0, 0000", with the time of day as the fraction,
in its `datenum`, which MathWorks now marks "Not recommended" in favour of
`datetime` [mathworks-datenum]. The numbers are what a data file holds:
reading a SAS or Stata dataset, or a `.mat` file, means reading these
counts.

The two packages that share an epoch do not share a range, and Stata keeps
two datetime encodings, one without leap seconds and one with them. Each
difference is its own identifier here, under [policy.md](../policy.md) §5.

## How it works

**SAS.** A SAS date value is "the number of days between January 1, 1960,
and a specified date", negative before it, and "SAS can perform
calculations on dates ranging from A.D. 1582 to A.D. 19,900". A SAS
datetime is "the number of seconds between January 1, 1960, and an
hour/minute/second within a specified date" [sas-lrcon-dates]. The SAS page
says nothing about leap seconds; Stata's documentation says "SAS ignores
leap seconds" [stata-help-datetime-conversion], and the datetime is read
as a count of 86 400-second days.

**Stata `%td` and `%tc`.** A Stata date is "days since 01jan1960
(01jan1960 = 0)"; datetime/c, displayed with `%tc`, is "milliseconds since
01jan1960 00:00:00.000, assuming 86,400 s/day" [stata-help-datetime]. Both
run from 01jan0100 to 31dec9999, the days −679 350 to 2 936 549
[stata-help-datetime-functions].

**Stata `%tC`.** Datetime/C is the same count "adjusted for leap seconds",
which Stata calls equivalent to UTC: 23:59:60 of a leap second is a valid
time, and "24 seconds have been inserted into datetime/C between 01jan1960
and 23nov2010" [stata-help-datetime-conversion]. Stata counts the leap
seconds "since 1972"; the fractional `TAI − UTC` of 1961–1971 is not part
of the count. So `%tC` = `%tc` + 1 000 ms × the leap seconds inserted
before the instant, which from 1972 is `TAI − UTC − 10 s`.

**Stata `%tw`.** A weekly date counts weeks since 1960w1, and every year
has 52: `week()` returns "integers 1 to 52", "the first week of a year is
the first 7-day period of the year", and the weekly dates run "0100w1 to
9999w52 (integers -96,720 to 418,079)", with 9999w52 beginning on
24dec9999 [stata-help-datetime-functions]. So week *w* begins on day
7(*w* − 1) + 1 of the year, and week 52, which begins on the 358th day,
runs to the year's end: eight days, or nine in a leap year. The weekly
number is 52 × (year − 1960) + *w* − 1.

**MATLAB.** A serial date number is "the whole and fractional number of
days from a fixed, preset date (January 0, 0000) in the proleptic ISO
calendar" [mathworks-datenum]. January 0 is 31 December of the year before,
so 1 January 0000 is day 1, and the number is the Rata Die plus 366, the
days of the leap year 0.

**Worked example.** 20 January 2010, 09:15:22.120 UTC.

1. From 1 January 1960 to 1 January 2010 is 50 years with 13 leap days,
   18 263 days; plus 19 is 18 282, the `%td` and the SAS date.
2. 18 282 × 86 400 000 ms + (9 × 3 600 + 15 × 60 + 22) × 1 000 + 120 =
   1 579 564 800 000 + 33 322 120 = 1 579 598 122 120, the `%tc`.
3. `TAI − UTC` was 34 s, so 24 leap seconds had been inserted since 1972;
   the `%tC` is 24 000 ms more, 1 579 598 146 120. Both are Stata's own
   numbers [stata-help-datetime].
4. 20 January is the year's 20th day, in week ⌊19 / 7⌋ + 1 = 3; the weekly
   date is 52 × 50 + 2 = 2 602.
5. The Rata Die is 733 792, so the `datenum` is 734 158.

## What is carried

- **`sas-date`**, `day_counts::SAS`: day 0 on 1 January 1960, from
  1 January 1582 to 31 December 19 900. SAS states the range in years;
  it is read as the whole of both years.
- **`stata-date`**, `day_counts::STATA`: the same count from 01jan0100 to
  31dec9999, a separate identifier because its range is not SAS's.
- **`stata-week`**, `day_counts::StataWeekCalendar`: year, week 1 to 52 and
  the day of the week counted from its first day, `day-of-stata-week`, 1 to
  7 or to 8 or 9 in week 52; `StataWeekDate::weekly` and `from_weekly` give
  the `%tw` number.
- **`matlab-datenum`**, `day_counts::MATLAB`, unbounded as MathWorks states
  no range, with the fraction in `spreadsheet::matlab_datenum` and
  `to_matlab_datenum`.
- **`sas-datetime`**, `hc_core::sas_stata::sas_datetime`: exact seconds
  from 1960 as a `Duration`, over the SAS range.
- **`stata-tc`**, `stata_tc`: whole milliseconds, the sub-millisecond part
  floored, which cannot name 23:59:60.
- **`stata-tc-utc`**, `stata_tc_utc`, from a `UtcInstant`: a leap second
  is accepted only where the leap-second table has one, and a time past
  the table is refused under `LeapPolicy::Strict`, the refusal Stata's
  manual gives for dates more than six months ahead.
- **Not carried**, with the reason:
  - *Negative MATLAB serials.* MathWorks gives no negative value and does
    not say how its fraction would read, so `matlab_datenum` refuses one
    rather than choose the OLE Automation date's rule or the floor.
  - *Stata's monthly, quarterly, half-yearly and yearly dates*, which name
    a month or a quarter rather than a day, and *business calendars*,
    which are the user's.
  - *Stata's `leapseconds.maint`.* Stata reads its own file of leap
    seconds; this library reads its own table, which is the same data
    wherever both are current.

## Accuracy

| Check | Test | Result |
| --- | --- | --- |
| SAS's `'26oct02'd` is 15 639, and the range ends at −138 061 and 6 552 815 | `sas_dates_count_from_1960_between_1582_and_19900` | exact |
| Stata's 20jan2010, 22jul2010 and 15jun2004, and the range −679 350 to 2 936 549 | `stata_dates_are_the_same_count_over_another_range` | exact |
| `%tc` and `%tC` of 20jan2010 09:15:22.120, both ways | `stata_20_january_2010` | exact |
| Noon of 23nov2010 in both encodings | `stata_noon_23_november_2010` | exact |
| `tc(15jun2004 12:00:00)` and 22jul2010 in `%tc` | `stata_typed_examples` | exact |
| 31dec2005 23:59:60 and 30jun1997 23:59:60 valid in `%tC`, 30dec2005 23:59:60 refused | `only_real_leap_seconds_have_a_tc_utc_value` | exact |
| `tw(1960w2)` = 1; 0100w1 and 9999w52, and their first days; `week` of 5 July 2013 is 27 | `stata_weeks_are_52_from_1_january` | exact |
| MathWorks' seven `datenum` examples, among them 1417 with the pivot year 1400 | `matlab_datenum_is_the_rata_die_plus_366` | exact |
| The SAS and Stata days are JDN − 2 436 935, MATLAB's JDN − 1 721 059 | `the_offsets_from_the_julian_day_number_are_the_published_ones` | exact |

One disagreement in the sources: the table "How Stata dates are stored"
in `help datetime` gives 2010w3 as 2 601. The ranges that
`help datetime_functions` states, 0100w1 = −96 720 and 1000w1 = −49 920,
both follow 52 × (year − 1960) + week − 1, which puts 2 601 at 2010w2;
this library follows the ranges, and 2010w3 is 2 602.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [sas-lrcon-dates] | The SAS date and datetime definitions, the range, the example | Yes, support.sas.com, 2026-09-26 |
| [stata-help-datetime] | `%td`, `%tc`, `%tC` and `%tw`, and the 2010 examples | Yes, stata.com, 2026-09-26 |
| [stata-help-datetime-conversion] | The two encodings, the leap seconds since 1972, SAS ignoring leap seconds | Yes, stata.com, 2026-09-26 |
| [stata-help-datetime-functions] | The ranges, `week`, `dofw`, `tw` | Yes, stata.com, 2026-09-26 |
| [mathworks-datenum] | The MATLAB definition and examples | Through a fetching tool, the site refusing direct requests, 2026-09-26 |

The PDF manuals [D] Datetime and [FN] Date and time functions were not
read beyond the overview; the help files are the same entries' text.
The SAS Global Forum and SUGI papers that give −138 061 as the first SAS
date were found and not read; the number here is derived from the SAS
page's range.

## Code

`crates/hc-calendars-solar/src/day_counts.rs` for the four calendars,
`crates/hc-calendars-solar/src/spreadsheet.rs` for `matlab_datenum`, and
`crates/hc-core/src/sas_stata.rs` for the datetimes. Anchors:
`stata_20_january_2010`, `stata_weeks_are_52_from_1_january`,
`matlab_datenum_is_the_rata_die_plus_366`.
