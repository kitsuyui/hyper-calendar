# The Babylonian calendar of the Seleucid era

Backs the identifier `babylonian` in `hc-calendars-lunar`.

## What it is

The lunisolar calendar of Babylonia in its regular form, as it ran from the
fourth century BCE until the last dated cuneiform texts of the first century
CE. Months began on the evening the new crescent was first seen from Babylon;
a thirteenth month was added in seven fixed years of every nineteen; the year
began in spring with Nisannu. Years were counted, from the reign of
Seleucus I on, in the Seleucid era, whose first year began on 1 Nisannu =
3 April 311 BCE (Julian) [wikipedia-seleucid-era]. Earlier tablets are dated
by regnal years.

The calendar is the ancestor of the Hebrew calendar's month names and of its
nineteen-year cycle, and the Seleucid count went on in Syriac and Jewish use
for centuries after Babylon — on other calendars, which this document does
not cover.

## How it works

**The cycle.** Of every nineteen Seleucid years, those congruent to 1, 4, 7,
9, 12 and 15 (mod 19) carry a second Addāru after the twelfth month, and the
year congruent to 18 carries a second Ulūlu after the sixth. The closed form
is `(7y + 13) mod 19 < 7` for "year *y* is long", and `y mod 19 = 18` for
"the extra month is Ulūlu II" [reingold2018, reingold2018code]. Before about
380 BCE the king intercalated by decree, and the cycle does not describe
those years [wikipedia-babylonian-calendar].

**The month.** A month begins at the sunset that opens the day after the
crescent was first seen. Reingold and Dershowitz model the sighting with a
*moonlag* criterion at Babylon (32.4794° N, 44.4328° E, 26 m): at sunset,
the Moon is at least 24 hours past conjunction and short of first quarter,
and it sets more than 48 minutes after the Sun [reingold2018code,
`babylonian-criterion`]. Parker and Dubberstein computed their table's month
starts with Schoch's visibility criterion instead [vangent2011], and the two
differ in the way the accuracy section measures.

**Months.** 1 Nīsannu, 2 Ayyāru, 3 Sīmannu, 4 Duʾūzu, 5 Ābu, 6 Ulūlu,
7 Tašrītu, 8 Araḫsamna, 9 Kisilīmu, 10 Ṭebētu, 11 Šabāṭu, 12 Addāru, in the
normalisation van Gent's converter prints [vangent2011]. The intercalary
month repeats the name of the month it follows: Ulūlu II, Addāru II.

**Worked example.** SE 18 is congruent to 18 (mod 19), so it is a long year
with a second Ulūlu. Its months run Nīsannu, Ayyāru, Sīmannu, Duʾūzu, Ābu,
Ulūlu, Ulūlu II, Tašrītu, …, Addāru — thirteen. Parker and Dubberstein's
table puts 1 Ulūlu II of SE 18 on 19 September 294 BCE (Julian), and so
does the criterion; the module's test `the_cited_rows_of_parker_and_dubberstein`
holds that row. SE 19 is congruent to 0 and is a common year of twelve
months.

## What is carried

- **Identifier** `babylonian`, in `hc-calendars-lunar`, with the month as
  `Month { ordinal, leap }`: Ulūlu II is `Month::leap(6)`, Addāru II is
  `Month::leap(12)`, the convention the Hebrew and Chinese calendars use here.
  The day begins at sunset and is named by the civil day it ends on,
  `DayBoundary::Sunset(DayNaming::ByEnd)`: the criterion for a day is
  judged "on eve of" it, at the sunset of the civil day before
  [reingold2018code, `babylonian-criterion`].
- **Range** SE −71 to SE 386: 1 Nīsannu of 383 BCE (RD −139 785) to 29 Addāru
  of 76 CE (RD 27 475). The lower bound is where Parker and Dubberstein's
  table starts following the nineteen-year rule without exception — its last
  intercalation outside the rule is in SE −73 — and the upper bound is where
  the table ends. Outside the range the calendar refuses.
- **Year count** the Seleucid era, continued backwards before SE 1 so that
  year 0 is 312/311 BCE, as van Gent's transcription does. That is a modern
  convention, and the document says so; the regnal labels the earlier tablets
  carry are not carried.
- **Month lengths** as the criterion gives them, each evening judged on its
  own: 29 or 30 days almost always, 31 in 38 months of the range where a
  first evening just clears the lag and the thirtieth after it just misses,
  never 28. The Babylonian practice of ending a month at thirty days when no
  crescent was seen is not imposed, because neither the book nor the table
  imposes it on computed months — the 1971 table prints one lunation of 31
  days [vangent2011] — and imposing it would make each month's start depend
  on the month before it, back to an anchor.
- **Not carried:** the centuries before 383 BCE, which need the table rather
  than a rule; regnal year labels; the Macedonian-month form of the Seleucid
  count, which is a separate row of the roadmap.

## Accuracy

Measured against the 1971 edition of Parker and Dubberstein's table, in van
Gent's transcription [vangent2011], over the 5 664 months from SE −71:

| Measure | Result |
| --- | --- |
| Intercalary months in the year and place the rule gives | 168 of 168 |
| First day of the month on the table's day | 4 694 of 5 664 (82.9%) |
| A day later than the table | 941 (16.6%) |
| A day earlier than the table | 29 (0.5%) |
| Further off than a day | 0 |
| Month lengths | 29 × 2 696, 30 × 2 929, 31 × 38, 28 × 0 |

The disagreement is between 13% and 19% in every fifty-year stretch of the
range, with no trend, so it is the two visibility criteria that differ — a
48-minute moonlag is the stricter test — and not the time scale (ΔT) at
which the sky is computed. The transcription is not vendored, since it is
van Gent's work and Parker and Dubberstein's; the figures above are asserted
by the test `measured_against_parker_dubberstein`, which is ignored unless the
environment variable `HC_PD_TABLE` names a copy converted to one line per
month, `<SE year> <month> <leap 0|1> <fixed day>`. Thirty-eight cited rows —
1 Nīsannu of every nineteenth year from SE −56 and every intercalary month of
the era's first two cycles — are checked in the ordinary tests, and the six
among them that come out a day later are named there.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [reingold2018] | The scheme: cycle, criterion, epoch | Not read directly; the published code was |
| [reingold2018code] | The exact definitions: `babylon`, `babylonian-epoch`, `babylonian-leap-year?`, `babylonian-criterion`, `babylonian-new-month-on-or-before`, `fixed-from-babylonian`, `babylonian-from-fixed`, `moonlag` | Yes, 2026-09-25 |
| [parker1956] | The reference table of month starts, 626 BCE to 75 CE | Not read directly |
| [vangent2011] | The 1971 table as data; the month-name normalisation; the range and the 31-day lunation | Yes, 2026-09-25 |
| [wikipedia-babylonian-calendar] | The regularisation dates (499 BCE, 380 BCE), attributed there to Britton | Yes, 2026-09-25 |
| [wikipedia-seleucid-era] | The epoch, 1 Nisanu = 3 April 311 BC | Yes, 2026-09-25 |

## Code

`crates/hc-calendars-lunar/src/babylonian.rs`. Anchors:
`the_epoch_is_the_third_of_april_311_bce`,
`the_cited_rows_of_parker_and_dubberstein`,
`the_range_is_the_tables_regular_span`; the measurement:
`measured_against_parker_dubberstein`. English month names are in `hc-i18n`,
with `second ` as the leap-month prefix.
