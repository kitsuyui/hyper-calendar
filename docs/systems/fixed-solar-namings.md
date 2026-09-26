# The Bangladeshi and Nanakshahi calendars: fixed namings of the Gregorian day under decree

Backs the identifiers `bangladeshi` and `nanakshahi` in
`hc-calendars-solar`.

## What it is

Two calendars of the subcontinent that took the months of a sidereal
solar reckoning and fixed their lengths, so that every month opens on the
same Gregorian date every year. Each was set by a committee, adopted by an
authority and revised later, which is why one identifier covers more than
one set of rules and why each needs its period stated.

**Bangladeshi.** The Bengali solar year of Bangladesh, *Bangabda*, whose
zero year begins in 593/594 CE [wikipedia-bengali-calendars]. In 1966 a
committee headed by Muhammad Shahidullah was appointed in East Pakistan
to reform the traditional Bengali calendar; it proposed the first five
months of 31 days and the rest of 30, Falgun of 31 in every leap year.
Bangladesh adopted the calendar officially in 1987. The government planned
a further change in 2018, which went into effect on Wednesday 16 October
2019, so that Kartik began on Thursday 17 October 2019
[wikipedia-bengali-calendars, citing Banglapedia, Samakal of 14 April 2019
and Prothom Alo of 17 October 2019, none read here].

**Nanakshahi.** The Sikh calendar designed by Pal Singh Purewal, counted
from the birth of Guru Nanak in 1469. A Calendar Reform Committee met at
the Institute of Sikh Studies, Chandigarh, in 1995 and submitted a
proposal to the Shiromani Gurdwara Parbandhak Committee (SGPC) in 1996;
the SGPC launched the calendar on 14 April 2003, with the approval of the
Akal Takht. In 2010 the SGPC modified it so that the month starts move
with the Bikrami calendar and several observances follow the lunar
phase; its opponents say the SGPC reverted to the Bikrami calendar. The
2003 form is called the *Mool* (original) Nanakshahi calendar by its
supporters, a term Purewal introduced in 2017 [wikipedia-nanakshahi-calendar].

## How it works

**Bangladeshi.** The year *N* begins on 14 April of Gregorian year
*N* + 593. Month lengths:

| Month | Before 1426 | From 1426 | Begins (from 1426) |
| --- | --- | --- | --- |
| Boishakh, Joishtho, Asharh, Srabon, Bhadro | 31 | 31 | 14 April, 15 May, 15 June, 16 July, 16 August |
| Ashvin | 30 | 31 | 16 September |
| Kartik, Ogrohayon, Poush, Magh | 30 | 30 | 17 October, 16 November, 16 December, 15 January |
| Falgun | 30, 31 in a leap year | 29, 30 in a leap year | 14 February |
| Choitro | 30 | 30 | 15 March |

The sources say Falgun gains its day "in every leap year" without saying
which; the calendar takes the Gregorian leap year of the February Falgun
spans, the one reading under which the months open on fixed Gregorian
dates.

**Nanakshahi (2003).** Year *N* begins on 14 March of Gregorian year
*N* + 1468. Chet, Vaisakh, Jeth, Harh and Sawan have 31 days; Bhadon, Assu,
Kattak, Maghar, Poh and Magh 30; Phaggan 30, or 31 when the Gregorian year
it ends in is a leap year, so that it holds 29 February
[wikipedia-nanakshahi-calendar].

**Worked example.** What are 16 December 2019 and 16 December 2025 in the
Bangladeshi calendar? Both are after 14 April, so the years are
2019 − 593 = 1426 and 2025 − 593 = 1432, both under the revision. From
14 April, Boishakh to Bhadro are 5 × 31 = 155 days and Ashvin 31, Kartik
and Ogrohayon 60: 246 days, so Poush 1 is 14 April + 246 days =
16 December. Both are 1 Poush, the Bengali date the source gives for
16 December. Under the 1987
lengths Ashvin had 30 days, and 16 December 2018 was 2 Poush 1425.

The same for the Nanakshahi: 14 April 2026 is 14 March + 31 days, so
1 Vaisakh 558 (2026 − 1468 = 558).

## What is carried

- **`bangladeshi`**: years 1 to 9 999; the 1987 lengths up to 1425 and
  the revision's from 1426, whose first five months are the same under
  both and whose Ashvin is the revision's. In use from 14 April 1987, the
  Pohela Boishakh of the year of adoption, since the source gives the year
  only; the years before are the 1966 calendar's, proleptic.
- **`nanakshahi`**: the 2003 calendar, years 1 to 9 999, in use from
  14 April 2003 to 13 March 2010, the last day of Nanakshahi 541, the
  year before the modification of 2010, whose day the source does not
  give.
- **Not carried:**
  - *The Bengali calendar of West Bengal and Assam*, which keeps the
    sidereal months and is `hindu-solar-bengali` in `hc-calendars-indic`.
  - *The SGPC's calendar from 2010*, whose months begin with the Bikrami
    saṅkrāntis: that is `hindu-solar-vikrami` and, for the lunar
    observances, `hindu-lunar`, not a variant of this one.
  - *The 1999 calendar* the SGPC released for the tercentenary of the
    Khalsa, "close to" the 2003 one: no table of it was read.

## Accuracy

| Check | Test | Result |
| --- | --- | --- |
| The national days on their Bengali dates: 21 February on 8 Falgun, 26 March on 12 Choitro, 14 April on 1 Boishakh, 5 August on 21 Srabon, 16 December on 1 Poush | `bangladeshi::tests::the_national_days_fall_on_their_bengali_dates` | all |
| Every month opens on the revised table's date from 1426 | `every_month_begins_on_the_date_the_revised_table_gives` | all |
| Kartik 1426 began on 17 October 2019 | `the_revision_took_effect_in_1426` | as stated |
| Falgun and Phaggan hold 29 February | `falgun_holds_the_gregorian_leap_day`, `phaggan_holds_the_gregorian_leap_day` | all |
| The Nanakshahi year turns on 14 March and counts from 1469; every month opens on its table date | `the_year_turns_on_fourteen_march_and_counts_from_guru_nanaks_birth`, `every_month_begins_on_the_gregorian_date_of_the_table` | all |

Nothing here is computed from the sky, so there is no model to disagree
with a table; the open questions are the decrees' own. The Bangla Academy
and Cabinet Division notices of the 1987 adoption and of the revision, and
the SGPC's resolutions of 2003 and 2010, were not read, and the days on
which the calendars came into and out of force are the library's readings
of years the secondary sources give.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [wikipedia-bengali-calendars] | The era, the 1966 committee, the 1987 adoption, the 2018 plan and 16–17 October 2019, the month table in both forms | Yes, 2026-09-26 |
| [wikipedia-nanakshahi-calendar] | Purewal, the committee of 1995 and proposal of 1996, the launch on 14 April 2003, the months, the 2010 modification, the *Mool* name | Yes, 2026-09-26 |

The module documentation of `bangladeshi` also names the English Wikipedia
article "Bangladeshi national calendar" as its author read it on
2026-09-23; on 2026-09-26 that title redirects to "Bengali calendar",
which is the page cited here.

## Code

`crates/hc-calendars-solar/src/bangladeshi.rs` (`YEAR_OFFSET`,
`REVISED_FROM`, `ADOPTED`) and `nanakshahi.rs` (`YEAR_OFFSET`, `LAUNCHED`,
`LAST_KEPT`). Anchors: `the_national_days_fall_on_their_bengali_dates`,
`the_revision_took_effect_in_1426`,
`every_month_begins_on_the_gregorian_date_of_the_table`.
