# The Olympiads, ancient and modern

Backs the identifier `olympiad` and the functions `olympiad::ioc_olympiad`,
`olympiad::ioc_olympiad_years` and `olympiad::ioc_olympiad_before_2004` in
`hc-calendars-regional`.

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
the 31st of December of the fourth year" [olympedia-olympiad]. The Charter
in force from 11 September 2000 words the earlier rule as its Rule 10: "The
term "Olympiad" designates a period of four successive years. The Olympiad
begins with the opening of one edition of the Games of the Olympiad and
ends with the opening of the following edition"; "In the event of
non-celebration of the Games of an Olympiad, such Olympiad begins four
years after the start of the preceding Olympiad"; and "The Olympiads are
numbered consecutively from the first Olympic Games (Games of the Olympiad)
of modern times, celebrated in Athens in 1896" [ioc-charter-2000]. Under
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

**Modern, before 2004.** An Olympiad is a stretch of days, from one opening
to the next. The Charter's own rule on the ceremony is that "the Olympic
Games shall be proclaimed open by the Head of State of the host country" at
the Opening Ceremony, which "shall take place not earlier than one day
before the competitions" [ioc-charter-2000, Rule 69 and its bye-law], so an
opening is the opening ceremony, which Olympedia dates for every Games
[olympedia-editions]. Three Games need a word:

- **1900** had none: "true Opening and Closing Ceremonies were not held"
  [olympedia-editions]; "Official Opening of the Games by: No official
  opening" [olympics-com-games]. The II Olympiad begins with the Games
  themselves, on 14 May 1900.
- **1896** opened "on 25 March 1896, or on 6 April 1896, depending on
  whether one used the Julian Calendar ... or the more modern Gregorian"
  [olympedia-editions]: one day, 6 April Gregorian.
- **1956** had two: the equestrian Games of the XVI Olympiad at Stockholm,
  "Opening ceremony 10 June", and the Games at Melbourne on 22 November
  [olympedia-editions]; olympics.com calls it the one time "the unity of
  time and place, as stipulated in the Charter, has not been observed"
  [olympics-com-games]. No source read says which opened the Olympiad, so
  the days between are refused.

The Olympiads whose Games were not celebrated begin "four years after the
start of the preceding Olympiad": the rule does not say to the day, and
the library reads it as the same day of the month four years on, so the VI
began on 6 July 1916, four years after Stockholm's opening, and the XII and
XIII on 1 August 1940 and 1944.

**Worked example.** Which Olympiad is 1 June 1908? The III began with the
ceremony of 14 May 1904 at St. Louis, and the IV with London's of 13 July
1908, although London's competitions had begun on 27 April: 1 June 1908 is
in the III Olympiad, and by the 2004 definition in the IV, whose first year
1908 is.

## What is carried

| Identifier or item | Crate | What it is | Range |
| --- | --- | --- | --- |
| `olympiad` | `hc-calendars-regional` | The Julian calendar with the Olympiad and its year beside each date, the Olympic year taken as the Julian year it begins in, as the published code does | 1 January 776 BC onward |
| `olympiad::from_julian_year`, `olympiad::julian_year` | the same | `olympiad-from-julian-year` and `julian-year-from-olympiad`, in astronomical numbering | Olympiad 1 onward |
| `olympiad::ioc_olympiad`, `olympiad::ioc_olympiad_years` | the same | The modern Olympiad of a Gregorian year, and the years of an Olympiad, under the 2004 definition | 1896 onward |
| `olympiad::IOC_GAMES_NOT_CELEBRATED` | the same | The VI, XII and XIII | As recorded to the Games of the XXXIII Olympiad |
| `olympiad::ioc_olympiad_before_2004`, `olympiad::IOC_OPENINGS` | the same | The modern Olympiad of a day under Rule 10 before 2004, and the 28 openings it runs between; `None` from 10 June to 21 November 1956 | 6 April 1896 to 31 August 2004 |
| `olympiad::SUMMER_GAMES`, `olympiad::summer_games` | the same | Every Games of the Olympiad, with its number, year, host, whether it was held and its opening and closing ceremonies: the VI, XII and XIII not held, the XXXII of 2020 held in 2021, the XXXIV and XXXV scheduled for 2028 and 2032 | I (1896) to XXXV (2032) |
| `olympiad::WINTER_GAMES`, `olympiad::winter_games` | the same | Every Olympic Winter Games, with its own count: the Games of 1940 and 1944 not held and not numbered, V in 1948 after IV in 1936, every two years between the Summer Games from 1994; the XXVI and XXVII scheduled for 2030 and 2034 | I (1924) to XXVII (2034) |

**The fields.** A date's `year` is the astronomical Julian year and its
month and day are Julian; the extra fields `olympiad` and
`year-of-olympiad` carry the count. A caller may give either; if both are
given they must agree.

**Not carried.** The midsummer boundary: the Olympic year began with the
Games at a full moon after the solstice, which no source read dates year by
year, so a date between 1 January and the Games is given the Olympic year
that begins the following summer, by nobody's reckoning but the published
code's convention: 1 March 775 BC is year 2 of Olympiad 1, whose Games
were months away. Grumel dates the era's point of departure to "the
beginning of July 776 b.c." [grumel-eras-historical]; the calendar starts
it at 1 January 776 BC instead because it follows the published code,
whose Olympic year is the Julian year, and a July start would need the
same unmodelled boundary in every later year. The first half of 776 BC is
therefore Olympiad 1 here, and before the era by Grumel's reckoning. A
modern Olympiad as a calendar: it is a four-year label on the Gregorian
year, or a stretch of days between openings, and carries nothing a
function does not.

