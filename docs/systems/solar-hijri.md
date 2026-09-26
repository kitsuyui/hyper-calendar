# The Solar Hijri calendar: the equinox, two readings of noon, two cycles, and Afghanistan

Backs the identifiers `persian`, `persian-apparent-noon` and
`persian-afghan` in `hc-calendars-equinox`, and `persian-arithmetic` and
`persian-arithmetic-33` in `hc-calendars-solar`.

## What it is

The civil calendar of Iran. Its year is solar and begins at Nowruz, the
day of the March equinox; it has six months of 31 days, five of 30 and a
last of 29 or 30; and its years are counted from the Hijra, 1 Farvardin 1
falling in March 622. It descends from the Jalālī calendar of the reform
led by Omar Khayyām, dated to the vernal equinox of A.D. 1079
[heydari-malayeri2004, §1]. The Iranian parliament adopted its principles
on 31 March 1925, 11 Farvardin 1304 [heydari-malayeri2004, §1;
wikipedia-solar-hijri-calendar]. The text of that law was not read here.

Afghanistan has kept the same calendar under the Arabic names of the
zodiac signs since 1301 SH (1922), with the month lengths of Iran's only
from 1336 SH (1957); from 29 July 2022 official correspondence has been
dated by the lunar Hijri year, while the solar year still dates the fiscal
year and the solar holidays [balland1990; rasa2022; rukhshana2022;
ariana2022]. That history, and the question of whose noon Afghanistan
applies, are stated in `persian_afghan`'s module documentation, which
carries its own sources; this document does not repeat them.

## How it works

**The year.** Nowruz, 1 Farvardin, is the day of the March equinox if the
equinox falls before noon, and the next day if it falls after; "the year
begins at the midnight closest to the instant of equinox"
[heydari-malayeri2004, §2]. A year is therefore 365 or 366 days, and the
thirtieth of Esfand exists in the long ones.

**Which noon.** The sources read disagree about the clock:

| Reading | Stated by | Noon in March, Universal Time | Identifier |
| --- | --- | --- | --- |
| The clock's noon: 12:00 Iran Standard Time, UTC+03:30, the mean time of the 52.5° E meridian | [wikipedia-solar-hijri-calendar], "noon (Tehran time)" | 08:30 | `persian` |
| The Sun's noon: its transit of the meridian of Tehran, 51.42° E | [heydari-malayeri2004, §2], "midnight (Tehran true time)"; [reingold2018code], `midday-in-tehran`, "true noon … in Tehran" | about 08:41.7 | `persian-apparent-noon` |

The second is about twelve minutes later than the first: four minutes for
the 1.08° by which Tehran lies west of 52.5° E, and seven or eight for the
equation of time, which in March holds the Sun behind the clock. The
readings name different days only when the equinox falls between the two
noons. No source read says which noon Iran's calendar authority applies,
so both are carried, as policy §5 requires.

**The months.** Farvardin, Ordibehesht, Khordad, Tir, Mordad and Shahrivar
of 31 days; Mehr, Aban, Azar, Dey and Bahman of 30; Esfand of 29, or 30 in
a long year.

**Worked example: Nowruz 1404.** The March equinox of 2025 fell at 09:01.5
UT on 20 March, as `hc-astro` places it. That is 12:31.5 Iran Standard Time: after the clock's noon at
08:30 UT, so 20 March is 30 Esfand 1403 and Nowruz 1404 is 21 March. The
Sun crossed Tehran's meridian at 08:41.7 UT that day; the equinox came
after that too, so the second reading agrees. And because 1403 began on
20 March 2024 — the equinox that year fell at 03:06.5 UT, before either
noon — 1403 had 366 days, and Esfand 1403 its thirtieth day. This is the
year the 2 820-year cycle gets wrong (below).

**Worked example: the first year the readings part.** In 2091 the
equinox falls at 08:41.56 UT on 20 March, and the Sun crosses Tehran's
meridian at 08:41.71 UT, nine seconds later. By the clock the equinox came
after noon, so Nowruz 1470 is 21 March; by the Sun it came before, so it is
20 March. Nine seconds is inside what the model can place, so the
apparent-noon calendar reports 1470 as decided by the model.

