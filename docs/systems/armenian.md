# The Armenian calendar: the Great Era's wandering year and Sarkawag's fixed year

Backs the identifiers `armenian` and `armenian-fixed` in
`hc-calendars-solar`.

## What it is

The calendar of the Armenian church and people: twelve months of thirty
days, Nawasard to Hroticʿ, and five epagomenal days, *aweleacʿ*
("additional"), with no leap day, so that the year is 365 days flat and
wanders through the seasons and the Julian year
[wikipedia-armenian-calendar]. Its years are counted in the Great Armenian
Era (Հայոց մեծ թվական), from 11 July 552 in the Julian calendar, the day
the Easter tables of Andreas of Byzantium, in use in Armenia since 352,
ran out [wikipedia-armenian-calendar]. The Council of Dvin examined the era
in 554 and it was adopted in 584 [hywiki-hayots-mets-tvakan]. It is still
used for cultural and religious purposes; the civil calendar of Armenia
has been the Gregorian since 1918 [wikipedia-armenian-calendar].

In 1084, when the first 532-year Easter cycle of the era ran out,
Yovhannēs Sarkawag, called Imastaser, "the philosopher", drew up a fixed
year: it always begins on 11 August and has 365¼ days, a sixth epagomenal
day every fourth year, and he counted it by subtracting 533 years rather
than 532 to absorb the year the wandering reckoning had gained. This
Lesser Armenian Era (Հայոց փոքր թվական) had limited use, mostly beside
the Great Era rather than in its place, and no church council confirmed
it [hywiki-hayots-poqr-tvakan; wikipedia-hovhannes-imastaser].

## How it works

**The Great Era.** Year *Y* begins 365 × (*Y* − 1) days after 11 July 552
Julian. The months are thirty days each and the thirteenth, *aweleacʿ*,
five. The calendar is therefore the Egyptian wandering year on another
epoch, and slips a day earlier in the Julian year every four years: over
1 461 Armenian years it loses a whole Julian year, and year 1462 began on
11 July 2012 Julian, 24 July 2012 Gregorian [wikipedia-armenian-calendar].

**The fixed year.** Year *Y*, numbered in the Great Era, begins on
11 August of Julian year *Y* + 551, and is 366 days long when the Julian
29 February falls inside it; the extra day is the sixth epagomenal day.

**Worked example.** Where does 1 Nawasard 533 fall in each? By the Great
Era, 532 × 365 = 194 180 days after 11 July 552. 532 Julian years hold
532 × 365 + 133 = 194 313 days, so 194 180 days is 133 days short of
11 July 1084: counting back 133 days from 11 July 1084 — 11 days of July,
30 of June, 31 of May, 30 of April, 31 of March — reaches 29 February 1084
(1084 is a Julian leap year). By the fixed year, 533 + 551 = 1084, so
1 Nawasard 533 is 11 August 1084, 164 days later. The two calendars share
the era and the months and disagree by five months in that year.

## What is carried

- **`armenian`**: the Great Era's wandering year, years 1 to 999 999, from
  RD 201 443, 11 July 552 Julian ([reingold2018code], `armenian-epoch`).
  In use from the epoch with no end, as the sources describe it.
- **`armenian-fixed`**: Sarkawag's fixed year, numbered in the Great Era,
  years 1 to 9 999; in use from 11 August 1084, proleptic before.
- The month names in Armenian script, declared with each calendar's shape;
  the era code `armenian`.
- **Not carried:**
  - *The Lesser Era's own year numbers* (1084 = year 1): the fixed year is
    numbered in the Great Era here, as Sarkawag's subtraction of 533 lets
    it be, so that the two calendars' years line up.
  - *The day names* of the thirty days of the month
    [wikipedia-armenian-calendar], which are a naming of the day and would
    be a cycle of their own.
  - *The 532-year Easter tables*, which are a computus.

## Accuracy

| Check | Test | Result |
| --- | --- | --- |
| The epoch is 11 July 552 Julian | `armenian::tests::the_epoch_is_the_eleventh_of_july_552` | as stated |
| The wandering year is the Egyptian one on another epoch | `only_the_epoch_separates_this_calendar_from_the_egyptian_one` | all |
| 1 Nawasard 533 is 29 February 1084 in the wandering year and 11 August 1084 in the fixed year, 164 days apart | `the_fixed_new_year_is_where_the_wandering_one_stood_in_four_twenty_eight` | as worked above |

Why Sarkawag chose 11 August no source read says. The arithmetic offers
one reading — 11 August is 31 days after the epoch's 11 July, 31 days is
124 years of drift, and 552 − 124 = 428 — and the test above states it as
arithmetic, not as a claim about his reasoning.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [wikipedia-armenian-calendar] | The 365-day year, the months, the epoch and the Easter tables of 352, the Sothic slip and 1462 in 2012, the Gregorian civil calendar from 1918 | Yes, 2026-09-26 |
| [hywiki-hayots-mets-tvakan] | The Great Era from 11 July 552, the Council of Dvin in 554, adoption in 584 | Yes, 2026-09-26 |
| [hywiki-hayots-poqr-tvakan] | Sarkawag's fixed year from 1084 and 11 August, 365¼ days, the 533 years, its limited use, no council's approval | Yes, 2026-09-26 |
| [wikipedia-hovhannes-imastaser] | 1084 and the added day | Yes, 2026-09-26 |
| [reingold2018code] | `armenian-epoch`, RD 201 443 | Yes, 2026-09-26 |
| [dulaurier1859] | The standard treatment of Armenian chronology | Not read |

Both Armenian Wikipedia articles rest on Julieta Eynatyan's article in the
encyclopaedia *Christian Armenia* and on the *Armenian Soviet
Encyclopedia*, vol. 6, pp. 200–201; English Wikipedia's epoch rests on
B. Tumanian, *History of Chronology* (1973). None was read. Dulaurier, or
Tumanian, is the source that would let Sarkawag's rule be cited from a
printed chronology.

## Code

`crates/hc-calendars-solar/src/armenian.rs` (`EPOCH`, `MONTHS`, `ERA`) and
`armenian_fixed.rs` (`REFORM_YEAR`, `JULIAN_OFFSET`, `NEW_YEAR_MONTH`,
`NEW_YEAR_DAY`, `REFORM`). Anchors:
`the_epoch_is_the_eleventh_of_july_552`,
`the_fixed_new_year_is_where_the_wandering_one_stood_in_four_twenty_eight`.
