# The Babylonian calendar of the Seleucid era

Backs the identifier `babylonian` in `hc-calendars-lunar`.

## What it is

The lunisolar calendar of Babylonia from the accession year of Nabopolassar,
626 BCE, where Parker and Dubberstein's table begins, until the last dated
cuneiform texts of the first century CE: in its regular form from the fourth
century BCE, and with the intercalations the king decreed before. Months began on the evening the new crescent was first seen from Babylon;
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
those years [wikipedia-babylonian-calendar]: Parker and Dubberstein's table
does, year by year, with 90 thirteenth months in the 243 years from SE −314
to SE −72 — 67 second Addārus and 23 second Ulūlus — the last outside the
rule in SE −73, and SE −72 common where the rule would make it long
[vangent2011].

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
- **Range** SE −314 to SE 386: 1 Nīsannu of 626 BCE (5 April, RD −228 554)
  to 29 Addāru of 76 CE (RD 27 475), the whole of Parker and Dubberstein's
  table. From SE −71, 383 BCE (RD −139 785), where the table starts
  following the nineteen-year rule without exception, the rule places the
  thirteenth month; before, `INTERCALATIONS_BEFORE_THE_RULE`, the table's 90
  intercalations, transcribed from van Gent's copy. The months are the same
  criterion's throughout. Outside the range the calendar refuses.
- **Year count** the Seleucid era, continued backwards before SE 1 so that
  year 0 is 312/311 BCE, as van Gent's transcription does. That is a modern
  convention, and the document says so.
- **Regnal labels** `babylonian::regnal_year`, the king and regnal year van
  Gent's converter labels each Seleucid year with, from its `REIGNS`: 1
  Interregnum for SE −314, the accession year of Nabopolassar, then 1
  Nabopolassar for SE −313 and so on to 11 Demetrius I for SE 160, the last
  year the converter labels. A year belongs wholly to one reign, the regnal
  year running from 1 Nisannu, so the year in which a king died is his, and
  his successor's first year is the next; Alexander III's labels begin at 7
  (SE −18) and Philip III's and Alexander IV's at 2, their first years
  carrying their predecessors' last, as the converter's offsets have it.
  The converter has no Labashi-Marduk, Bardiya, Nebuchadnezzar III or IV,
  or Antigonus, whose reigns fall inside years it gives to others. This is
  van Gent's labelling, a secondary source for Parker and Dubberstein's;
  their own table, which dates accessions to the day, was not read, and a
  label is a year's, not a day's.
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
| The 33 sample dates of *Calendrical Calculations*, 586 BCE to 2094, as its published code computes them [reingold2018code, `dates.l`]: `babylonian` | 3 of the 3 in its range (−625 to 76), 586 BCE among them, whose year the rule and the table both make common; the other 30 refused, in `every_sample_date_agrees_or_is_refused_or_is_a_known_difference` (`crates/hyper-calendar/tests/rd_sample_dates.rs`) |
| Intercalary months in the year and place the rule gives | 168 of 168 |
| First day of the month on the table's day | 4 694 of 5 664 (82.9%) |
| A day later than the table | 941 (16.6%) |
| A day earlier than the table | 29 (0.5%) |
| Further off than a day | 0 |
| Month lengths | 29 × 2 696, 30 × 2 929, 31 × 38, 28 × 0 |
| Every day of the range converts back to itself (`the_calendar_round_trips_every_day_of_its_range`) | All 256 030 days in a release build. A debug build takes every 776th day, 40 days at each end, and every 1 Nisanu with the day before it |

Before the rule, over the 3 006 months from SE −314 to SE −72, with the
table's own intercalations
(`PARKER_DUBBERSTEIN_AGREEMENT_BEFORE_THE_RULE`, the same ignored test):

| Measure | Result |
| --- | --- |
| Intercalary months in the table's year and place | 90 of 90, every year of the 243 walked month by month in a release build (`the_years_before_the_rule_intercalate_as_the_table_does`), every 37th in a debug build |
| First day of the month on the table's day | 2 525 of 3 006 (84.0%) |
| A day later than the table | 465 |
| A day earlier than the table | 14 |
| Two days off | 2: 1 Tašrītu of SE −288, later here, and 1 Kisilīmu of SE −200, earlier, after the table's one lunation of 31 days before the rule |
| Month lengths | 29 × 1 428, 30 × 1 559, 31 × 18, 28 × 0 |

The regnal labels are checked against the dated examples that pair one with
a Julian day (`the_regnal_labels_are_the_converters`): "13 Ulūlū in the 5th
year of Darius III [20 September 331 BCE]", the eclipse of BM 36390
[vangent2011], which the criterion places a day later, its 1 Ulūlu being a
day after the table's; the Babylonian Chronicle's "second day of the month
of Adar [16 March]" in Nebuchadnezzar's seventh year, 597 BCE, on the day
[wikipedia-siege-of-jerusalem-597]; and 568 BCE as his thirty-seventh year,
from VAT 4956 [wikipedia-criticism-jw-vat4956].

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
| [vangent2011] | The 1971 table as data, its intercalations before the rule among them; the month-name normalisation; the range and the 31-day lunation; the regnal labels (`babylon_ruler_name`, `babylon_ruler_year` and the offsets of `babycal.js`); the Darius III eclipse | Yes, 2026-09-25; the regnal data and the converter's code re-read 2026-09-29 |
| [wikipedia-siege-of-jerusalem-597] | The Chronicle's 2 Addāru, 16 March 597 BCE, in Nebuchadnezzar's seventh year | Yes, 2026-09-29, secondary |
| [wikipedia-criticism-jw-vat4956] | 568 BCE as Nebuchadnezzar's thirty-seventh year, from VAT 4956 | Yes, 2026-09-29, secondary |
| [wikipedia-babylonian-calendar] | The regularisation dates (499 BCE, 380 BCE), attributed there to Britton | Yes, 2026-09-25 |
| [wikipedia-seleucid-era] | The epoch, 1 Nisanu = 3 April 311 BC | Yes, 2026-09-25 |

## Code

`crates/hc-calendars-lunar/src/babylonian.rs`. Anchors:
`the_epoch_is_the_third_of_april_311_bce`,
`the_cited_rows_of_parker_and_dubberstein`,
`the_range_is_the_tables_regular_span`,
`the_calendar_round_trips_every_day_of_its_range`, which the Arsacid era
in `crates/hc-calendars-regional/src/arsacid.rs` also relies on for its
own days ([seleucid-eras.md](seleucid-eras.md)); the measurement:
`measured_against_parker_dubberstein`. English month names are in `hc-i18n`,
with `second ` as the leap-month prefix.
