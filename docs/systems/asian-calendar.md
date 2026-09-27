# The calendar of the Roman province of Asia

Backs the identifier `asian` in `hc-calendars-solar`.

## What it is

In 9 BC the koinon of the Greeks of Asia adopted a proposal of the
proconsul Paullus Fabius Maximus: the year would begin on Augustus's
birthday, and the months of the province would be fixed to the Julian
year. The decree was set up in the chief cities of the province. Copies
survive from Priene, Apamea, Eumeneia, Dorylaion, Metropolis and Maeonia
[bultrighini2021]. The text used here is Dittenberger's, OGIS 458, which
joins the Priene copy to the others [dittenberger-ogis2].

The months are the old Macedonian ones, with Dios renamed Kaisar. The
calendar lasted: a Pergamon inscription of 129–138 uses it, and a sermon
of 387 dates the Epiphany "according to the Asians" [bultrighini2021].
Cities also kept local eras, and at Kaunos and Smyrna the calendar of
Asia appears under local month names [bultrighini2021]. Neither is
carried.

## How it works

The decree fixes three things [dittenberger-ogis2, lines 50–77]:

* **The new year.** "The new month shall begin for all the cities on the
  ninth day before the Kalends of October, which is the birthday of
  Augustus": 23 September. The first month is Kaisar, "as has already been
  decreed".
* **The months.** Every month begins on the ninth day before the Kalends
  of a Roman month. The decree lists the months and their days: Kaisar
  31, Apellaios 30, Audnaios 31, Peritios 31, Dystros 28, Xandikos 31,
  Artemision 30, Daisios 31, Panemos 30, Loos 31, Gorpiaios 31,
  Hyperberetaios 30, "together 365". Bultrighini gives the same table
  [bultrighini2021].
* **The leap day.** Because of the intercalary day, Xandikos "shall have
  32 days", and the intercalary day "shall always be of the intercalary
  Kalends of the month Xandikos, two years coming between". Mommsen, in
  Dittenberger's note 49, reads this as the intercalated day standing
  before every other day of the month. Bultrighini puts it "after day 1"
  [bultrighini2021]. "Two years coming between" is a leap year every third
  year, the Roman practice of 9 BC, and the proconsul's letter asks for
  the Roman custom. Dittenberger's note 45 holds that Asia intercalated
  "in the same years as the Roman fasti", which the whole scheme leaves no
  doubt of. So once Rome kept the Julian leap years correctly, so did
  Asia.

Each month therefore begins on a fixed Julian day. Kaisar on 23 September,
Apellaios on 24 October, Audnaios on 23 November, Peritios on 24 December,
Dystros on 24 January, Xandikos on 21 February, Artemision on 24 March,
Daisios on 23 April, Panemos on 24 May, Loos on 23 June, Gorpiaios on 24
July, Hyperberetaios on 24 August. The ninth day before the Kalends of
March is 21 February in a leap year too, because the Roman leap day is a
second sixth day before those Kalends. So Xandikos takes the Julian leap
day and has 32 days.

**How the days are numbered.** A 31-day month opens with a day that is not
numbered, and then counts 1 to 30. The inscriptions call that day Sebaste:
"on the first Sebaste of the month Kaisar" at Lagina, "of the month Kaisar,
on Sebaste" at Kalymna (Dittenberger's note 36). The dated equations
require this count. The Metropolis *hemerologion* puts 7 October on day
14 and 5 December on day 12 [bultrighini2021]. Kaisar begins on 23
September, so 7 October is its fifteenth day, which is day 14 only if the
first day is unnumbered. In a leap year Xandikos has two days before its
day 1: Sebaste and the intercalary day. The sources read disagree on
their order, so this library calls them the first and second unnumbered
days.

**Worked example.** What Asian date is 1 March of AD 85? The Asian year
began on 23 September 84. Xandikos began on 21 February 85, the ninth day
before the Kalends of March. AD 85 is not a leap year, so Xandikos has one
unnumbered day: 21 February is Sebaste, 22 February is day 1, and 1 March
is day 8. The Sardis epitaph writes exactly this: "on the 8th of the
month Xandikos, on the Kalends of March" [bultrighini2021, no. 71]. In a
leap year, 21 and 22 February are the two unnumbered days and 23 February
is day 1. From the Kalends of March on, the numbers are the same in every
year.

## What is carried

