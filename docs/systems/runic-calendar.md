# The runic calendar: the Swedish runestaff's day letters and golden numbers

Backs `hc_calendars_solar::cycles::runic` in `hc-calendars-solar`. There
is no calendar identifier: see *What is carried*.

## What it is

A runic calendar — *runstav* in Swedish, rune staff or runic almanac in
English — is a perpetual calendar of the Julian year carved on a stave of
wood, bone or horn, or on a booklet of small boards, and read with two
numbers that change from year to year: the year's Sunday letter and its
golden number [wikipedia-runic-calendar, wikipedia-sv-runstav]. Most of the
thousands that survive are Swedish and of the sixteenth and seventeenth
centuries; the oldest, from Nyköping, is of the thirteenth
[wikipedia-runic-calendar, wikipedia-sv-runstav]. The Gregorian reform of
1753 made them useless and they went out of use [wikipedia-sv-runstav].

A staff has three rows along its length [cucina2019, wikipedia-sv-runstav]:

1. **The day letters.** The first seven runes of the younger futhark,
   ᚠ ᚢ ᚦ ᚬ ᚱ ᚴ ᚼ, repeated round the year, one to a day: they "correspond
   to the roman letters ABCDEFG for the days of the week (Latin *litterae
   dominicales*, i.e. Sunday letters)" [cucina2019]. The year's Sunday
   letter says which of the seven marks its Sundays.
2. **The golden numbers.** The sixteen runes of the younger futhark stand
   for 1 to 16, "augmented by three extra, fabricated runes (ᛮ, ᛯ, and ᛰ,
   called respectively *árlaug*, *tvímaðr* and *belgþorn*)" for 17 to 19
   [cucina2019]. They stand against the days of the new moons: "the new
   moon would always fall on the day with the same number as the year of
   the cycle" [wikipedia-runic-calendar]. On the Swedish staffs the order
   of the futhark is the medieval one, in which the *l*- and *m*-runes had
   changed places, so 14 is ᛚ and 15 ᛘ [wikipedia-sv-runstav,
   wikipedia-runic-calendar].
3. **The feast marks.** Crosses and pictographs for the holidays of the
   Church calendar and of the farming year, which vary from staff to
   staff and from diocese to diocese — the Vatican booklet's "conform to
   the Åbo diocese" [cucina2019].

**Two traditions.** The Norwegian *primstav* is the same kind of object
with the emphasis the other way: its days are notches and its feasts
pictures, and it seldom carries golden numbers at all —
"Gyldentall forekommer sjelden på norske primstaver, men brukes i svenske
runekalendere" [wikipedia-no-primstav]. The runes, and so this
document, are the Swedish runestaff's.

**Two series of golden numbers.** The golden numbers of the Swedish staffs
"match with some days according to the old series (Latin *aureus numerus
antiquus*)" [cucina2019] — the medieval Church calendar's, from the
Alexandrian nineteen-year cycle. By the sixteenth century that cycle's new
moons had drifted three or four days from the sky, and the staffs made
after 1690 carry "corrected" golden numbers: a pre-1690 staff opens on
1 January with the þ-rune, 3, a later one with *belgþorn*, 19
[wikipedia-sv-runstav, citing Berg and Berg 1998, not read].

## How it works

**The day letters.** 1 January is ᚠ (A), 2 January ᚢ (B), and so on,
round and round: 7 January is ᚼ (G), 8 January ᚠ again. A common year has
365 lettered days; a leap day has no letter of its own, "consequently, in
leap years the dominical letter retrogrades by one place in the alphabet
at the date of intercalation, and there are two letters for the year, one
during January and February, the other during the remainder of the year"
[explanatory-supplement-1961, p. 421]. This is the convention
`cycles::julian_dominical_letter` already follows: 29 February has no
letter and 1 March is always ᚬ (D).

