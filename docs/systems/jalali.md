# The Jalālī (Malekī) calendar: the astronomers' rule and Ṭūsī's arithmetic

Backs the identifier `jalali-tusi` in `hc-calendars-solar`, and `jalali`
and `jalali-natanz` in `hc-calendars-equinox`.

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
  authors give [abdollahy1990]; Qoṭb-al-Dīn Širāzī's is "the first day in
  which, at solar noon [...the moment when the sun contacts the observer's
  meridian], the sun is in Aries" [karamati2014], and Persian Wikipedia's
  the day the Sun has reached the equinox "by the time of its transit of the
  local meridian" [fawiki-gahshomari-jalali]. The months "consisted of
  thirty days each" [abdollahy1990]. That is `jalali`, below.
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
before it, the leap years before are 26, 22, …, 6, 2; year 2 being long is
this library's inference from the list and from the rule below, not
something the article states. Iranica gives the rule that reproduces the
whole table: "add 3 to the year in question …
multiply the total by 39 … divide the product by 161; if the remainder is
less than 39, the year was a leap year" [abdollahy1990]. The test
`the_rule_is_tusis_table_for_all_295_years` builds the table from the
list and checks the rule against it for every year: 72 long years in 295.

**Iranica's total is half a day less.** The same article counts the 295
years as "295 x 365 + 286 x 1/4 days", a quarter-day intercalated 295 − 9
= 286 times [abdollahy1990], which is 71½ extra days, not 72. A table of
whole days cannot hold the half: the rule and the walk back from 31 make
year 2 long and give 72, while 71 would follow if the first period of the
table were counted from year 1 so that year 2 was not long. Ṭūsī's table
itself would settle it, and it was not read. Until it is, the calendar
follows the rule, and every date from 1 Farvardīn 3 on rests on that
choice.