| Identifier | Range | Year | Usage |
| --- | --- | --- | --- |
| `asian` | 23 September AD 4 to the end of year 9 999 | The Julian year, AD, in which the Asian year begins | Unrecorded |

`asian::AsianDate` stores the day's place in its month, counting the
unnumbered days, so that every day has one date. `AsianDate::written_day`
and `AsianDate::from_written` give the day as the calendar writes it:
unnumbered, or by its number.

**The year number is this library's.** The province did not count its
years by one era. The cities used their own eras, which Bultrighini and
Leschhorn discuss and which are not read here. A date carries the Julian
year in which its Asian year began and no era code, as a label for the
arithmetic and not a count anyone wrote.

**Why the range starts in AD 4.** The decree's own year is disputed:
Bultrighini argues for 8 BC and cites Buxton and Hannah for 5 BC
[bultrighini2021]; Wikipedia gives both readings [wikipedia-julian-calendar].
Rome was also not yet keeping the Julian leap years correctly, and Asia
followed Rome's years. Every reconstruction
of the Roman leap years in Wikipedia's table first puts a Roman date on
the day the proleptic Julian calendar gives it on 25 February AD 4 or
earlier [wikipedia-julian-calendar]. Before that, an Asian date's Julian
day depends on which reconstruction one follows. The first Asian year
wholly after that day begins on 23 September AD 4, and the calendar
starts there. The earlier years are refused rather than guessed.

**Not carried.** The local month names of Kaunos and Smyrna, and
Laodikeia's numbered months; the local eras; a count of days "waning" (ἀπιόντος), which
Acmonia writes and no source read defines; the order of Sebaste and the
intercalary day in a leap Xandikos.

## Accuracy

The arithmetic is exact: a Julian day under another name. The checks are
against the decree and the dated equations Bultrighini collects.

| Check | Test | Result |
| --- | --- | --- |
| The months total 365 days, and Xandikos has 32 in a leap year | `the_decree_lists_365_days_and_32_for_xandikos_in_a_leap_year` | Holds |
| Every month begins on the ninth day before the Kalends | `every_month_begins_on_the_ninth_day_before_the_kalends` | Holds for seven years, common and leap |
| The year begins on 23 September; Pergamon's penultimate day of Hyperberetaios is 21 September | `the_year_begins_on_augustus_birthday` | Holds |
| Metropolis, four days; Sardis; the Hemerologia's 1 January and 31 October; the Epiphany of 387 | `the_dated_equations_of_the_inscriptions_hold` | 8 of 8, in a common and a leap year |
| The unnumbered days | `a_thirty_one_day_month_opens_with_an_unnumbered_day`, `a_leap_xandikos_has_two_unnumbered_days_and_keeps_its_numbers` | Holds |
| Every day round-trips; the range is refused outside | `every_day_round_trips_and_the_range_is_refused_outside` | Holds |

**Known disagreements.** Two decrees from Acmonia are a day off
[bultrighini2021, nos. 74–75]. The one of AD 85 writes 5 March as 13
Xandikos, where this calendar has 12. Laffi and Thonemann, as Bultrighini
reports them, explain it by Acmonia counting a 31-day month "Sebaste, Day
2, Day 3". The one of AD 68 writes 8 April as 17 Artemision, where this
calendar has 16. Artemision has 30 days, so that explanation does not
cover it. The test `acmonia_disagrees_by_a_day_as_bultrighini_notes` pins
both differences. A funerary epigram from Amastris, which Bultrighini says
disagrees by seven weeks, is not in the province and is not checked.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [dittenberger-ogis2] | The decree's text, lines 50–77: the new year, Kaisar, the months and their days, the leap Xandikos, the start of each month; notes 36 and 49 | Yes, 2026-09-27, in the Internet Archive's text of the 1905 edition |
| [bultrighini2021] | The table of months, the copies of the decree, the disputed year, the dated equations nos. 71–81 | Yes, 2026-09-27 |
| [wikipedia-julian-calendar] | The Roman leap-year reconstructions and the day each aligns with the proleptic Julian calendar; the two readings of the decree's year | Yes, 2026-09-27 |

Laffi 1967, Samuel 1972, Sherk 1969, Thonemann 2015 and Blümel and
Merkelbach's edition of the Priene copy (I.Priene 14) are cited by
Bultrighini and were not read.

## Code

`crates/hc-calendars-solar/src/asian.rs`; the anchors are the tests named
above.