**The golden numbers.** The *Explanatory Supplement* tabulates the Julian
ecclesiastical lunar calendar, "the adopted 19-year cycle of dates for new
moon, with the golden number as the argument" [explanatory-supplement-1961,
table 14.4, p. 422]: nineteen rows of twelve months, each cell the day of
that month on which a lunation of that year begins, two where two do
(golden number 3 has new moons on 1 and 31 January), none where none does
(3 has none in February). Read by column instead of by row, the same table
is the golden-number row of the staff: 1 January carries 3, 3 January 11,
5 January 19, 6 January 8, 8 January 16 — each step eight golden numbers on,
one or two days later. The table has 235 entries, 12 for each golden number
and a thirteenth for seven of them, which is the 235 lunations of nineteen
Julian years and the "gyllentalscykeln som upprepas 12 gånger varpå läggs
till 7 gyllental, sammanlagt 235 tecken" of the Swedish account
[wikipedia-sv-runstav]. In leap years three February new moons fall a day
later — 26 to 27 February for 6, 28 to 29 for 14, 25 to 26 for 17 — "In
leap years use the date in parentheses" [explanatory-supplement-1961]; the
lunar calendar takes its extra day before the 25th.

**Worked example.** Easter 1513, which the *Explanatory Supplement* works
itself [explanatory-supplement-1961, example 14.1, p. 424]. 1513 is 13 in
the Metonic cycle (1513 = 19 × 79 + 12) and its Julian Sunday letter is B.
On the staff, the first day on or after 8 March carrying the golden number
13, ᛒ, is 11 March: the paschal new moon. Its fourteenth day, 24 March, is
the paschal full moon; 24 March carries the day letter ᚴ (F), and in a
year whose Sundays are ᚢ (B) that is a Thursday. The next day carrying ᚢ is
27 March, Easter Day — the date on which, the *Supplement* notes, Ponce de
León sighted Florida.

## What is carried

- **`cycles::runic`**, a reading and not a calendar. The staff has no
  date of its own to convert: its rows are read against the Julian date,
  and what they give is the day's letter and, where there is one, the
  golden number of the year whose new moon falls on it. That is the shape
  of the `cycles` module — the golden number, the Sunday letter and the
  epact are readings of a Julian year, not calendars — so the staff is a
  reading beside them, `runic::stave_day` for a fixed day and
  `runic::reading` for a Julian month and day in a common or a leap year,
  with `runic::is_new_moon` and `runic::paschal_new_moon` read off them.
  A calendar identifier would have had to invent a year for dates the
  staff does not have.
- **The runes** as Unicode characters: the seven day-letter runes and the
  nineteen golden-number runes as Wikipedia tabulates them, the last three
  the characters Unicode encodes as the "golden number runes"
  U+16EE–U+16F0 [wikipedia-runic-calendar, unicode-17-nameslist]. The
  forms cut on actual staffs vary with the carver; the characters are the
  standard's.
- **The old series only.** Table 14.4 is the *aureus numerus antiquus*,
  the series of the staffs before 1690 [cucina2019, wikipedia-sv-runstav].
- **Not carried:**
  - *The corrected golden numbers* of the staffs after 1690. The one
    statement read about them is that they begin with 19 on 1 January;
    no source read tabulates them.
  - *The feast marks.* They vary with the staff and the diocese, and no
    source read tabulates one staff's.
  - *The Norwegian primstav*, whose content is those marks.
  - *Pentadic numerals*, the later staffs' alternative to the golden
    runes [wikipedia-runic-calendar], for want of a table of their forms.
  - *The Gregorian staffs* cut after 1753 [wikipedia-sv-runstav].

## Accuracy

The reference is table 14.4 itself, transcribed cell by cell from the page
image, so the checks are that the transcription is the table and that the
table is the computus:

| Check | Test | Result |
| --- | --- | --- |
| Every golden number's new moons are 29 or 30 days apart, round the year and into the next | `every_lunation_is_twenty_nine_or_thirty_days` | 19 rows |
| 235 new moons in the table, 12 or 13 for each golden number, never two on one day | `the_table_has_235_new_moons_and_one_to_a_day` | all |
| Each golden number cut is eight on from the one before it, round the whole year | `a_day_carries_the_golden_number_eight_on_from_the_last` | 234 steps |
| 1 January carries 3, the þ-rune [wikipedia-sv-runstav]; January as the 1559 Book of Common Prayer's kalendar prints it [bcp1559] | `january_is_the_old_series` | 20 entries |
| The age of the moon on 22 March from the table is the Julian epact | `the_march_new_moon_gives_the_julian_epact` | 19 rows |
| Easter 1513 on 27 March, from new moon 11 March and full moon 24 March [explanatory-supplement-1961] | `easter_1513_read_off_the_stave` | exact |
| Easter read off the staff equals Delambre's rule in `hc-holiday`, which shares no code with it, for every year 326–1582 | `hyper-calendar`'s `easter_read_off_the_runestaff_is_the_julian_computus` | 1 257 years |
| The Vatican booklet's solar cycle begins at the leap year 1520 with the Sunday letters G and A, ᚼ and ᚠ [cucina2019]: A for January and February, G after | `the_vatican_booklet_starts_its_solar_cycle_at_1520` | exact |
| The days carrying a year's Sunday letter are its Sundays, and no others | `the_sundays_carry_the_years_letter` | 7 years |
| The leap year's three February moves and its letterless 29 February | `the_leap_year_moves_three_february_new_moons_a_day_later` | all |

The known difference from the sky is the Alexandrian cycle's own: its new
moons run late by a day in about three centuries, which is why the staffs
were corrected after 1690, and is the table's, not this module's.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [explanatory-supplement-1961] | Table 14.4, the Julian ecclesiastical new moons by golden number, and the leap-year dates; the letters and the intercalary day, p. 421; example 14.1, Easter 1513 | Yes, the archive.org scan, 2026-09-26 |
| [cucina2019] | The staff's three rows; the seven day runes as A–G; the sixteen runes and three extra ones as the golden numbers 1–19; the old series; the Vatican booklet's solar cycle from 1520 | Yes, 2026-09-26 |
| [wikipedia-runic-calendar] | The golden-number runes' table; the new moon on the day of the year's number; the Nyköping staff; pentadic numerals | Yes, 2026-09-26 |
| [wikipedia-sv-runstav] | The l- and m-rune order; 235 signs; the old and the corrected series, 3 and 19 on 1 January, 1690; the end in 1753 | Yes, 2026-09-26 |
| [wikipedia-no-primstav] | The Norwegian primstav seldom carries golden numbers | Yes, 2026-09-26 |
| [unicode-17-nameslist] | U+16EE–U+16F0 as golden numbers 17, 18 and 19 | Yes, 2026-09-26 |
| [bcp1559] | January's golden numbers in an English kalendar of the old series, as a check on the table | Yes, the transcription, 2026-09-26 |
| [worm1643] | Ole Worm's *Fasti Danici*, the classic printed account of the Danish runic calendar | Not read |
| [berg1998] | The corrected golden numbers after 1690, as cited by the Swedish Wikipedia | Not read |

Worm's *Fasti Danici* or a museum's transcription of a single staff would
let the golden-number row be checked against a staff rather than against
the computus the staffs follow; Berg and Berg, or a table of a
post-1690 staff, would let the corrected series be carried.

## Code

`crates/hc-calendars-solar/src/cycles/runic.rs`: the table is
`NEW_MOONS`, the runes `DAY_LETTER_RUNES` and `GOLDEN_NUMBER_RUNES`.
Anchors: `easter_1513_read_off_the_stave`, `january_is_the_old_series`,
and in `crates/hyper-calendar/tests/holiday_calendars.rs`,
`easter_read_off_the_runestaff_is_the_julian_computus`.
