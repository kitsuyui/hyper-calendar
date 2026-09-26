# The Liberalia Triday Calendar: a solar and a lunar calendar of three-day tridays

Backs the identifiers `liberalia-triday-solar` in `hc-calendars-solar` and
`liberalia-triday-lunar` in `hc-calendars-lunar`.

## What it is

Peter Meyer's "modest proposal for calendar reform" of November 1999,
which replaces the seven-day week with the *triday* — "a day of preparation
followed by a day of active or passive participation followed by a day of
rest and reflection" — and builds two independent calendars of whole
tridays on one epoch, a solar one of quarters and a lunar one of months,
so that a day may be dated in either or both, "like stating a date in both
a sacred and a civil calendar" [meyer-liberalia-triday]. Nobody adopted it.
The epoch is the Roman feast of the Liberalia, 17 March, in 1904, the day
of a central annular eclipse four days before the equinox.

## How it works

**The triday.** Its days are Sophiesday, Zoesday and Norasday. Every
quarter, month and year of both calendars is a whole number of tridays, so
each begins on a Sophiesday and ends on a Norasday
[meyer-liberalia-triday].

**The solar calendar** writes `year-quarter-triday-day SLT`. Its quarters
are Kamaliel, Gabriel, Samlo and Abrasax, of 30, 31, 30 and 31 tridays,
except that Abrasax has 30 when "the solar year number + 1 is divisible by
4 or by 198". A year is 366 days or 363; 396 years are 144 636 days, a mean
year of 365.2424̄ days, "the same as the current length of the vernal
equinox year" [meyer-liberalia-triday]. Years are any integer.

**The lunar calendar** writes `cycle-year-month-triday-day LLT`. A cycle is
384 lunar years of twelve months, Armedon, Nousanios, Harmozel, Phaionios,
Ainios, Oraiel, Mellephaneus, Loios, Davithe, Mousanios, Amethes and
Eleleth, each of ten tridays except Oraiel, of nine, and Eleleth, of nine
"unless the lunar year number minus 2 is divisible by 8, in which case it
has 10 tridays, unless the lunar year number is 2". A year is 354 days or
357, and a cycle 136 077 days, 4 608 months of 29.530599 days on average
[meyer-liberalia-triday].

**The epoch.** `0-000-01-01-1 LLT` and `0-1-01-1 SLT` are both Julian Day
Number 2 416 557, 17 March 1904 [meyer-liberalia-triday].

**Worked example.** Meyer first published the calendar on "Zoesday, 9
Mellephaneus 98, 15 Samlo 95", 1 November 1999. The solar year 96 begins
on 17 March 2000, by his table; 95 + 1 = 96 is divisible by 4, so year 95
is short, 363 days, and began on 20 March 1999. 1 November is 226 days
later; Kamaliel and Gabriel are 90 and 93 days, so it is day 44 of Samlo,
the second day of its fifteenth triday: `95-3-15-2 SLT`, a Zoesday. The
lunar year 98 of cycle 0 begins 98 · 354 + 3 · 11 = 34 725 days after the
epoch — the long years before it are 10, 18, …, 90 — on 13 April 1999, and
1 November is 202 days into it: six months of 177 days leave 25, so it is
the twenty-sixth day of Mellephaneus, the second of its ninth triday,
`0-098-07-09-2 LLT`.

## What is carried

- **`liberalia-triday-solar`**: year, the quarter in the month slot, named
  Kamaliel to Abrasax, and the day of the quarter, 1 to 93; the triday and
  the day of the triday are the `triday` and `day-of-triday` fields, the
  day's name the `triday-day` cycle. `is_leap_year` answers whether the
  year is long, 366 days, since the triday Abrasax drops is the
  calendar's intercalary unit. Years −99 999 to 99 999.
- **`liberalia-triday-lunar`**: the continuous year `384 · cycle + year`,
  the month and the day of the month, with `cycle`, `year-of-cycle`,
  `triday` and `day-of-triday` fields; cycles −260 to 260. The epoch, the
  day names and the triday arithmetic are the solar module's.
- **Not carried:** the triday offset of section 5 of the page, a
  convenience for reckoning one calendar's triday from the other's, which
  follows from the two calendars; Meyer's studies of the vernal equinox and
  the dark moon, which are his measurements of the rules, not rules.
- `usage` is unrecorded: a proposal.

## Accuracy

| Check | Test | Result |
| --- | --- | --- |
| The correlation table of JDN 2 416 554 to 2 416 565, solar and lunar | `the_correlation_table_around_the_epoch` (both modules) | all 24 |
| The table of the solar New Year's Days of 2000 to 2015, with their lunar dates | `the_new_years_of_2000_to_2015`, `the_lunar_dates_of_the_solar_new_years_of_2000_to_2015` | all 32 |
| The dated examples: the two publication days, Hofmann's birthdays, Bicycle Day, the combined dates of the text | `the_dated_examples` (both modules) | all |
| The tables of section 4: 30, 31, 30, 31 tridays and the omitted 31st; ten tridays, nine in Oraiel and the tenth of Eleleth | `the_quarter_table`, `the_month_table` | all |
| 144 636 days in 396 solar years; 136 077 in 384 lunar years, 47 long | `the_cycle_is_144_636_days`, `the_cycle_is_136_077_days` | exact |

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [meyer-liberalia-triday] | The whole definition, the names, the tables, the correlation and the dated examples | Yes, 2026-09-26 |

## Code

`crates/hc-calendars-solar/src/liberalia.rs` and
`crates/hc-calendars-lunar/src/liberalia_lunar.rs`. Anchors:
`the_correlation_table_around_the_epoch`, `the_new_years_of_2000_to_2015`,
`the_dated_examples`.
