# The Jalālī (Malekī) calendar in Ṭūsī's arithmetic

Backs the identifier `jalali-tusi` in `hc-calendars-solar`.

## What it is

The Seljuk sultan Jalāl-al-Dawla Malekšāh had a new solar calendar made,
variously called *tārīḵ-e jalālī*, *malekī*, *solṭānī* and *moḥdaṯ*, whose
purpose, early historians and astronomers agree, was to fix the new year,
Nowrūz, at the vernal equinox. "Calculations based on the many Jalālī
dates recorded by historians and astronomers give the Hejrī date of its
adoption as Friday, 9 Ramażān 471/15 March 1079 (= 19 Farvardīn 448
Yazdegerdī)" [abdollahy1990]. The months kept the Zoroastrian names,
qualified as *jalālī* or *malekī* to tell them from the *qadīmī* months of
the Yazdegerdī year, and the first eighteen days of the Farvardīn in which
the era began were treated as an intercalation [abdollahy1990].

The calendar has two definitions, and they are not the same calendar
(policy §5):

* **As the astronomers defined it.** Nowrūz is "the day on which the sun
  entered Aries before noon", the definition Ṭūsī, Oloḡ Beg and later
  authors give [abdollahy1990]. That is `jalali`, which is Researching in
  [calendars.md](../calendars.md): no source read names the meridian or the
  day a month begins on.
* **As an arithmetic scheme.** The months "were not true solar months but
  consisted of thirty days each", the seasons alone being astronomically
  true, and the astronomers "worked out rules for the sequence of ordinary
  and leap years" [abdollahy1990]. Ḵāzenī's rule in a cycle of 220 years
  was, it seems, abandoned later; Naṣīr-al-Dīn Ṭūsī, at the Marāḡa
  observatory in the thirteenth century, "gives a table in which the
  quadrennia and quinquennia of the first 295 Jalālī years are shown"
  [abdollahy1990]. This document is about that table.

The Zoroastrian communities of Iran that adopted the Jalālī calendar still
kept it into the twentieth century [panaino1990iv].

## How it works

**The year.** Twelve months of thirty days, then five extra days, or six
in a long year. Among the Zoroastrian communities that kept the calendar,
"the 5 or 6 epagomenal days follow the month of Esfandārmoḏ", the twelfth,
"or, in some villages in the district of Naṭanz, the month of Bahman"
[panaino1990iv]. This library places them after the twelfth month.

**The long years.** Ṭūsī's table puts "an extra day ... every four years,
and after seven such quadrennia the extra day was added to a period of
five years. The quinquennial leap years are the Jalālī years 31, 64, 97,
130, 163, 192, 225, 258, and 291" [abdollahy1990]. The gaps between those
years are 33, 33, 33, 33, 29, 33, 33 and 33, so the count of quadrennia is
seven in most periods and six in one; the list, not the "seven", is what
fixes the table. Walking back by fours from 31, the five-year period
before it, the leap years before are 26, 22, …, 6, 2. Iranica gives the
rule that reproduces the whole table: "add 3 to the year in question …
multiply the total by 39 … divide the product by 161; if the remainder is
less than 39, the year was a leap year" [abdollahy1990]. The test
`the_rule_is_tusis_table_for_all_295_years` builds the table from the
list and checks the rule against it for every year: 72 long years in 295.

**Worked example.** Year 26 is long, since 29 × 39 = 1131 and 1131 − 7 ×
161 = 4 < 39; year 27 is not (30 × 39 − 7 × 161 = 43). 1 Farvardīn 1 is
15 March 1079 (Julian); the long years 2, 6, …, 26 are seven in the first
26 years, so 1 Farvardīn 27 is 26 × 365 + 7 = 9 497 days on, 15 March
1105. Years 27 to 30 are short, 31 is the first quinquennial long year, and
1 Farvardīn 32 is 5 × 365 + 1 = 1 826 days after 15 March 1105: 15 March
1110. Over the table the new year slips from 15 March to 12 March Julian,
1 Farvardīn 295 falling on 12 March 1373, since the table's mean year,
365 + 72/295 days, is shorter than the Julian 365¼.

## What is carried

| Identifier | Rule | Range |
| --- | --- | --- |
| `jalali-tusi` | Twelve thirty-day months under [`zoroastrian::MONTHS`](../../crates/hc-calendars-solar/src/zoroastrian.rs), the extra days after the twelfth, long when (*y* + 3)·39 mod 161 < 39 | 1 Farvardīn 1, 15 March 1079, to the last extra day of 295, 20 March 1374 (Gregorian; 12 March Julian) |

**Only the table's years.** The rule is Iranica's fit to Ṭūsī's table, not
a rule anyone is recorded as keeping, and the table ends at 295. The
calendar refuses the years after it — `YearOutOfRange` and
`AfterSupportedRange` — rather than extend the rule under the table's name.
A continuation by the rule would be a calendar of its own name under
policy §5; none is registered.

**Not carried.** The astronomical calendar `jalali`; Ḵāzenī's 220-year
rule, which was not read; the Naṭanz placement of the extra days after
Bahman, which would be its own identifier; the Jalālī day and month names
that Ṭūsī says earlier astronomers introduced, which are in Iranica's
Tables 35 and 36, images that were not read; and the *Zīj-e īl-ḵānī*
itself.

## Accuracy

The arithmetic is exact. What it matches is the published description of
Ṭūsī's table, not the table itself, which was not read.

| Check | Test | Result |
| --- | --- | --- |
| 1 Farvardīn 1 is Friday 15 March 1079 (Julian) | `the_epoch_is_friday_the_fifteenth_of_march_1079` | Holds |
| … and 19 Farvardīn 448 Y.Z. of the Qadimi reckoning, `zoroastrian-qadimi` | the same | Holds |
| The rule is the table the quinquennia describe, for every year 1–295 | `the_rule_is_tusis_table_for_all_295_years` | 295 of 295; 72 long years |
| The extra days follow Esfandārmoḏ, and a long year's sixth is the day before Nowrūz | `the_extra_days_follow_the_twelfth_month` | Holds |
| Every day of the table round-trips | `every_day_of_the_table_round_trips` | 107 747 days |
| The years after the table are refused | `the_years_after_the_table_are_refused` | Holds |

The Hejrī side of the epoch, 9 Ramaḍān 471, is `islamic-civil`'s to check
and is not tested here: the solar crate does not depend on the lunar one.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [abdollahy1990] | The epoch and its Hejrī and Yazdegerdī equivalents; the thirty-day months; Ṭūsī's table of 295 years and its quinquennia; the mod-161 rule; the astronomical definition of Nowrūz | Yes, in the Wayback Machine's copy, 2026-09-26; Tables 35 and 36 are images and were not read |
| [panaino1990iv] | The extra days after Esfandārmoḏ, or after Bahman in Naṭanz, among the Zoroastrian communities that adopted the calendar | Yes, the same copy |
| [wikipedia-jalali-calendar] | The astronomical reading, months by the Sun's entry into the signs | Yes, 2026-09-26 |

Ṭūsī's *Zīj-e īl-ḵānī*, Ḵāzenī's *al-Zīj al-moʿtabar al-sanjarī*, and
Taqizadeh and Abdollahy's own earlier studies, which Iranica cites, were
not read.

## Code

`crates/hc-calendars-solar/src/jalali_tusi.rs`. Anchors:
`the_epoch_is_friday_the_fifteenth_of_march_1079`,
`the_rule_is_tusis_table_for_all_295_years`,
`the_extra_days_follow_the_twelfth_month`,
`every_day_of_the_table_round_trips`,
`the_years_after_the_table_are_refused`.