**Two cycles.** Two arithmetic rules have been published to reproduce the
equinox without an ephemeris:

- *The 33-year rule.* Eight leap years in thirty-three, those leaving a
  remainder of 1, 5, 9, 13, 17, 22, 26 or 30 on division by 33
  [heydari-malayeri2004, §§5, 8]: the first leap year of the cycle after
  four common ones, then one every fourth year. Its mean year, 365 + 8⁄33 =
  365.2424… days, is the interval from one March equinox to the next
  (§3–4). Heydari-Malayeri reports Borkowski's finding that the rule is
  valid from A.P. 1178 to 1634 [borkowski1996, not read]. Example: A.P.
  1375 leaves 22 and is leap; it began on 20 March 1996
  [heydari-malayeri2004, §8].
- *The 2 820-year cycle* of Behruz and Birashk [behruz1952, birashk1993,
  neither read], 683 leap years in 2 820 arranged in 21 subcycles of 128
  years and one of 132 [heydari-malayeri2004, §7], in the form Reingold
  and Dershowitz give as `arithmetic-persian-leap-year?`: a year is leap
  when ((*y* − 474) mod 2 820 + 474 + 38) × 31 mod 128 < 31
  [reingold2018code]. Its mean year is the mean tropical year, 365.24219858
  days, which is not the equinox-to-equinox year, and Heydari-Malayeri
  calls the cycle erroneous (§7).

Worked by hand for 1403 and 1404: 1403 mod 33 = 17 and 1404 mod 33 = 18,
so the 33-year rule makes 1403 leap, as the equinox did. For the 2 820-year
cycle, (1403 + 38) × 31 = 44 671, which leaves 127 on division by 128, so
1403 is common; (1404 + 38) × 31 = 44 702 leaves 30, so 1404 is leap. The
cycle therefore puts Nowruz 1404 on 20 March 2025, a day early.

## What is carried

- **`persian`** (`hc-calendars-equinox`, CLDR `persian`): the equinox from
  `hc-astro` against 08:30 UT. Years 1 to 2379 (Gregorian 3000). In use from
  31 March 1925, proleptic before.
- **`persian-apparent-noon`** (`hc-calendars-equinox`): the equinox against
  the Sun's transit at Tehran as Reingold and Dershowitz place it, 35.68° N,
  51.42° E ([reingold2018code], `tehran`). The same range and period of
  use: the calendar of the same law, under the other reading.
- **`persian-afghan`** (`hc-calendars-equinox`): `persian`'s days under the
  Dari month names; its module states its period of use and sources.
- **`persian-arithmetic`** (`hc-calendars-solar`): the 2 820-year cycle,
  years 1 to 9999, epoch 1 Farvardin 1 = 19 March 622 Julian
  ([reingold2018code], `persian-epoch`). No period of use: no authority
  promulgated it. The identifier is the name Reingold and Dershowitz give
  the scheme, `arithmetic-persian`.
- **`persian-arithmetic-33`** (`hc-calendars-solar`): the 33-year rule,
  years 1 to 9999. Counting the rule back from its modern years puts
  1 Farvardin 1 on 18 March 622 Julian, a day before the 2 820-year
  cycle's epoch. No period of use.
- The five share the date type, the era code `ap` and the month names in
  Persian script.
- **Not carried:**
  - *Borkowski's full algorithm*, which reconstructs the leap years over
    about 3 000 years with break years between runs of 33-year cycles
    [heydari-malayeri2004, §8]: the paper was not read.
  - *A noon at Kabul* for `persian-afghan`: no Afghan source read states
    one; see that module.
  - *The Zoroastrian Bastani calendar* on the same year, which is a naming
    and is `zoroastrian.md`'s to describe.

## Accuracy