## Accuracy

The functions are exact integer arithmetic; what can be checked is the
conventions.

| Check | Test | Result |
| --- | --- | --- |
| The 33 sample dates of *Calendrical Calculations*, 586 BCE to 2094, as its published code computes them [reingold2018code, `dates.l`]: `olympiad`, cycle and year | `every_sample_date_agrees_or_is_refused_or_is_a_known_difference` (`crates/hyper-calendar/tests/rd_sample_dates.rs`) | 33 of 33 |
| Olympiad 1, year 1 is 776 BC; Jerome's 194.3 is 2 BC; Olympiad *N* < 195 begins in 780 − 4*N* BC | `the_first_olympiad_is_776_bc_and_jeromes_194_3_is_2_bc` | Holds for all 194 |
| The two published functions are inverses | `the_two_functions_are_inverses` | Holds, 776 BC to AD 3000 |
| The modern Olympiads: I in 1896, VI in 1916–1919, XI in 1936, XIV in 1948, XXXII for 2020 and 2021 | `the_modern_olympiads_count_from_1896_with_the_lost_games_numbered` | Holds |
| The calendar is the Julian calendar with the count beside it, and refuses before 776 BC and disagreeing fields | `the_calendar_is_the_julian_calendar_with_the_olympiad_beside_it`, `the_calendar_refuses_before_776_bc_and_disagreeing_fields` | Holds |
| Every Games of the Olympiad and Winter Games Olympedia lists, 1896–2032 and 1924–2034, with their ceremonies; the Summer numbers the Olympiads of their years, the Winter count without gaps, and Olympedia's openings of 1908–2004 the days Rule 10's Olympiads begin on but 1956's | `the_games_are_olympedias_editions` | Holds |
| Before 2004: the I from 6 April 1896, the II from 14 May 1900, the IV from 13 July 1908, the VI from 6 July 1916, the XIV from 29 July 1948, 1956 refused from 10 June to 21 November, the XXVIII from 13 August 2004 to 31 August 2004; every Games opened in its Olympiad's first year by the 2004 count | `the_olympiads_before_2004_run_from_opening_to_opening` | Holds |

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [reingold2018code] | `olympiad-start`, `olympiad-from-julian-year`, `julian-year-from-olympiad` | Yes, 2026-09-26 |
| [wikipedia-olympiad] | The formula 780 − 4*N* BC, the summer start, Jerome's example, Timaeus and the *Chronicon Paschale*; the modern count from 1 January 1896, the Games not celebrated, the 2020 Games | Yes, 2026-09-26 |
| [olympedia-olympiad] | The Charter's two definitions, before and after 1 September 2004 | Yes, 2026-09-26 |
| [ioc-charter-2000] | Rule 10, the Olympiad before 2004 and its Games not celebrated; Rule 69, the Opening Ceremony | Yes, 2026-09-29, the olympic.org HTML of the edition in force from 11 September 2000 in the Wayback Machine; the 1991, 1996 and 2003 editions are PDFs and were not read |
| [olympedia-editions] | The opening ceremony of every Games of 1896–2004, and the competition dates; 1896's two calendars, 1900 without ceremonies, Stockholm 1956; every Games of the Olympiad to 2032 and Winter Games to 2034, with their ceremonies | Yes, 2026-09-29, secondary |
| [olympics-com-games] | The IOC's Games pages: 1900's "No official opening", 1956's two cities, and the dates the ceremonies are checked against | Yes, 2026-09-29, in the Wayback Machine's copies of 2024, the live site refusing the connection; its start days are the first day of competition for 1900–1928 and are UTC days, a day early for Melbourne, Tokyo and Seoul |
| [grumel-eras-historical] | The Olympiads as a historical era, from "the beginning of July 776 b.c." | Yes, 2026-09-26; the epoch's wording re-read 2026-09-27 |

## Code

`crates/hc-calendars-regional/src/olympiad.rs`; `hc_olympic_games` writes
`SUMMER_GAMES` and `WINTER_GAMES` as lines, one a Games, with Olympedia's
host spelling, status and ceremony days, empty where it dates none
(`the_games_are_listed_as_olympedia_lists_them`, whose rows were read from
Olympedia's editions on 2026-10-03: Athina 1896, Berlin 1916 not held, Tokyo
2020 held in 2021, Paris 2024, and the Winter Games of 1924, 1952 and 2022).
Anchors:
`the_first_olympiad_is_776_bc_and_jeromes_194_3_is_2_bc`,
`the_two_functions_are_inverses`,
`the_modern_olympiads_count_from_1896_with_the_lost_games_numbered`,
`the_calendar_is_the_julian_calendar_with_the_olympiad_beside_it`,
`the_calendar_refuses_before_776_bc_and_disagreeing_fields`,
`the_olympiads_before_2004_run_from_opening_to_opening`,
`the_games_are_olympedias_editions`.
