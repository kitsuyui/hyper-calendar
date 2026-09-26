# The Olympiads, ancient and modern

Backs the identifier `olympiad` and the functions `olympiad::ioc_olympiad`
and `olympiad::ioc_olympiad_years` in `hc-calendars-regional`.

## What it is

**The ancient count.** The Greeks counted four-year periods from the Games
at Olympia. Coroebus's victory, "at a time equivalent to the summer of 776
BC", is year 1 of Olympiad 1; "each olympiad started with the holding of
the games, which originally began on the first or second full moon after
the summer solstice". Of those who dated by Olympiad, "the first to do so
consistently was Timaeus of Tauromenium in the third century BC",
Christian chroniclers
continued the system, and the *Chronicon Paschale* dates events down to
the 352nd Olympiad [wikipedia-olympiad]. Grumel calls "each period of four
years, at the end of which the games took place" an Olympiad
[grumel-eras-historical]. The count is a year count: historians wrote
"year 3 of Olympiad 194", and nobody dated a day by it.

**The modern count.** The International Olympic Committee numbers the
Olympiads of the modern Games from 1896, and the Games are those "of the
*N*th Olympiad" whether or not they were held. Before the Olympic Charter
of 1 September 2004, "an Olympiad began with the opening of one edition of
the Games of the Olympiad and ends with the opening of the following
edition"; since, "an Olympiad is a period of four consecutive calendar
years, beginning on the first of January of the first year and ending on
the 31st of December of the fourth year" [olympedia-olympiad]. Under
policy §10 the set is in scope because the IOC defines it.

## How it works

**Ancient.** "For N less than 195, Olympiad N is reckoned as having started
in the year 780 − (4 × N) BC" [wikipedia-olympiad]. Reingold and
Dershowitz's published code counts at the level of the year: `olympiad-start`
is `(bce 776)`, and `olympiad-from-julian-year` takes the years elapsed
since it — one fewer for a year AD, because the standard numbering skips
year 0 — to Olympiad 1 + ⌊*years*/4⌋ and year 1 + (*years* mod 4);
`julian-year-from-olympiad` is its inverse [reingold2018code]. In
astronomical year numbering the correction for year 0 disappears: *years*
is the astronomical Julian year plus 775.

**Worked example.** Jerome dates the birth of Christ "to year 3 of
Olympiad 194 … which equates to the year 2 BC" [wikipedia-olympiad].
Year 3 of Olympiad 194 is 4 × 193 + 2 = 774 years after 776 BC: 776 − 774
= 2 BC, astronomical −1. In the other direction, AD 1 is astronomical 1,
*years* = 776 = 4 × 194, so Olympiad 195, year 1; 1 BC, astronomical 0, is
year 4 of Olympiad 194.

**Modern.** "The first Olympiad started on 1 January 1896, and an Olympiad
starts on 1 January of the years evenly divisible by four", and the count
runs on through Games "not celebrated": the VI Olympiad of 1916–1919, the
XII of 1940–1943 and the XIII of 1944–1947; "the 1936 Games were those of
the XI Olympiad, while the next Summer Games were those of 1948, which were
the Games of the XIV Olympiad". The Games of the XXXII Olympiad were
"postponed to 2021 rather than cancelled" [wikipedia-olympiad]. So the
Olympiad of Gregorian year *Y* ≥ 1896 is 1 + ⌊(*Y* − 1896)/4⌋: 2021 is in
the XXXII.

## What is carried

| Identifier or item | Crate | What it is | Range |
| --- | --- | --- | --- |
| `olympiad` | `hc-calendars-regional` | The Julian calendar with the Olympiad and its year beside each date, the Olympic year taken as the Julian year it begins in, as the published code does | 1 January 776 BC onward |
| `olympiad::from_julian_year`, `olympiad::julian_year` | the same | `olympiad-from-julian-year` and `julian-year-from-olympiad`, in astronomical numbering | Olympiad 1 onward |
| `olympiad::ioc_olympiad`, `olympiad::ioc_olympiad_years` | the same | The modern Olympiad of a Gregorian year, and the years of an Olympiad, under the 2004 definition | 1896 onward |
| `olympiad::IOC_GAMES_NOT_CELEBRATED` | the same | The VI, XII and XIII | As recorded to the Games of the XXXIII Olympiad |

**The fields.** A date's `year` is the astronomical Julian year and its
month and day are Julian; the extra fields `olympiad` and
`year-of-olympiad` carry the count. A caller may give either; if both are
given they must agree.

**Not carried.** The midsummer boundary: the Olympic year began with the
Games at a full moon after the solstice, which no source read dates year by
year, so a date between 1 January and the Games is given the Olympic year
that began the summer before by nobody's reckoning but the published
code's convention. The modern definition before 2004, from one opening
ceremony to the next, which would need every opening day as data. A
modern Olympiad as a calendar: it is a four-year label on the Gregorian
year and carries nothing a function does not.

## Accuracy

The functions are exact integer arithmetic; what can be checked is the
conventions.

| Check | Test | Result |
| --- | --- | --- |
| Olympiad 1, year 1 is 776 BC; Jerome's 194.3 is 2 BC; Olympiad *N* < 195 begins in 780 − 4*N* BC | `the_first_olympiad_is_776_bc_and_jeromes_194_3_is_2_bc` | Holds for all 194 |
| The two published functions are inverses | `the_two_functions_are_inverses` | Holds, 776 BC to AD 3000 |
| The modern Olympiads: I in 1896, VI in 1916–1919, XI in 1936, XIV in 1948, XXXII for 2020 and 2021 | `the_modern_olympiads_count_from_1896_with_the_lost_games_numbered` | Holds |
| The calendar is the Julian calendar with the count beside it, and refuses before 776 BC and disagreeing fields | `the_calendar_is_the_julian_calendar_with_the_olympiad_beside_it`, `the_calendar_refuses_before_776_bc_and_disagreeing_fields` | Holds |

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [reingold2018code] | `olympiad-start`, `olympiad-from-julian-year`, `julian-year-from-olympiad` | Yes, 2026-09-26 |
| [wikipedia-olympiad] | The formula 780 − 4*N* BC, the summer start, Jerome's example, Timaeus and the *Chronicon Paschale*; the modern count from 1 January 1896, the Games not celebrated, the 2020 Games | Yes, 2026-09-26 |
| [olympedia-olympiad] | The Charter's two definitions, before and after 1 September 2004 | Yes, 2026-09-26; the Charter itself was not read |
| [grumel-eras-historical] | The Olympiads as a historical era | Yes, 2026-09-26 |

## Code

`crates/hc-calendars-regional/src/olympiad.rs`. Anchors:
`the_first_olympiad_is_776_bc_and_jeromes_194_3_is_2_bc`,
`the_two_functions_are_inverses`,
`the_modern_olympiads_count_from_1896_with_the_lost_games_numbered`,
`the_calendar_is_the_julian_calendar_with_the_olympiad_beside_it`,
`the_calendar_refuses_before_776_bc_and_disagreeing_fields`.