**Worked example.** Year 26 is long, since 29 × 39 = 1131 and 1131 − 7 ×
161 = 4 < 39; year 27 is not (30 × 39 − 7 × 161 = 43). 1 Farvardīn 1 is
15 March 1079 (Julian); the long years 2, 6, …, 26 are seven in the first
26 years, so 1 Farvardīn 27 is 26 × 365 + 7 = 9 497 days on, 15 March
1105. Years 27 to 30 are short, 31 is the first quinquennial long year, and
1 Farvardīn 32 is 5 × 365 + 1 = 1 826 days after 15 March 1105: 15 March
1110. Over the table the new year slips from 15 March to 12 March Julian,
1 Farvardīn 295 falling on 12 March 1373, since the table's mean year,
365 + 72/295 days by the rule (365 + 71.5/295 by Iranica's total), is
shorter than the Julian 365¼.

**The astronomers' calendar.** The noon is the Sun's at the observer's
meridian, and no source read names a standard one: Ṭabarī's *Zīj-e mofrad*
"considers the place of observations to be Isfahan", the Seljuk capital
[karamati2014], and English Wikipedia says "Khayyam ... positioned Isfahan
as the prime meridian", citing Amanat (2017), not read
[wikipedia-jalali-calendar]. `jalali` takes the Sun's transit at Isfahan,
51.670° E [wikipedia-isfahan]: Nowrūz is the day of the March equinox if the
equinox falls before Isfahan's apparent noon, and the day after otherwise,
the rule `persian-apparent-noon` applies at Tehran. Twelve months of thirty
days follow, then the extra days, five or six as the next Nowrūz falls.
Year *y* is the Solar Hijri year *y* + 457, Persian Wikipedia making 1
Jalālī "۴۵۸ هجری خورشیدی" [fawiki-gahshomari-jalali].

*Worked example.* The equinox of 1079 fell about six hours before
Isfahan's noon of 15 March (Julian), 367 minutes by the model, so Nowrūz 1
is Friday 15 March 1079, Abdollahy's epoch [abdollahy1990]. 1080 is a
Julian leap year, and its equinox falls twelve minutes before Isfahan's
noon of 14 March, so year 2 begins on 14 March 1080, 365 days after year
1. The equinox of 1081 falls five and a half hours after the noon of
14 March, so year 3 begins on 15 March 1081, and year 2 has 366 days: the
long year 2 the table's rule infers, above.

**Naṭanz.** Among the Zoroastrian communities of Iran that adopted the
Jalālī calendar, "the 5 or 6 epagomenal days follow the month of
Esfandārmoḏ or, in some villages in the district of Naṭanz, the month of
Bahman" [panaino1990iv]; Panaino's pre-Islamic part says the same, "in the
district of Natanz ... the epagomenal days are still inserted after the
eleventh month, Bahman" [panaino1990], and at Abyāna, a village of the
district, the five days "are added ... to the end of Bahman, the eleventh
month, and not to the end of the twelfth month", as Yarshater saw in 1969
[yarshater1983-abyana]. `jalali-natanz` is that placement on `jalali`'s
years: Farvardīn to Bahman, the extra days, then Esfandārmoḏ. No source
read gives the villages' own reckoning of a long year, so the model's
Nowrūz stands in for it; Taqizadeh (1952, p. 610) and Lambton (in Hartner,
1971), whom Panaino cites, were not read.

## What is carried

| Identifier | Rule | Range |
| --- | --- | --- |
| `jalali` | Nowrūz the day the equinox falls before the Sun's noon at Isfahan, else the next; twelve thirty-day months under [`zoroastrian::MONTHS`](../../crates/hc-calendars-solar/src/zoroastrian.rs), then five or six extra days as month 13 | 15 March 1079 (Julian) to the day before Nowrūz of 3001 |
| `jalali-natanz` | `jalali`'s years, the extra days after Bahman, month 11, and before Esfandārmoḏ | the same |
| `jalali-tusi` | Twelve thirty-day months under [`zoroastrian::MONTHS`](../../crates/hc-calendars-solar/src/zoroastrian.rs), the extra days after the twelfth, long when (*y* + 3)·39 mod 161 < 39 | 1 Farvardīn 1, 15 March 1079, to the last extra day of 295, 20 March 1374 (Gregorian; 12 March Julian) |

**Only the table's years.** The rule is Iranica's fit to Ṭūsī's table, not
a rule anyone is recorded as keeping, and the table ends at 295. The
calendar refuses the years after it — `YearOutOfRange` and
`AfterSupportedRange` — rather than extend the rule under the table's name.
A continuation by the rule would be a calendar of its own name under
policy §5; none is registered.

**Not carried.** Ḵāzenī's 220-year rule, which was not read; the reading
in which the months began with the Sun's entry into the signs, which
Abdollahy calls the mistake of "some people" and English Wikipedia states
uncited, with month lengths of 1302 and 1303 that are the Solar Hijri
calendar of 1911–1925, `persian-burj`'s row; the month lengths Persian
Wikipedia reports from the *Zīj-e Sanjarī* through Taqizadeh and Dehkhoda,
neither read; Ṭūsī's table on the Naṭanz placement, which nobody is
recorded as keeping; the Jalālī day and month names
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
| `jalali`: Nowrūz 1 is Friday 15 March 1079 (Julian), `jalali-tusi`'s epoch, 367 minutes before the noon | `the_epoch_is_friday_the_fifteenth_of_march_1079` (`crates/hc-calendars-equinox/src/jalali.rs`) | Holds |
| … against Ṭūsī's table over its 295 years: the same Nowrūz in 285, a day later in 10, never earlier; 72 long years in both, year 2 among them; no year within the minute's tolerance, the first such year being 1013 | `nowruz_follows_the_equinox_and_the_table_is_close_to_the_sky` | Holds |
| `jalali-natanz`: Farvardīn to Bahman as in `jalali`, the extra days from day 331, Esfandārmoḏ after them | `the_extra_days_close_the_year_or_follow_bahman` | Holds |
| Both round-trip four years from the epoch and a sample of the range | `every_day_round_trips_in_both` | Holds |

The Hejrī side of the epoch, 9 Ramaḍān 471, is `islamic-civil`'s to check
and is not tested here: the solar crate does not depend on the lunar one.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [abdollahy1990] | The epoch and its Hejrī and Yazdegerdī equivalents; the thirty-day months; Ṭūsī's table of 295 years and its quinquennia; the 295 × 365 + 286 × ¼ days of the table; the mod-161 rule; the astronomical definition of Nowrūz | Yes, in the Wayback Machine's copy, 2026-09-26; the total of 286 quarter-days in a search engine's extract of the article, 2026-09-27, the live page refusing automated access; Tables 35 and 36 are images and were not read |
| [panaino1990iv] | The extra days after Esfandārmoḏ, or after Bahman in Naṭanz, among the Zoroastrian communities that adopted the calendar | Yes, the same copy; re-read 2026-09-29 |
| [panaino1990] | "In the district of Natanz ... still inserted after the eleventh month, Bahman" | Yes, 2026-09-29, the same copy |
| [karamati2014] | Qoṭb-al-Dīn Širāzī's definition at solar noon, the observer's meridian; Isfahan as the place of observations in Ṭabarī's *Zīj-e mofrad* | Yes, 2026-09-29, in the Wayback Machine's copy, the live page refusing |
| [fawiki-gahshomari-jalali] | The local meridian; the epoch as Solar Hijri 458; thirty-day months and five extra days after the Geophysics Institute; the *Zīj-e Sanjarī*'s month lengths, reported | Yes, 2026-09-29, secondary |
| [yarshater1983-abyana] | The five days after Bahman at Abyāna, 1969 | Yes, 2026-09-29, in the Wayback Machine's copy |
| [wikipedia-isfahan] | Isfahan's coordinates | Yes, 2026-09-29 |
| [wikipedia-jalali-calendar] | The astronomical reading, months by the Sun's entry into the signs | Yes, 2026-09-26 |

Ṭūsī's *Zīj-e īl-ḵānī*, Ḵāzenī's *al-Zīj al-moʿtabar al-sanjarī*, and
Taqizadeh and Abdollahy's own earlier studies, which Iranica cites, were
not read.

## Code

`crates/hc-calendars-solar/src/jalali_tusi.rs` and
`crates/hc-calendars-equinox/src/jalali.rs`. Anchors:
`the_epoch_is_friday_the_fifteenth_of_march_1079`,
`the_rule_is_tusis_table_for_all_295_years`,
`the_extra_days_follow_the_twelfth_month`,
`every_day_of_the_table_round_trips`,
`the_years_after_the_table_are_refused`;
`the_epoch_is_friday_the_fifteenth_of_march_1079`,
`nowruz_follows_the_equinox_and_the_table_is_close_to_the_sky`,
`the_extra_days_close_the_year_or_follow_bahman`.