| Check | Test | Result |
| --- | --- | --- |
| The leap years 1354–1419 of Wikipedia's correspondence table, and ten published Nowruzes, reproduced by `persian` | `persian::tests::the_leap_years_are_the_ones_the_correspondence_table_marks`, `nowruz_lands_where_iran_put_it` | all |
| The same Nowruzes, and 1375 on 20 March 1996, by `persian-apparent-noon` | `persian_apparent_noon::tests::nowruz_lands_where_iran_put_it` | all |
| The two readings of noon agree on every Nowruz from 1178 to 1469, and differ in exactly twenty years of 1–2379: 166, 426, 492, 525, 686, 719, 752, 785, 1078, 1111, 1144, 1177, 1470, 1503, 1536, 1602, 1701, 2027, 2093, 2159 | `the_two_noons_give_the_same_days_from_1178_to_1469`, `the_twenty_years_the_two_noons_part_company` | measured |
| The 33-year rule agrees with `persian` on every Nowruz from 1178 to 1634, and not in 1177 or 1635 — Borkowski's range as reported | `persian::tests::the_33_year_rule_agrees_from_1178_to_1634` | 457 of 457 |
| The 33-year rule reproduces the correspondence table's leap years and Heydari-Malayeri's 1375 | `persian_33::tests::the_leap_years_are_the_ones_the_correspondence_table_marks`, `heydari_malayeris_example_year_is_leap` | all |
| The 2 820-year cycle misses Nowruz 1404 | `birashks_cycle_and_the_equinox_part_company_in_1404` | as stated |

Measured, not published: within 1178–1634 the 2 820-year cycle and
`persian` disagree first at 1210 and 1243 and then from 1404 on, in 1404,
1437, 1470, 1503, 1532, 1536 and more; nearer the present only 1404.

Years the model cannot call: an equinox within a minute of the deciding
noon. For `persian` that is 1309 (21 March 1930, about ten seconds before
08:30 UT), for `persian-apparent-noon` 1470. Each module's
`new_year_margin` gives the distance for any year, and the tests name both
years rather than claim them.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [heydari-malayeri2004] | The rule and its midnight in Tehran true time (§2); the 1925 adoption (§1); the 33-year cycle and its remainders (§§5, 8); 1375 as a leap year from 20 March 1996 (§8); the 2 820-year cycle's structure and the case against it (§7); Borkowski's range (§8) | Yes, 2026-09-26, arXiv:astro-ph/0409620v2 |
| [reingold2018code] | `midday-in-tehran`, `tehran`, `persian-new-year-on-or-before`, `persian-epoch`, `arithmetic-persian-leap-year?` | Yes, 2026-09-26, `calendar.l` |
| [reingold2018] | The book's chapter on the Persian calendar, which the code implements | Not read |
| [wikipedia-solar-hijri-calendar] | The standard-time reading, "noon (Tehran time)"; the correspondence table 1354–1419 the tests use; the 1925 law | Read by the modules' author, 2026-09-22 and 2026-09-26; not re-read |
| [borkowski1996] | The 33-year rule's validity, 1178–1634, as Heydari-Malayeri reports it | Not read; the author's page was unreachable on 2026-09-26 |
| [birashk1993] | The 2 820-year cycle | Not read; cited by Heydari-Malayeri |
| [behruz1952] | The 2 820-year cycle's first proposal | Not read; cited by Heydari-Malayeri |
| [balland1990], [rasa2022], [rukhshana2022], [ariana2022] | Afghanistan's adoption, month lengths and 2022 change | Read by the `persian_afghan` author, 2026-09-26; not re-read |

The law of 11 Farvardin 1304, and a statement by Iran's calendar
authority of which noon it applies, are the primary sources that would
settle the choice between the two readings; neither was read.

## Code

`crates/hc-calendars-equinox/src/persian.rs` (`persian`, and the noon
rules shared with the next), `persian_apparent_noon.rs`, `persian_afghan.rs`,
`places.rs` (`TEHRAN_PERSIAN`, `IRAN_STANDARD_OFFSET_DAYS`);
`crates/hc-calendars-solar/src/persian.rs` (`persian-arithmetic`, the date
type and `MONTHS`) and `persian_33.rs`. Anchors:
`nowruz_lands_where_iran_put_it` in both equinox modules,
`the_twenty_years_the_two_noons_part_company`,
`the_33_year_rule_agrees_from_1178_to_1634`,
`heydari_malayeris_example_year_is_leap`,
`birashks_cycle_and_the_equinox_part_company_in_1404`.
